// NOTE: This file was autogenerated by re_types_builder; DO NOT EDIT.
// Based on "crates/re_types/definitions/rerun/components/point2d.fbs"

#pragma once

#include "../data_cell.hpp"
#include "../datatypes/vec2d.hpp"

#include <arrow/type_fwd.h>
#include <cstdint>
#include <utility>

namespace rerun {
    namespace components {
        /// A point in 2D space.
        struct Point2D {
            rerun::datatypes::Vec2D xy;

            /// Name of the component, used for serialization.
            static const char* NAME;

          public:
            Point2D() = default;

            Point2D(rerun::datatypes::Vec2D _xy) : xy(std::move(_xy)) {}

            Point2D& operator=(rerun::datatypes::Vec2D _xy) {
                xy = std::move(_xy);
                return *this;
            }

            /// Returns the arrow data type this type corresponds to.
            static const std::shared_ptr<arrow::DataType>& to_arrow_datatype();

            /// Creates a new array builder with an array of this type.
            static arrow::Result<std::shared_ptr<arrow::FixedSizeListBuilder>>
                new_arrow_array_builder(arrow::MemoryPool* memory_pool);

            /// Fills an arrow array builder with an array of this type.
            static arrow::Status fill_arrow_array_builder(
                arrow::FixedSizeListBuilder* builder, const Point2D* elements, size_t num_elements
            );

            /// Creates a Rerun DataCell from an array of Point2D components.
            static arrow::Result<rerun::DataCell> to_data_cell(
                const Point2D* instances, size_t num_instances
            );
        };
    } // namespace components
} // namespace rerun
