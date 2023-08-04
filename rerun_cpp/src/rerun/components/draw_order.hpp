// NOTE: This file was autogenerated by re_types_builder; DO NOT EDIT.
// Based on "crates/re_types/definitions/rerun/components/draw_order.fbs"

#pragma once

#include "../data_cell.hpp"

#include <arrow/type_fwd.h>
#include <cstdint>
#include <utility>

namespace rerun {
    namespace components {
        /// Draw order used for the display order of 2D elements.
        ///
        /// Higher values are drawn on top of lower values.
        /// An entity can have only a single draw order component.
        /// Within an entity draw order is governed by the order of the components.
        ///
        /// Draw order for entities with the same draw order is generally undefined.
        struct DrawOrder {
            float value;

            /// Name of the component, used for serialization.
            static const char* NAME;

          public:
            DrawOrder() = default;

            DrawOrder(float _value) : value(std::move(_value)) {}

            DrawOrder& operator=(float _value) {
                value = std::move(_value);
                return *this;
            }

            /// Returns the arrow data type this type corresponds to.
            static const std::shared_ptr<arrow::DataType>& to_arrow_datatype();

            /// Creates a new array builder with an array of this type.
            static arrow::Result<std::shared_ptr<arrow::FloatBuilder>> new_arrow_array_builder(
                arrow::MemoryPool* memory_pool
            );

            /// Fills an arrow array builder with an array of this type.
            static arrow::Status fill_arrow_array_builder(
                arrow::FloatBuilder* builder, const DrawOrder* elements, size_t num_elements
            );

            /// Creates a Rerun DataCell from an array of DrawOrder components.
            static arrow::Result<rerun::DataCell> to_data_cell(
                const DrawOrder* instances, size_t num_instances
            );
        };
    } // namespace components
} // namespace rerun
