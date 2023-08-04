// NOTE: This file was autogenerated by re_types_builder; DO NOT EDIT.
// Based on "crates/re_types/definitions/rerun/testing/components/fuzzy_deps.fbs"

#pragma once

#include "../data_cell.hpp"

#include <arrow/type_fwd.h>
#include <cstdint>
#include <string>
#include <utility>

namespace rerun {
    namespace components {
        struct StringComponent {
            std::string value;

            /// Name of the component, used for serialization.
            static const char* NAME;

          public:
            StringComponent() = default;

            StringComponent(std::string _value) : value(std::move(_value)) {}

            StringComponent& operator=(std::string _value) {
                value = std::move(_value);
                return *this;
            }

            /// Returns the arrow data type this type corresponds to.
            static const std::shared_ptr<arrow::DataType>& to_arrow_datatype();

            /// Creates a new array builder with an array of this type.
            static arrow::Result<std::shared_ptr<arrow::StringBuilder>> new_arrow_array_builder(
                arrow::MemoryPool* memory_pool
            );

            /// Fills an arrow array builder with an array of this type.
            static arrow::Status fill_arrow_array_builder(
                arrow::StringBuilder* builder, const StringComponent* elements, size_t num_elements
            );

            /// Creates a Rerun DataCell from an array of StringComponent components.
            static arrow::Result<rerun::DataCell> to_data_cell(
                const StringComponent* instances, size_t num_instances
            );
        };
    } // namespace components
} // namespace rerun
