import numpy as np

from ._encrusted_ext import (
    _decode_mask_bool,
    _decode_mask_u8,
    _decode_mask_u16,
    _decode_mask_u32,
    _decode_mask_u64,
    _decode_mask_i8,
    _decode_mask_i16,
    _decode_mask_i32,
    _decode_mask_i64,
    _decode_mask_f32,
    _decode_mask_f64,
)

DTYPE_MAPPING = {
    np.bool: _decode_mask_bool,
    bool: _decode_mask_bool,
    np.uint8: _decode_mask_u8,
    np.uint16: _decode_mask_u16,
    np.uint32: _decode_mask_u32,
    np.uint64: _decode_mask_u64,
    np.int8: _decode_mask_i8,
    np.int16: _decode_mask_i16,
    np.int32: _decode_mask_i32,
    np.int64: _decode_mask_i64,
    np.float32: _decode_mask_f32,
    np.float64: _decode_mask_f64,
}

def decode(byte_str: str, dtype: type) -> np.ndarray:
    fn = DTYPE_MAPPING.get(dtype, False)
    if fn:
        return fn(byte_str)
    raise TypeError(f"{dtype} invalid type.")
