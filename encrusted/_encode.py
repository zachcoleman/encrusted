import numpy as np

from ._encrusted_ext import (
    py_encode_1,
    py_encode_2,
    py_encode_3,
    py_encode_4,
    py_encode_dyn,
)

NDIM_TO_ENCODE = {1: py_encode_1, 2: py_encode_2, 3: py_encode_3, 4: py_encode_4}

DTYPE_TO_HEADER = {
    np.dtype("bool"): 0,
    np.dtype("uint8"): 1,
    np.dtype("uint16"): 2,
    np.dtype("uint32"): 3,
    np.dtype("uint64"): 4,
    np.dtype("int8"): 5,
    np.dtype("int16"): 6,
    np.dtype("int32"): 7,
    np.dtype("int64"): 8,
    np.dtype("float32"): 9,
    np.dtype("float64"): 10,
}


def encode(arr: np.ndarray, compressed: bool = True) -> str:
    """Encode a numpy array into a base64 string w/ the following
    format:
    ```
      [0 raw / 1 compressed][0..4 for dim][0..10 for dtype][...]
    ```
    The first 4 characters of the base64 string are reserved.
    The rest of the string is the encoded bytes of encoded array.

    5 dimensions:
      0 - dyn
      1 - 1
      ...
      4 - 4

    11 data types:
      0  - bool
      1  - u8
      2  - u16
      3  - u32
      4  - u64
      5  - i8
      6  - i16
      7  - i32
      8  - i64
      9  - f32
      10 - f64

    Args:
        arr: np.ndarray
        compressed: bool if array should be compressed w/ DEFLATE
    Returns:
        base64 encoded string
    """
    encode_fn = NDIM_TO_ENCODE.get(arr.ndim, py_encode_dyn)
    compressed_byte = 1 if compressed else 0
    dim_byte = arr.ndim if arr.ndim in NDIM_TO_ENCODE else 0
    if arr.dtype in DTYPE_TO_HEADER:
        return (
            f"{compressed_byte}{dim_byte}{DTYPE_TO_HEADER.get(arr.dtype):02}"
            + encode_fn(compressed, arr)
        )
    raise TypeError(f"unsupported dtype for arr: {arr.dtype}")
