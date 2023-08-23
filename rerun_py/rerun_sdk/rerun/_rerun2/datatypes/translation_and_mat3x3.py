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
from ._overrides import translationandmat3x3_init  # noqa: F401
from .. import datatypes
__all__ = ["TranslationAndMat3x3", "TranslationAndMat3x3Array", "TranslationAndMat3x3ArrayLike", "TranslationAndMat3x3Like", "TranslationAndMat3x3Type"]

def _translationandmat3x3_translation_converter(x: datatypes.Vec3DLike | None) -> datatypes.Vec3D | None:
    if x is None:
        return None
    elif isinstance(x, datatypes.Vec3D):
        return x
    else:
        return datatypes.Vec3D(x)


def _translationandmat3x3_matrix_converter(x: datatypes.Mat3x3Like | None) -> datatypes.Mat3x3 | None:
    if x is None:
        return None
    elif isinstance(x, datatypes.Mat3x3):
        return x
    else:
        return datatypes.Mat3x3(x)



@define(init=False)
class TranslationAndMat3x3:
    """
    Representation of an affine transform via a 3x3 affine matrix paired with a translation.

    First applies the matrix, then the translation.
    """

    def __init__(self, *args, **kwargs):  #type: ignore[no-untyped-def]
        translationandmat3x3_init(self, *args, **kwargs)

    from_parent: bool = field(converter=bool)
    """
    If true, the transform maps from the parent space to the space where the transform was logged.
    Otherwise, the transform maps from the space to its parent.
    """

    translation: datatypes.Vec3D | None = field(default=None, converter=_translationandmat3x3_translation_converter)
    """
    3D translation, applied after the matrix.
    """

    matrix: datatypes.Mat3x3 | None = field(default=None, converter=_translationandmat3x3_matrix_converter)
    """
    3x3 matrix for scale, rotation & shear.
    """



TranslationAndMat3x3Like = TranslationAndMat3x3
TranslationAndMat3x3ArrayLike = Union[
    TranslationAndMat3x3,
    Sequence[TranslationAndMat3x3Like],
    
]


# --- Arrow support ---

class TranslationAndMat3x3Type(BaseExtensionType):
    def __init__(self) -> None:
        pa.ExtensionType.__init__(
            self, pa.struct([pa.field("translation", pa.list_(pa.field("item", pa.float32(), nullable=False, metadata={}), 3), nullable=True, metadata={}), pa.field("matrix", pa.list_(pa.field("item", pa.float32(), nullable=False, metadata={}), 9), nullable=True, metadata={}), pa.field("from_parent", pa.bool_(), nullable=False, metadata={})]), "rerun.datatypes.TranslationAndMat3x3"
        )

class TranslationAndMat3x3Array(BaseExtensionArray[TranslationAndMat3x3ArrayLike]):
    _EXTENSION_NAME = "rerun.datatypes.TranslationAndMat3x3"
    _EXTENSION_TYPE = TranslationAndMat3x3Type

    @staticmethod
    def _native_to_pa_array(data: TranslationAndMat3x3ArrayLike, data_type: pa.DataType) -> pa.Array:
        raise NotImplementedError

TranslationAndMat3x3Type._ARRAY_TYPE = TranslationAndMat3x3Array

# TODO(cmc): bring back registration to pyarrow once legacy types are gone
# pa.register_extension_type(TranslationAndMat3x3Type())


