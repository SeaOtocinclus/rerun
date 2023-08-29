// NOTE: This file was autogenerated by re_types_builder; DO NOT EDIT.

#![allow(trivial_numeric_casts)]
#![allow(unused_parens)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::iter_on_single_items)]
#![allow(clippy::map_flatten)]
#![allow(clippy::match_wildcard_for_single_variants)]
#![allow(clippy::needless_question_mark)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::unnecessary_cast)]

/// A batch of 3D arrows with optional colors, radii, labels, etc.
///
/// ## Example
///
/// ```ignore
/// //! Log a batch of 3D arrows.
///
/// use std::f32::consts::TAU;
///
/// use rerun::{
///    archetypes::Arrows3D,
///    components::{Color, Vector3D},
///    MsgSender, RecordingStreamBuilder,
/// };
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///    let (rec_stream, storage) = RecordingStreamBuilder::new(env!("CARGO_BIN_NAME")).memory()?;
///
///    let (vectors, colors): (Vec<_>, Vec<_>) = (0..100)
///        .map(|i| {
///            let angle = TAU * i as f32 * 0.01;
///            let length = ((i + 1) as f32).log2();
///            let c = (angle / TAU * 255.0) as u8;
///            (
///                Vector3D::from([length * angle.sin(), 0.0, length * angle.cos()]),
///                Color::from_unmultiplied_rgba(255 - c, c, 128, 128),
///            )
///        })
///        .unzip();
///
///    MsgSender::from_archetype("arrows", &Arrows3D::new(vectors).with_colors(colors))?
///        .send(&rec_stream)?;
///
///    rerun::native_viewer::show(storage.take())?;
///    Ok(())
/// }
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct Arrows3D {
    /// All the vectors for each arrow in the batch.
    pub vectors: Vec<crate::components::Vector3D>,

    /// All the origin points for each arrow in the batch.
    pub origins: Option<Vec<crate::components::Origin3D>>,

    /// Optional radii for the arrows.
    ///
    /// The shaft is rendered as a line with `radius = 0.5 * radius`.
    /// The tip is rendered with `height = 2.0 * radius` and `radius = 1.0 * radius`.
    pub radii: Option<Vec<crate::components::Radius>>,

    /// Optional colors for the points.
    pub colors: Option<Vec<crate::components::Color>>,

    /// Optional text labels for the arrows.
    pub labels: Option<Vec<crate::components::Label>>,

    /// Optional class Ids for the points.
    ///
    /// The class ID provides colors and labels if not specified explicitly.
    pub class_ids: Option<Vec<crate::components::ClassId>>,

    /// Unique identifiers for each individual point in the batch.
    pub instance_keys: Option<Vec<crate::components::InstanceKey>>,
}

static REQUIRED_COMPONENTS: once_cell::sync::Lazy<[crate::ComponentName; 1usize]> =
    once_cell::sync::Lazy::new(|| ["rerun.components.Vector3D".into()]);

static RECOMMENDED_COMPONENTS: once_cell::sync::Lazy<[crate::ComponentName; 1usize]> =
    once_cell::sync::Lazy::new(|| ["rerun.components.Origin3D".into()]);

static OPTIONAL_COMPONENTS: once_cell::sync::Lazy<[crate::ComponentName; 5usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            "rerun.radius".into(),
            "rerun.colorrgba".into(),
            "rerun.label".into(),
            "rerun.components.ClassId".into(),
            "rerun.instance_key".into(),
        ]
    });

static ALL_COMPONENTS: once_cell::sync::Lazy<[crate::ComponentName; 7usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            "rerun.components.Vector3D".into(),
            "rerun.components.Origin3D".into(),
            "rerun.radius".into(),
            "rerun.colorrgba".into(),
            "rerun.label".into(),
            "rerun.components.ClassId".into(),
            "rerun.instance_key".into(),
        ]
    });

impl Arrows3D {
    pub const NUM_COMPONENTS: usize = 7usize;
}

impl crate::Archetype for Arrows3D {
    #[inline]
    fn name() -> crate::ArchetypeName {
        crate::ArchetypeName::Borrowed("rerun.archetypes.Arrows3D")
    }

