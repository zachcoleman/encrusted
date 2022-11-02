import numpy as np

from ._encrusted_ext import (
    py_decode_1_bool,
    py_decode_1_f32,
    py_decode_1_f64,
    py_decode_1_i8,
    py_decode_1_i16,
    py_decode_1_i32,
    py_decode_1_i64,
    py_decode_1_u8,
    py_decode_1_u16,
    py_decode_1_u32,
    py_decode_1_u64,
    py_decode_2_bool,
    py_decode_2_f32,
    py_decode_2_f64,
    py_decode_2_i8,
    py_decode_2_i16,
    py_decode_2_i32,
    py_decode_2_i64,
    py_decode_2_u8,
    py_decode_2_u16,
    py_decode_2_u32,
    py_decode_2_u64,
    py_decode_3_bool,
    py_decode_3_f32,
    py_decode_3_f64,
    py_decode_3_i8,
    py_decode_3_i16,
    py_decode_3_i32,
    py_decode_3_i64,
    py_decode_3_u8,
    py_decode_3_u16,
    py_decode_3_u32,
    py_decode_3_u64,
    py_decode_4_bool,
    py_decode_4_f32,
    py_decode_4_f64,
    py_decode_4_i8,
    py_decode_4_i16,
    py_decode_4_i32,
    py_decode_4_i64,
    py_decode_4_u8,
    py_decode_4_u16,
    py_decode_4_u32,
    py_decode_4_u64,
    py_decode_dyn_bool,
    py_decode_dyn_f32,
    py_decode_dyn_f64,
    py_decode_dyn_i8,
    py_decode_dyn_i16,
    py_decode_dyn_i32,
    py_decode_dyn_i64,
    py_decode_dyn_u8,
    py_decode_dyn_u16,
    py_decode_dyn_u32,
    py_decode_dyn_u64,
)

DTYPE_MAPPING_DYN = {
    0: py_decode_dyn_bool,
    1: py_decode_dyn_u8,
    2: py_decode_dyn_u16,
    3: py_decode_dyn_u32,
    4: py_decode_dyn_u64,
    5: py_decode_dyn_i8,
    6: py_decode_dyn_i16,
    7: py_decode_dyn_i32,
    8: py_decode_dyn_i64,
    9: py_decode_dyn_f32,
    10: py_decode_dyn_f64,
}
DTYPE_MAPPING_1 = {
    0: py_decode_1_bool,
    1: py_decode_1_u8,
    2: py_decode_1_u16,
    3: py_decode_1_u32,
    4: py_decode_1_u64,
    5: py_decode_1_i8,
    6: py_decode_1_i16,
    7: py_decode_1_i32,
    8: py_decode_1_i64,
    9: py_decode_1_f32,
    10: py_decode_1_f64,
}
DTYPE_MAPPING_2 = {
    0: py_decode_2_bool,
    1: py_decode_2_u8,
    2: py_decode_2_u16,
    3: py_decode_2_u32,
    4: py_decode_2_u64,
    5: py_decode_2_i8,
    6: py_decode_2_i16,
    7: py_decode_2_i32,
    8: py_decode_2_i64,
    9: py_decode_2_f32,
    10: py_decode_2_f64,
}
DTYPE_MAPPING_3 = {
    0: py_decode_3_bool,
    1: py_decode_3_u8,
    2: py_decode_3_u16,
    3: py_decode_3_u32,
    4: py_decode_3_u64,
    5: py_decode_3_i8,
    6: py_decode_3_i16,
    7: py_decode_3_i32,
    8: py_decode_3_i64,
    9: py_decode_3_f32,
    10: py_decode_3_f64,
}
DTYPE_MAPPING_4 = {
    0: py_decode_4_bool,
    1: py_decode_4_u8,
    2: py_decode_4_u16,
    3: py_decode_4_u32,
    4: py_decode_4_u64,
    5: py_decode_4_i8,
    6: py_decode_4_i16,
    7: py_decode_4_i32,
    8: py_decode_4_i64,
    9: py_decode_4_f32,
    10: py_decode_4_f64,
}
SHAPE_TO_MAPPING = {
    0: DTYPE_MAPPING_DYN,
    1: DTYPE_MAPPING_1,
    2: DTYPE_MAPPING_2,
    3: DTYPE_MAPPING_3,
    4: DTYPE_MAPPING_4,
}


def decode(byte_str: str) -> np.ndarray:
    """Decode a numpy array from a base64 string w/ the following
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
        byte_str: base64 encoded string
    Returns:
        np.ndarray
    """
    compressed = byte_str[0] == "1"
    dim_key = int(byte_str[1])
    dtype_key = int(byte_str[2:4])
    decode_fn = SHAPE_TO_MAPPING.get(dim_key, {}).get(dtype_key, False)
    if decode_fn:
        return decode_fn(compressed, byte_str[4:])
    raise TypeError(f"invalid headers {byte_str[:4]}")
