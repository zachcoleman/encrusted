import numpy as np

from encrusted import decode, encode

SIZE_1 = (10,)
SIZE_2 = (10, 10)
SIZE_3 = (10, 10, 10)
SIZE_4 = (10, 10, 10, 10)
SIZE_D = (10, 10, 10, 10, 10)

SIZES = [SIZE_1, SIZE_2, SIZE_3, SIZE_4, SIZE_D]
TYPES = [
    bool,
    np.uint8,
    np.uint16,
    np.uint32,
    np.uint64,
    np.int8,
    np.int16,
    np.int32,
    np.int64,
    np.float32,
    np.float64,
]
COMPRESSED = [False, True]


def test_encode_decode():
    for s in SIZES:
        for t in TYPES:
            for c in COMPRESSED:
                arr = np.random.random(s).astype(t)
                assert np.array_equal(
                    decode(encode(arr, c)), arr
                ), f"failed on: size: {s} type: {t} compression: {c}"
