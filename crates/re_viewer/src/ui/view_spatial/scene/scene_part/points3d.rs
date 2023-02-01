use std::sync::Arc;

use ahash::{HashMap, HashMapExt};
use glam::{Mat4, Vec3};
use itertools::Itertools as _;

use re_data_store::{EntityPath, EntityProperties};
use re_log_types::{
    component_types::{ClassId, ColorRGBA, InstanceKey, KeypointId, Label, Point3D, Radius},
    msg_bundle::Component,
};
use re_query::{query_primary_with_history, EntityView, QueryError};
use re_renderer::Size;

use crate::{
    misc::{
        InteractionHighlight, OptionalSpaceViewEntityHighlight, SpaceViewHighlights,
        TransformCache, ViewerContext,
    },
    ui::{
        annotations::ResolvedAnnotationInfo,
        scene::SceneQuery,
        view_spatial::{
            scene::{scene_part::instance_path_hash_for_picking, Keypoints},
            Label3D, SceneSpatial,
        },
        Annotations, DefaultColor,
    },
};

use super::ScenePart;

pub struct Points3DPart {
    /// If the number of points in the batch is > max_labels, don't render point labels.
    pub(crate) max_labels: usize,
}

impl Points3DPart {
    fn process_annotations(
        query: &SceneQuery<'_>,
        entity_view: &EntityView<Point3D>,
        annotations: &Arc<Annotations>,
        point_positions: &[Vec3],
    ) -> Result<(Vec<ResolvedAnnotationInfo>, Keypoints), QueryError> {
        crate::profile_function!();

        let mut keypoints: Keypoints = HashMap::new();

        let annotation_info = itertools::izip!(
            point_positions.iter(),
            entity_view.iter_component::<KeypointId>()?,
            entity_view.iter_component::<ClassId>()?,
        )
        .map(|(position, keypoint_id, class_id)| {
            let class_description = annotations.class_description(class_id);

            if let Some(keypoint_id) = keypoint_id {
                if let Some(class_id) = class_id {
                    keypoints
                        .entry((class_id, query.latest_at.as_i64()))
                        .or_insert_with(Default::default)
                        .insert(keypoint_id, *position);
                }
                class_description.annotation_info_with_keypoint(keypoint_id)
            } else {
                class_description.annotation_info()
            }
        })
        .collect();

        Ok((annotation_info, keypoints))
    }

    fn process_colors<'a>(
        entity_view: &'a EntityView<Point3D>,
        ent_path: &'a EntityPath,
        highlights: &'a [InteractionHighlight],
        annotation_infos: &'a [ResolvedAnnotationInfo],
    ) -> Result<impl Iterator<Item = egui::Color32> + 'a, QueryError> {
        crate::profile_function!();
        let default_color = DefaultColor::EntityPath(ent_path);

        let colors = itertools::izip!(
            highlights.iter(),
            annotation_infos.iter(),
            entity_view.iter_component::<ColorRGBA>()?,
        )
        .map(move |(highlight, annotation_info, color)| {
            SceneSpatial::apply_hover_and_selection_effect_color(
                annotation_info.color(color.map(move |c| c.to_array()).as_ref(), default_color),
                *highlight,
            )
        });
        Ok(colors)
    }

    fn process_radii<'a>(
        entity_view: &'a EntityView<Point3D>,
        highlights: &'a [InteractionHighlight],
    ) -> Result<impl Iterator<Item = Size> + 'a, QueryError> {
        crate::profile_function!();
        let radii = itertools::izip!(highlights.iter(), entity_view.iter_component::<Radius>()?)
            .map(move |(highlight, radius)| {
                SceneSpatial::apply_hover_and_selection_effect_size(
                    radius.map_or(Size::AUTO, |radius| Size::new_scene(radius.0)),
                    *highlight,
                )
            });
        Ok(radii)
    }

    fn process_labels<'a>(
        entity_view: &'a EntityView<Point3D>,
        annotation_infos: &'a [ResolvedAnnotationInfo],
        world_from_obj: Mat4,
    ) -> Result<impl Iterator<Item = Label3D> + 'a, QueryError> {
        crate::profile_function!();
        let labels = itertools::izip!(
            annotation_infos.iter(),
            entity_view.iter_primary()?,
            entity_view.iter_component::<Label>()?
        )
        .filter_map(move |(annotation_info, point, label)| {
            let label = annotation_info.label(label.map(|l| l.0).as_ref());
            match (point, label) {
                (Some(point), Some(label)) => Some(Label3D {
                    text: label,
                    origin: world_from_obj.transform_point3(point.into()),
                }),
                _ => None,
            }
        });
        Ok(labels)
    }

    #[allow(clippy::too_many_arguments)]
    fn process_entity_view(
        &self,
        scene: &mut SceneSpatial,
        query: &SceneQuery<'_>,
        properties: &EntityProperties,
        entity_view: &EntityView<Point3D>,
        ent_path: &EntityPath,
        world_from_obj: Mat4,
        entity_highlight: OptionalSpaceViewEntityHighlight<'_>,
    ) -> Result<(), QueryError> {
        crate::profile_function!();

        scene.num_logged_3d_objects += 1;

        let annotations = scene.annotation_map.find(ent_path);
        let show_labels = true;

        let ((point_positions, annotation_infos, keypoints), instance_path_hashes) =
            try_parallel2::<QueryError, _, _>(
                || {
                    let point_positions = {
                        crate::profile_scope!("collect_points");
                        entity_view
                            .iter_primary()?
                            .filter_map(|pt| pt.map(glam::Vec3::from))
                            .collect::<Vec<_>>()
                    };
                    let (annotation_infos, keypoints) = Self::process_annotations(
                        query,
                        entity_view,
                        &annotations,
                        point_positions.as_slice(),
                    )?;
                    Ok((point_positions, annotation_infos, keypoints))
                },
                || {
                    crate::profile_scope!("instance_hashes");
                    Ok(entity_view
                        .iter_instance_keys()?
                        .map(|instance_key| {
                            instance_path_hash_for_picking(
                                ent_path,
                                instance_key,
                                entity_view,
                                properties,
                                entity_highlight,
                            )
                        })
                        .collect::<Vec<_>>())
                },
            )?;

        let highlights = {
            crate::profile_scope!("highlights");
            instance_path_hashes
                .iter()
                .map(|hash| entity_highlight.index_highlight(hash.instance_key))
                .collect::<Vec<_>>()
        };

        let (colors, radii, labels) = try_parallel3(
            || {
                crate::profile_scope!("colors");
                Self::process_colors(entity_view, ent_path, &highlights, &annotation_infos)
                    .map(|it| it.collect_vec())
            },
            || {
                crate::profile_scope!("radii");
                Self::process_radii(entity_view, &highlights).map(|it| it.collect_vec())
            },
            || {
                crate::profile_scope!("labels");
                Self::process_labels(entity_view, &annotation_infos, world_from_obj)
                    .map(|it| it.collect_vec())
            },
        )?;

        if show_labels && instance_path_hashes.len() <= self.max_labels {
            scene.ui.labels_3d.extend(labels);
        }

        scene
            .primitives
            .points
            .batch("3d points")
            .world_from_obj(world_from_obj)
            .add_points(point_positions.into_iter())
            .colors(colors.into_iter())
            .radii(radii.into_iter())
            .user_data(instance_path_hashes.into_iter());

        scene.load_keypoint_connections(ent_path, keypoints, &annotations, properties.interactive);

        Ok(())
    }
}

