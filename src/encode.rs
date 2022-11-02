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
