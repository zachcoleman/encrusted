from collections import defaultdict

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

DTYPE_MAPPING_1 = {
    np.bool: py_decode_1_bool,
    bool: py_decode_1_bool,
    np.uint8: py_decode_1_u8,
    np.uint16: py_decode_1_u16,
    np.uint32: py_decode_1_u32,
    np.uint64: py_decode_1_u64,
    np.int8: py_decode_1_i8,
    np.int16: py_decode_1_i16,
    np.int32: py_decode_1_i32,
    np.int64: py_decode_1_i64,
    np.float32: py_decode_1_f32,
    np.float64: py_decode_1_f64,
}
DTYPE_MAPPING_2 = {
    np.bool: py_decode_2_bool,
    bool: py_decode_2_bool,
    np.uint8: py_decode_2_u8,
    np.uint16: py_decode_2_u16,
    np.uint32: py_decode_2_u32,
    np.uint64: py_decode_2_u64,
    np.int8: py_decode_2_i8,
    np.int16: py_decode_2_i16,
    np.int32: py_decode_2_i32,
    np.int64: py_decode_2_i64,
    np.float32: py_decode_2_f32,
    np.float64: py_decode_2_f64,
}
DTYPE_MAPPING_3 = {
    np.bool: py_decode_3_bool,
    bool: py_decode_3_bool,
    np.uint8: py_decode_3_u8,
    np.uint16: py_decode_3_u16,
    np.uint32: py_decode_3_u32,
    np.uint64: py_decode_3_u64,
    np.int8: py_decode_3_i8,
    np.int16: py_decode_3_i16,
    np.int32: py_decode_3_i32,
    np.int64: py_decode_3_i64,
    np.float32: py_decode_3_f32,
    np.float64: py_decode_3_f64,
}
DTYPE_MAPPING_4 = {
    np.bool: py_decode_4_bool,
    bool: py_decode_4_bool,
    np.uint8: py_decode_4_u8,
    np.uint16: py_decode_4_u16,
    np.uint32: py_decode_4_u32,
    np.uint64: py_decode_4_u64,
    np.int8: py_decode_4_i8,
    np.int16: py_decode_4_i16,
    np.int32: py_decode_4_i32,
    np.int64: py_decode_4_i64,
    np.float32: py_decode_4_f32,
    np.float64: py_decode_4_f64,
}
DTYPE_MAPPING_DYN = {
    np.bool: py_decode_dyn_bool,
    bool: py_decode_dyn_bool,
    np.uint8: py_decode_dyn_u8,
    np.uint16: py_decode_dyn_u16,
    np.uint32: py_decode_dyn_u32,
    np.uint64: py_decode_dyn_u64,
    np.int8: py_decode_dyn_i8,
    np.int16: py_decode_dyn_i16,
    np.int32: py_decode_dyn_i32,
    np.int64: py_decode_dyn_i64,
    np.float32: py_decode_dyn_f32,
    np.float64: py_decode_dyn_f64,
}

SHAPE_TO_MAPPING = defaultdict(lambda: DTYPE_MAPPING_DYN)
SHAPE_TO_MAPPING[1] = DTYPE_MAPPING_1
SHAPE_TO_MAPPING[2] = DTYPE_MAPPING_2
SHAPE_TO_MAPPING[3] = DTYPE_MAPPING_3
SHAPE_TO_MAPPING[4] = DTYPE_MAPPING_4


def decode(byte_str: str, dtype: type, dim: int) -> np.ndarray:
    fn = SHAPE_TO_MAPPING[dim].get(dtype, False)
    if fn:
        return fn(byte_str)
    raise TypeError(f"{dtype} invalid type.")
