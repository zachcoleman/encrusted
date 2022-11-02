use bincode::deserialize;
use numpy::*;
use pyo3::prelude::*;

use flate2::write::DeflateDecoder;
use std::io::prelude::*;

use crate::make_decode_fns;

/*
Data Format:
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
*/

fn str_to_bytes(compressed: bool, byte_str: String) -> Vec<u8> {
    let bytes = base64::decode(byte_str).unwrap();
    if compressed {
        let mut writer = Vec::new();
        let mut deflater = DeflateDecoder::new(writer);
        deflater.write_all(&bytes[..]).unwrap();
        writer = deflater.finish().unwrap();
        writer
    } else {
        bytes
    }
}

make_decode_fns!(decode_1, numpy::PyArray1, ndarray::Array1);
make_decode_fns!(decode_2, numpy::PyArray2, ndarray::Array2);
make_decode_fns!(decode_3, numpy::PyArray3, ndarray::Array3);
make_decode_fns!(decode_4, numpy::PyArray4, ndarray::Array4);
make_decode_fns!(decode_dyn, numpy::PyArrayDyn, ndarray::ArrayD);
