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
from ._overrides import angle_init  # noqa: F401
__all__ = ["Angle", "AngleArray", "AngleArrayLike", "AngleLike", "AngleType"]


@define(init=False)
class Angle:
    """
    Angle in either radians or degrees.
    """

    def __init__(self, *args, **kwargs):  #type: ignore[no-untyped-def]
        angle_init(self, *args, **kwargs)

    inner: float = field(converter=float)
    """
    Radians (float):
        3D rotation angle in radians. Only one of `degrees` or `radians` should be set.

    Degrees (float):
        3D rotation angle in degrees. Only one of `degrees` or `radians` should be set.
    """

    kind: Literal["radians", "degrees"] = field(default="radians")
if TYPE_CHECKING:
    AngleLike = Union[
        Angle,float,
    ]
    AngleArrayLike = Union[
        Angle,float,
        Sequence[AngleLike],
    ]
else:
    AngleLike = Any
    AngleArrayLike = Any

# --- Arrow support ---

class AngleType(BaseExtensionType):
    def __init__(self) -> None:
        pa.ExtensionType.__init__(
            self, pa.dense_union([pa.field("_null_markers", pa.null(), nullable=True, metadata={}), pa.field("Radians", pa.float32(), nullable=False, metadata={}), pa.field("Degrees", pa.float32(), nullable=False, metadata={})]), "rerun.datatypes.Angle"
        )

class AngleArray(BaseExtensionArray[AngleArrayLike]):
    _EXTENSION_NAME = "rerun.datatypes.Angle"
    _EXTENSION_TYPE = AngleType

    @staticmethod
    def _native_to_pa_array(data: AngleArrayLike, data_type: pa.DataType) -> pa.Array:
        raise NotImplementedError

AngleType._ARRAY_TYPE = AngleArray

# TODO(cmc): bring back registration to pyarrow once legacy types are gone
# pa.register_extension_type(AngleType())


