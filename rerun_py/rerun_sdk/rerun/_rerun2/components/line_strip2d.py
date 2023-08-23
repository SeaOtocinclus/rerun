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
from ._overrides import linestrip2d_native_to_pa_array  # noqa: F401
from .. import datatypes
__all__ = ["LineStrip2D", "LineStrip2DArray", "LineStrip2DArrayLike", "LineStrip2DLike", "LineStrip2DType"]

@define
class LineStrip2D:
    """
    A line strip in 2D space.

    A line strip is a list of points connected by line segments. It can be used to draw
    approximations of smooth curves.

    The points will be connected in order, like so:
    ```text
           2------3     5
          /        \   /
    0----1          \ /
                     4
    ```
    """

    points: list[datatypes.Vec2D] = field()


if TYPE_CHECKING:
    LineStrip2DLike = Union[
        LineStrip2D,
        datatypes.Vec2DArrayLike, npt.NDArray[np.float32]
    ]
else:
    LineStrip2DLike = Any

LineStrip2DArrayLike = Union[
    LineStrip2D,
    Sequence[LineStrip2DLike],
    npt.NDArray[np.float32]
]


# --- Arrow support ---

class LineStrip2DType(BaseExtensionType):
    def __init__(self) -> None:
        pa.ExtensionType.__init__(
            self, pa.list_(pa.field("item", pa.list_(pa.field("item", pa.float32(), nullable=False, metadata={}), 2), nullable=False, metadata={})), "rerun.linestrip2d"
        )

class LineStrip2DArray(BaseExtensionArray[LineStrip2DArrayLike]):
    _EXTENSION_NAME = "rerun.linestrip2d"
    _EXTENSION_TYPE = LineStrip2DType

    @staticmethod
    def _native_to_pa_array(data: LineStrip2DArrayLike, data_type: pa.DataType) -> pa.Array:
        return linestrip2d_native_to_pa_array(data, data_type)

LineStrip2DType._ARRAY_TYPE = LineStrip2DArray

# TODO(cmc): bring back registration to pyarrow once legacy types are gone
# pa.register_extension_type(LineStrip2DType())


