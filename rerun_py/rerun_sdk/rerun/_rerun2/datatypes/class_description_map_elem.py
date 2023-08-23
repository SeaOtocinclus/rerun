# NOTE: This file was autogenerated by re_types_builder; DO NOT EDIT.

from __future__ import annotations

from typing import (Any, Dict, Iterable, Optional, Sequence, Set, Tuple, Union,
    TYPE_CHECKING, SupportsFloat, Literal)

from attrs import define, field
import numpy as np
import numpy.typing as npt
import pyarrow as pa

from .._baseclasses import (
    Archetype,
    BaseExtensionType,
    BaseExtensionArray,
    BaseDelegatingExtensionType,
    BaseDelegatingExtensionArray
)
from .._converters import (
    int_or_none,
    float_or_none,
    bool_or_none,
    str_or_none,
    to_np_uint8,
    to_np_uint16,
    to_np_uint32,
    to_np_uint64,
    to_np_int8,
    to_np_int16,
    to_np_int32,
    to_np_int64,
    to_np_bool,
    to_np_float16,
    to_np_float32,
    to_np_float64
)
from ._overrides import classdescriptionmapelem_native_to_pa_array  # noqa: F401
from .. import datatypes
__all__ = ["ClassDescriptionMapElem", "ClassDescriptionMapElemArray", "ClassDescriptionMapElemArrayLike", "ClassDescriptionMapElemLike", "ClassDescriptionMapElemType"]

def _classdescriptionmapelem_class_id_converter(x: datatypes.ClassIdLike) -> datatypes.ClassId:
    if isinstance(x, datatypes.ClassId):
        return x
    else:
        return datatypes.ClassId(x)


@define
class ClassDescriptionMapElem:
    """
    A helper type for mapping class IDs to class descriptions.

    This is internal to the `AnnotationContext` structure.
    """

    class_id: datatypes.ClassId = field(converter=_classdescriptionmapelem_class_id_converter)
    class_description: datatypes.ClassDescription = field()


if TYPE_CHECKING:
    ClassDescriptionMapElemLike = Union[
        ClassDescriptionMapElem,
        datatypes.ClassDescriptionLike
    ]
else:
    ClassDescriptionMapElemLike = Any

ClassDescriptionMapElemArrayLike = Union[
    ClassDescriptionMapElem,
    Sequence[ClassDescriptionMapElemLike],
    
]


# --- Arrow support ---

class ClassDescriptionMapElemType(BaseExtensionType):
    def __init__(self) -> None:
        pa.ExtensionType.__init__(
            self, pa.struct([pa.field("class_id", pa.uint16(), nullable=False, metadata={}), pa.field("class_description", pa.struct([pa.field("info", pa.struct([pa.field("id", pa.uint16(), nullable=False, metadata={}), pa.field("label", pa.utf8(), nullable=True, metadata={}), pa.field("color", pa.uint32(), nullable=True, metadata={})]), nullable=False, metadata={}), pa.field("keypoint_annotations", pa.list_(pa.field("item", pa.struct([pa.field("id", pa.uint16(), nullable=False, metadata={}), pa.field("label", pa.utf8(), nullable=True, metadata={}), pa.field("color", pa.uint32(), nullable=True, metadata={})]), nullable=False, metadata={})), nullable=False, metadata={}), pa.field("keypoint_connections", pa.list_(pa.field("item", pa.struct([pa.field("keypoint0", pa.uint16(), nullable=False, metadata={}), pa.field("keypoint1", pa.uint16(), nullable=False, metadata={})]), nullable=False, metadata={})), nullable=False, metadata={})]), nullable=False, metadata={})]), "rerun.datatypes.ClassDescriptionMapElem"
        )

class ClassDescriptionMapElemArray(BaseExtensionArray[ClassDescriptionMapElemArrayLike]):
    _EXTENSION_NAME = "rerun.datatypes.ClassDescriptionMapElem"
    _EXTENSION_TYPE = ClassDescriptionMapElemType

    @staticmethod
    def _native_to_pa_array(data: ClassDescriptionMapElemArrayLike, data_type: pa.DataType) -> pa.Array:
        return classdescriptionmapelem_native_to_pa_array(data, data_type)

ClassDescriptionMapElemType._ARRAY_TYPE = ClassDescriptionMapElemArray

# TODO(cmc): bring back registration to pyarrow once legacy types are gone
# pa.register_extension_type(ClassDescriptionMapElemType())


