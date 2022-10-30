import numpy as np

from ._encrusted_ext import (
    _encode_mask
)

def encode(arr: np.ndarray) -> str:
    return _encode_mask(arr)