// NOTE: This file was autogenerated by re_types_builder; DO NOT EDIT.
// Based on "crates/re_types/definitions/rerun/datatypes/class_description_map_elem.fbs"

#pragma once

#include "../components/class_id.hpp"
#include "../datatypes/class_description.hpp"

#include <arrow/type_fwd.h>
#include <cstdint>

namespace rerun {
    namespace datatypes {
        /// A helper type for mapping class IDs to class descriptions.
        ///
        /// This is internal to the `AnnotationContext` structure.
        struct ClassDescriptionMapElem {
            rerun::components::ClassId class_id;

            rerun::datatypes::ClassDescription class_description;

          public:
            ClassDescriptionMapElem() = default;

            /// Returns the arrow data type this type corresponds to.
            static const std::shared_ptr<arrow::DataType>& to_arrow_datatype();

            /// Creates a new array builder with an array of this type.
            static arrow::Result<std::shared_ptr<arrow::StructBuilder>> new_arrow_array_builder(
                arrow::MemoryPool* memory_pool
            );

            /// Fills an arrow array builder with an array of this type.
            static arrow::Status fill_arrow_array_builder(
                arrow::StructBuilder* builder, const ClassDescriptionMapElem* elements,
                size_t num_elements
            );
        };
    } // namespace datatypes
} // namespace rerun