impl ScenePart for Points3DPart {
    fn load(
        &self,
        scene: &mut SceneSpatial,
        ctx: &mut ViewerContext<'_>,
        query: &SceneQuery<'_>,
        transforms: &TransformCache,
        highlights: &SpaceViewHighlights,
    ) {
        crate::profile_scope!("Points3DPart");

        for (ent_path, props) in query.iter_entities() {
            let Some(world_from_obj) = transforms.reference_from_entity(ent_path) else {
                continue;
            };
            let entity_highlight = highlights.entity_highlight(ent_path.hash());

            match query_primary_with_history::<Point3D, 7>(
                &ctx.log_db.entity_db.arrow_store,
                &query.timeline,
                &query.latest_at,
                &props.visible_history,
                ent_path,
                [
                    Point3D::name(),
                    InstanceKey::name(),
                    ColorRGBA::name(),
                    Radius::name(),
                    Label::name(),
                    ClassId::name(),
                    KeypointId::name(),
                ],
            )
            .and_then(|entities| {
                for entity in entities {
                    self.process_entity_view(
                        scene,
                        query,
                        &props,
                        &entity,
                        ent_path,
                        world_from_obj,
                        entity_highlight,
                    )?;
                }
                Ok(())
            }) {
                Ok(_) | Err(QueryError::PrimaryNotFound) => {}
                Err(err) => {
                    re_log::error_once!("Unexpected error querying {ent_path:?}: {err}");
                }
            }
        }
    }
}

// ----

#[cfg(not(target_arch = "wasm32"))]
fn parallel2<A: Send, B: Send>(
    a: impl Send + FnOnce() -> A,
    b: impl Send + FnOnce() -> B,
) -> (A, B) {
    rayon::join(a, b)
}

#[cfg(target_arch = "wasm32")]
fn parallel2<A, B>(a: impl FnOnce() -> A, b: impl FnOnce() -> B) -> (A, B) {
    ((a)(), (b)())
}

fn parallel3<A: Send, B: Send, C: Send>(
    a: impl Send + FnOnce() -> A,
    b: impl Send + FnOnce() -> B,
    c: impl Send + FnOnce() -> C,
) -> (A, B, C) {
    let ((a, b), c) = parallel2(|| parallel2(a, b), c);
    (a, b, c)
}

fn try_parallel2<Error: Send, A: Send, B: Send>(
    a: impl Send + FnOnce() -> Result<A, Error>,
    b: impl Send + FnOnce() -> Result<B, Error>,
) -> Result<(A, B), Error> {
    let (a, b) = parallel2(a, b);
    Ok((a?, b?))
}

fn try_parallel3<Error: Send, A: Send, B: Send, C: Send>(
    a: impl Send + FnOnce() -> Result<A, Error>,
    b: impl Send + FnOnce() -> Result<B, Error>,
    c: impl Send + FnOnce() -> Result<C, Error>,
) -> Result<(A, B, C), Error> {
    let ((a, b), c) = parallel2(|| parallel2(a, b), c);
    Ok((a?, b?, c?))
}
