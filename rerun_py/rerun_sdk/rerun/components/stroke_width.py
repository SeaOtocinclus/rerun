# DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/python.rs
# Based on "crates/re_types/definitions/rerun/components/stroke_width.fbs".

# You can extend this class by creating a "StrokeWidthExt" class in "stroke_width_ext.py".

from __future__ import annotations

from typing import TYPE_CHECKING, Any, Sequence, Union

import numpy as np
import numpy.typing as npt
import pyarrow as pa
from attrs import define, field

from .._baseclasses import BaseBatch, BaseExtensionType, ComponentBatchMixin
from .stroke_width_ext import StrokeWidthExt

__all__ = ["StrokeWidth", "StrokeWidthArrayLike", "StrokeWidthBatch", "StrokeWidthLike", "StrokeWidthType"]


@define(init=False)
class StrokeWidth(StrokeWidthExt):
    """**Component**: The width of a stroke specified in UI points."""

    def __init__(self: Any, width: StrokeWidthLike):
        """Create a new instance of the StrokeWidth component."""

        # You can define your own __init__ function as a member of StrokeWidthExt in stroke_width_ext.py
        self.__attrs_init__(width=width)

    width: float = field(converter=float)

    def __array__(self, dtype: npt.DTypeLike = None) -> npt.NDArray[Any]:
        # You can define your own __array__ function as a member of StrokeWidthExt in stroke_width_ext.py
        return np.asarray(self.width, dtype=dtype)

    def __float__(self) -> float:
        return float(self.width)


if TYPE_CHECKING:
    StrokeWidthLike = Union[StrokeWidth, float]
else:
    StrokeWidthLike = Any

StrokeWidthArrayLike = Union[StrokeWidth, Sequence[StrokeWidthLike], float, npt.ArrayLike]


class StrokeWidthType(BaseExtensionType):
    _TYPE_NAME: str = "rerun.components.StrokeWidth"

    def __init__(self) -> None:
        pa.ExtensionType.__init__(self, pa.float32(), self._TYPE_NAME)


class StrokeWidthBatch(BaseBatch[StrokeWidthArrayLike], ComponentBatchMixin):
    _ARROW_TYPE = StrokeWidthType()

    @staticmethod
    def _native_to_pa_array(data: StrokeWidthArrayLike, data_type: pa.DataType) -> pa.Array:
        return StrokeWidthExt.native_to_pa_array_override(data, data_type)