    #[inline]
    fn required_components() -> &'static [crate::ComponentName] {
        REQUIRED_COMPONENTS.as_slice()
    }

    #[inline]
    fn recommended_components() -> &'static [crate::ComponentName] {
        RECOMMENDED_COMPONENTS.as_slice()
    }

    #[inline]
    fn optional_components() -> &'static [crate::ComponentName] {
        OPTIONAL_COMPONENTS.as_slice()
    }

    #[inline]
    fn all_components() -> &'static [crate::ComponentName] {
        ALL_COMPONENTS.as_slice()
    }

    #[inline]
    fn indicator_component() -> crate::ComponentName {
        "rerun.components.Arrows3DIndicator".into()
    }

    #[inline]
    fn num_instances(&self) -> usize {
        self.vectors.len()
    }

    #[inline]
    fn try_to_arrow(
        &self,
    ) -> crate::SerializationResult<
        Vec<(::arrow2::datatypes::Field, Box<dyn ::arrow2::array::Array>)>,
    > {
        use crate::{Loggable as _, ResultExt as _};
        Ok([
            {
                Some({
                    let array =
                        <crate::components::Vector3D>::try_to_arrow(self.vectors.iter(), None);
                    array.map(|array| {
                        let datatype = ::arrow2::datatypes::DataType::Extension(
                            "rerun.components.Vector3D".into(),
                            Box::new(array.data_type().clone()),
                            Some("rerun.components.Vector3D".into()),
                        );
                        (
                            ::arrow2::datatypes::Field::new("vectors", datatype, false),
                            array,
                        )
                    })
                })
                .transpose()
                .with_context("rerun.archetypes.Arrows3D#vectors")?
            },
            {
                self.origins
                    .as_ref()
                    .map(|many| {
                        let array = <crate::components::Origin3D>::try_to_arrow(many.iter(), None);
                        array.map(|array| {
                            let datatype = ::arrow2::datatypes::DataType::Extension(
                                "rerun.components.Origin3D".into(),
                                Box::new(array.data_type().clone()),
                                Some("rerun.components.Origin3D".into()),
                            );
                            (
                                ::arrow2::datatypes::Field::new("origins", datatype, false),
                                array,
                            )
                        })
                    })
                    .transpose()
                    .with_context("rerun.archetypes.Arrows3D#origins")?
            },
            {
                self.radii
                    .as_ref()
                    .map(|many| {
                        let array = <crate::components::Radius>::try_to_arrow(many.iter(), None);
                        array.map(|array| {
                            let datatype = ::arrow2::datatypes::DataType::Extension(
                                "rerun.components.Radius".into(),
                                Box::new(array.data_type().clone()),
                                Some("rerun.radius".into()),
                            );
                            (
                                ::arrow2::datatypes::Field::new("radii", datatype, false),
                                array,
                            )
                        })
                    })
                    .transpose()
                    .with_context("rerun.archetypes.Arrows3D#radii")?
            },
            {
                self.colors
                    .as_ref()
                    .map(|many| {
                        let array = <crate::components::Color>::try_to_arrow(many.iter(), None);
                        array.map(|array| {
                            let datatype = ::arrow2::datatypes::DataType::Extension(
                                "rerun.components.Color".into(),
                                Box::new(array.data_type().clone()),
                                Some("rerun.colorrgba".into()),
                            );
                            (
                                ::arrow2::datatypes::Field::new("colors", datatype, false),
                                array,
                            )
                        })
                    })
                    .transpose()
                    .with_context("rerun.archetypes.Arrows3D#colors")?
            },
            {
                self.labels
                    .as_ref()
                    .map(|many| {
                        let array = <crate::components::Label>::try_to_arrow(many.iter(), None);
                        array.map(|array| {
                            let datatype = ::arrow2::datatypes::DataType::Extension(
                                "rerun.components.Label".into(),
                                Box::new(array.data_type().clone()),
                                Some("rerun.label".into()),
                            );
                            (
                                ::arrow2::datatypes::Field::new("labels", datatype, false),
                                array,
                            )
                        })
                    })
                    .transpose()
                    .with_context("rerun.archetypes.Arrows3D#labels")?
            },
            {
                self.class_ids
                    .as_ref()
                    .map(|many| {
                        let array = <crate::components::ClassId>::try_to_arrow(many.iter(), None);
                        array.map(|array| {
                            let datatype = ::arrow2::datatypes::DataType::Extension(
                                "rerun.components.ClassId".into(),
                                Box::new(array.data_type().clone()),
                                Some("rerun.components.ClassId".into()),
                            );
                            (
                                ::arrow2::datatypes::Field::new("class_ids", datatype, false),
                                array,
                            )
                        })
                    })
                    .transpose()
                    .with_context("rerun.archetypes.Arrows3D#class_ids")?
            },
            {
                self.instance_keys
                    .as_ref()
                    .map(|many| {
                        let array =
                            <crate::components::InstanceKey>::try_to_arrow(many.iter(), None);
                        array.map(|array| {
                            let datatype = ::arrow2::datatypes::DataType::Extension(
                                "rerun.components.InstanceKey".into(),
                                Box::new(array.data_type().clone()),
                                Some("rerun.instance_key".into()),
                            );
                            (
                                ::arrow2::datatypes::Field::new("instance_keys", datatype, false),
                                array,
                            )
                        })
                    })
                    .transpose()
                    .with_context("rerun.archetypes.Arrows3D#instance_keys")?
            },
            {
                let datatype = ::arrow2::datatypes::DataType::Extension(
                    "rerun.components.Arrows3DIndicator".to_owned(),
                    Box::new(::arrow2::datatypes::DataType::Null),
                    Some("rerun.components.Arrows3DIndicator".to_owned()),
                );
                let array = ::arrow2::array::NullArray::new(
                    datatype.to_logical_type().clone(),
                    self.num_instances(),
                )
                .boxed();
                Some((
                    ::arrow2::datatypes::Field::new(
                        "rerun.components.Arrows3DIndicator",
                        datatype,
                        false,
                    ),
                    array,
                ))
            },
        ]
        .into_iter()
        .flatten()
        .collect())
    }

    #[inline]
    fn try_from_arrow(
        data: impl IntoIterator<Item = (::arrow2::datatypes::Field, Box<dyn ::arrow2::array::Array>)>,
    ) -> crate::DeserializationResult<Self> {
        use crate::{Loggable as _, ResultExt as _};
        let arrays_by_name: ::std::collections::HashMap<_, _> = data
            .into_iter()
            .map(|(field, array)| (field.name, array))
            .collect();
        let vectors = {
            let array = arrays_by_name
                .get("vectors")
                .ok_or_else(crate::DeserializationError::missing_data)
                .with_context("rerun.archetypes.Arrows3D#vectors")?;
            <crate::components::Vector3D>::try_from_arrow_opt(&**array)
                .with_context("rerun.archetypes.Arrows3D#vectors")?
                .into_iter()
                .map(|v| v.ok_or_else(crate::DeserializationError::missing_data))
                .collect::<crate::DeserializationResult<Vec<_>>>()
                .with_context("rerun.archetypes.Arrows3D#vectors")?
        };
        let origins = if let Some(array) = arrays_by_name.get("origins") {
            Some({
                <crate::components::Origin3D>::try_from_arrow_opt(&**array)
                    .with_context("rerun.archetypes.Arrows3D#origins")?
                    .into_iter()
                    .map(|v| v.ok_or_else(crate::DeserializationError::missing_data))
                    .collect::<crate::DeserializationResult<Vec<_>>>()
                    .with_context("rerun.archetypes.Arrows3D#origins")?
            })
        } else {
            None
        };
        let radii = if let Some(array) = arrays_by_name.get("radii") {
            Some({
                <crate::components::Radius>::try_from_arrow_opt(&**array)
                    .with_context("rerun.archetypes.Arrows3D#radii")?
                    .into_iter()
                    .map(|v| v.ok_or_else(crate::DeserializationError::missing_data))
                    .collect::<crate::DeserializationResult<Vec<_>>>()
                    .with_context("rerun.archetypes.Arrows3D#radii")?
            })
        } else {
            None
        };
        let colors = if let Some(array) = arrays_by_name.get("colors") {
            Some({
                <crate::components::Color>::try_from_arrow_opt(&**array)
                    .with_context("rerun.archetypes.Arrows3D#colors")?
                    .into_iter()
                    .map(|v| v.ok_or_else(crate::DeserializationError::missing_data))
                    .collect::<crate::DeserializationResult<Vec<_>>>()
                    .with_context("rerun.archetypes.Arrows3D#colors")?
            })
        } else {
            None
        };
        let labels = if let Some(array) = arrays_by_name.get("labels") {
            Some({
                <crate::components::Label>::try_from_arrow_opt(&**array)
                    .with_context("rerun.archetypes.Arrows3D#labels")?
                    .into_iter()
                    .map(|v| v.ok_or_else(crate::DeserializationError::missing_data))
                    .collect::<crate::DeserializationResult<Vec<_>>>()
                    .with_context("rerun.archetypes.Arrows3D#labels")?
            })
        } else {
            None
        };
        let class_ids = if let Some(array) = arrays_by_name.get("class_ids") {
            Some({
                <crate::components::ClassId>::try_from_arrow_opt(&**array)
                    .with_context("rerun.archetypes.Arrows3D#class_ids")?
                    .into_iter()
                    .map(|v| v.ok_or_else(crate::DeserializationError::missing_data))
                    .collect::<crate::DeserializationResult<Vec<_>>>()
                    .with_context("rerun.archetypes.Arrows3D#class_ids")?
            })
        } else {
            None
        };
        let instance_keys = if let Some(array) = arrays_by_name.get("instance_keys") {
            Some({
                <crate::components::InstanceKey>::try_from_arrow_opt(&**array)
                    .with_context("rerun.archetypes.Arrows3D#instance_keys")?
                    .into_iter()
                    .map(|v| v.ok_or_else(crate::DeserializationError::missing_data))
                    .collect::<crate::DeserializationResult<Vec<_>>>()
                    .with_context("rerun.archetypes.Arrows3D#instance_keys")?
            })
        } else {
            None
        };
        Ok(Self {
            vectors,
            origins,
            radii,
            colors,
            labels,
            class_ids,
            instance_keys,
        })
    }
}

