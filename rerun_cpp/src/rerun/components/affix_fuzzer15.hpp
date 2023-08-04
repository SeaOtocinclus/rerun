// NOTE: This file was autogenerated by re_types_builder; DO NOT EDIT.
// Based on "crates/re_types/definitions/rerun/testing/components/fuzzy.fbs"

#pragma once

#include "../data_cell.hpp"
#include "../datatypes/affix_fuzzer3.hpp"

#include <arrow/type_fwd.h>
#include <cstdint>
#include <optional>
#include <utility>

namespace rerun {
    namespace components {
        struct AffixFuzzer15 {
            std::optional<rerun::datatypes::AffixFuzzer3> single_optional_union;

            /// Name of the component, used for serialization.
            static const char* NAME;

          public:
            AffixFuzzer15() = default;

            AffixFuzzer15(std::optional<rerun::datatypes::AffixFuzzer3> _single_optional_union)
                : single_optional_union(std::move(_single_optional_union)) {}

            AffixFuzzer15& operator=(
                std::optional<rerun::datatypes::AffixFuzzer3> _single_optional_union
            ) {
                single_optional_union = std::move(_single_optional_union);
                return *this;
            }

            /// Returns the arrow data type this type corresponds to.
            static const std::shared_ptr<arrow::DataType>& to_arrow_datatype();

            /// Creates a new array builder with an array of this type.
            static arrow::Result<std::shared_ptr<arrow::DenseUnionBuilder>> new_arrow_array_builder(
                arrow::MemoryPool* memory_pool
            );

            /// Fills an arrow array builder with an array of this type.
            static arrow::Status fill_arrow_array_builder(
                arrow::DenseUnionBuilder* builder, const AffixFuzzer15* elements,
                size_t num_elements
            );

            /// Creates a Rerun DataCell from an array of AffixFuzzer15 components.
            static arrow::Result<rerun::DataCell> to_data_cell(
                const AffixFuzzer15* instances, size_t num_instances
            );
        };
    } // namespace components
} // namespace rerun
