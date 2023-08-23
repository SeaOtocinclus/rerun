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
from .. import components
__all__ = ["LineStrips2D"]

@define(str=False, repr=False)
class LineStrips2D(Archetype):
    """
    A batch of line strips with positions and optional colors, radii, labels, etc.

    Example
    -------

    Many strips:
    ```python
    import rerun as rr
    import rerun.experimental as rr2

    rr.init("line_strip2d", spawn=True)

    rr2.log(
       "batch",
       rr2.LineStrips2D(
           [
               [[0, 0], [2, 1], [4, -1], [6, 0]],
               [[0, 3], [1, 4], [2, 2], [3, 4], [4, 2], [5, 4], [6, 3]],
           ],
           colors=[[255, 0, 0], [0, 255, 0]],
           radii=[0.025, 0.005],
           labels=["one strip here", "and one strip there"],
       ),
    )

    # Log an extra rect to set the view bounds
    rr.log_rect("bounds", [3, 1.5, 8, 9], rect_format=rr.RectFormat.XCYCWH)
    ```

    Many individual segments:
    ```python
    import numpy as np
    import rerun as rr
    import rerun.experimental as rr2

    rr.init("line_segments2d", spawn=True)

    rr2.log(
       "segments",
       rr2.LineStrips2D(
           np.array(
               [
                   [[0, 0], [2, 1]],
                   [[4, -1], [6, 0]],
               ]
           )
       ),
    )

    # Log an extra rect to set the view bounds
    rr.log_rect("bounds", [3, 0, 8, 6], rect_format=rr.RectFormat.XCYCWH)
    ```
    """

    strips: components.LineStrip2DArray = field(
    metadata={'component': 'primary'}, converter=components.LineStrip2DArray.from_similar, # type: ignore[misc]
    )
    """
    All the actual 2D line strips that make up the batch.
    """

    radii: components.RadiusArray | None = field(
    metadata={'component': 'secondary'}, default=None, converter=components.RadiusArray.from_similar, # type: ignore[misc]
    )
    """
    Optional radii for the line strips.
    """

    colors: components.ColorArray | None = field(
    metadata={'component': 'secondary'}, default=None, converter=components.ColorArray.from_similar, # type: ignore[misc]
    )
    """
    Optional colors for the line strips.
    """

    labels: components.LabelArray | None = field(
    metadata={'component': 'secondary'}, default=None, converter=components.LabelArray.from_similar, # type: ignore[misc]
    )
    """
    Optional text labels for the line strips.
    """

    draw_order: components.DrawOrderArray | None = field(
    metadata={'component': 'secondary'}, default=None, converter=components.DrawOrderArray.from_similar, # type: ignore[misc]
    )
    """
    An optional floating point value that specifies the 2D drawing order of each line strip.
    Objects with higher values are drawn on top of those with lower values.

    The default for 2D lines is 20.0.
    """

    class_ids: components.ClassIdArray | None = field(
    metadata={'component': 'secondary'}, default=None, converter=components.ClassIdArray.from_similar, # type: ignore[misc]
    )
    """
    Optional `ClassId`s for the lines.

    The class ID provides colors and labels if not specified explicitly.
    """

    instance_keys: components.InstanceKeyArray | None = field(
    metadata={'component': 'secondary'}, default=None, converter=components.InstanceKeyArray.from_similar, # type: ignore[misc]
    )
    """
    Unique identifiers for each individual line strip in the batch.
    """

    __str__ = Archetype.__str__
    __repr__ = Archetype.__repr__