impl Arrows3D {
    pub fn new(vectors: impl IntoIterator<Item = impl Into<crate::components::Vector3D>>) -> Self {
        Self {
            vectors: vectors.into_iter().map(Into::into).collect(),
            origins: None,
            radii: None,
            colors: None,
            labels: None,
            class_ids: None,
            instance_keys: None,
        }
    }

    pub fn with_origins(
        mut self,
        origins: impl IntoIterator<Item = impl Into<crate::components::Origin3D>>,
    ) -> Self {
        self.origins = Some(origins.into_iter().map(Into::into).collect());
        self
    }

    pub fn with_radii(
        mut self,
        radii: impl IntoIterator<Item = impl Into<crate::components::Radius>>,
    ) -> Self {
        self.radii = Some(radii.into_iter().map(Into::into).collect());
        self
    }

    pub fn with_colors(
        mut self,
        colors: impl IntoIterator<Item = impl Into<crate::components::Color>>,
    ) -> Self {
        self.colors = Some(colors.into_iter().map(Into::into).collect());
        self
    }

    pub fn with_labels(
        mut self,
        labels: impl IntoIterator<Item = impl Into<crate::components::Label>>,
    ) -> Self {
        self.labels = Some(labels.into_iter().map(Into::into).collect());
        self
    }

    pub fn with_class_ids(
        mut self,
        class_ids: impl IntoIterator<Item = impl Into<crate::components::ClassId>>,
    ) -> Self {
        self.class_ids = Some(class_ids.into_iter().map(Into::into).collect());
        self
    }

    pub fn with_instance_keys(
        mut self,
        instance_keys: impl IntoIterator<Item = impl Into<crate::components::InstanceKey>>,
    ) -> Self {
        self.instance_keys = Some(instance_keys.into_iter().map(Into::into).collect());
        self
    }
}
