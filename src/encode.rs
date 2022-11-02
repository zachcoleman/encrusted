use numpy::*;
use pyo3::prelude::*;

use flate2::write::DeflateEncoder;
use flate2::Compression;
use std::io::prelude::*;

use crate::make_encode_fns;
use crate::make_py_encode_fns;
use crate::numpy_dispatch_x1;
use crate::numpy_dispatch_x2;
use crate::numpy_dispatch_x3;
use crate::numpy_dispatch_x4;
use crate::numpy_dispatch_xd;

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

make_encode_fns!(encode_dyn, PyReadonlyArrayDyn);
make_encode_fns!(encode_1, PyReadonlyArray1);
make_encode_fns!(encode_2, PyReadonlyArray2);
make_encode_fns!(encode_3, PyReadonlyArray3);
make_encode_fns!(encode_4, PyReadonlyArray4);

make_py_encode_fns!(encode_dyn, xd, PyReadonlyArrayDyn);
make_py_encode_fns!(encode_1, x1, PyReadonlyArray1);
make_py_encode_fns!(encode_2, x2, PyReadonlyArray2);
make_py_encode_fns!(encode_3, x3, PyReadonlyArray3);
make_py_encode_fns!(encode_4, x4, PyReadonlyArray4);
