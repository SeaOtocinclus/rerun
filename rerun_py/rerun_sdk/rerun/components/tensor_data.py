# DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/python.rs
# Based on "crates/re_types/definitions/rerun/components/tensor_data.fbs".

# You can extend this class by creating a "TensorDataExt" class in "tensor_data_ext.py".

from __future__ import annotations

from .. import datatypes
from .._baseclasses import ComponentBatchMixin

__all__ = ["TensorData", "TensorDataBatch", "TensorDataType"]


class TensorData(datatypes.TensorData):
    """
    **Component**: A multi-dimensional `Tensor` of data.

    The number of dimensions and their respective lengths is specified by the `shape` field.
    The dimensions are ordered from outermost to innermost. For example, in the common case of
    a 2D RGB Image, the shape would be `[height, width, channel]`.

    These dimensions are combined with an index to look up values from the `buffer` field,
    which stores a contiguous array of typed values.

    Note that the buffer may be encoded in a compressed format such as `jpeg` or
    in a format with downsampled chroma, such as NV12 or YUY2.
    For file formats, the shape is used as a hint, for chroma downsampled format
    the shape has to be the shape of the decoded image.
    """

    # You can define your own __init__ function as a member of TensorDataExt in tensor_data_ext.py

    # Note: there are no fields here because TensorData delegates to datatypes.TensorData
    pass


class TensorDataType(datatypes.TensorDataType):
    _TYPE_NAME: str = "rerun.components.TensorData"


class TensorDataBatch(datatypes.TensorDataBatch, ComponentBatchMixin):
    _ARROW_TYPE = TensorDataType()
