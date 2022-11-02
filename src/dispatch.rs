// dim 1
#[macro_export]
macro_rules! numpy_dispatch_x1 {
    ($py:ident, $f:ident, $ret_type:ty, $c:ident, $arr:ident) => {
        |c: bool, x: &'a PyAny| -> $ret_type {
            if let Ok(i) = x.extract::<numpy::PyReadonlyArray1<bool>>() {
                return $f::<bool>($py, c, i);
            }
            if let Ok(i) = x.extract::<numpy::PyReadonlyArray1<i8>>() {
                return $f::<i8>($py, c, i);
            }
            if let Ok(i) = x.extract::<numpy::PyReadonlyArray1<i16>>() {
                return $f::<i16>($py, c, i);
            }
            if let Ok(i) = x.extract::<numpy::PyReadonlyArray1<i32>>() {
                return $f::<i32>($py, c, i);
            }
            if let Ok(i) = x.extract::<numpy::PyReadonlyArray1<i64>>() {
                return $f::<i64>($py, c, i);
            }
            if let Ok(i) = x.extract::<numpy::PyReadonlyArray1<u8>>() {
                return $f::<u8>($py, c, i);
            }
            if let Ok(i) = x.extract::<numpy::PyReadonlyArray1<u16>>() {
                return $f::<u16>($py, c, i);
            }
            if let Ok(i) = x.extract::<numpy::PyReadonlyArray1<u32>>() {
                return $f::<u32>($py, c, i);
            }
            if let Ok(i) = x.extract::<numpy::PyReadonlyArray1<u64>>() {
                return $f::<u64>($py, c, i);
            }
            if let Ok(i) = x.extract::<numpy::PyReadonlyArray1<f32>>() {
                return $f::<f32>($py, c, i);
            }
            if let Ok(i) = x.extract::<numpy::PyReadonlyArray1<f64>>() {
                return $f::<f64>($py, c, i);
            }
            Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "Unsupported numpy dtype",
            ))
        }($c, $arr)
    };
}

// dim 2
#[macro_export]
macro_rules! numpy_dispatch_x2 {
    ($py:ident, $f:ident, $ret_type:ty, $c:ident, $arr:ident) => {
        |c: bool, x: &'a PyAny| -> $ret_type {
            // dim 2
            if let Ok(i) = x.extract::<numpy::PyReadonlyArray2<bool>>() {
                return $f::<bool>($py, c, i);
            }
            if let Ok(i) = x.extract::<numpy::PyReadonlyArray2<i8>>() {
                return $f::<i8>($py, c, i);
            }
            if let Ok(i) = x.extract::<numpy::PyReadonlyArray2<i16>>() {
                return $f::<i16>($py, c, i);
            }
            if let Ok(i) = x.extract::<numpy::PyReadonlyArray2<i32>>() {
                return $f::<i32>($py, c, i);
            }
            if let Ok(i) = x.extract::<numpy::PyReadonlyArray2<i64>>() {
                return $f::<i64>($py, c, i);
            }
            if let Ok(i) = x.extract::<numpy::PyReadonlyArray2<u8>>() {
                return $f::<u8>($py, c, i);
            }
            if let Ok(i) = x.extract::<numpy::PyReadonlyArray2<u16>>() {
                return $f::<u16>($py, c, i);
            }
            if let Ok(i) = x.extract::<numpy::PyReadonlyArray2<u32>>() {
                return $f::<u32>($py, c, i);
            }
            if let Ok(i) = x.extract::<numpy::PyReadonlyArray2<u64>>() {
                return $f::<u64>($py, c, i);
            }
            if let Ok(i) = x.extract::<numpy::PyReadonlyArray2<f32>>() {
                return $f::<f32>($py, c, i);
            }
            if let Ok(i) = x.extract::<numpy::PyReadonlyArray2<f64>>() {
                return $f::<f64>($py, c, i);
            }
            Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "Unsupported numpy dtype",
            ))
        }($c, $arr)
    };
}

// dim 3
#[macro_export]
macro_rules! numpy_dispatch_x3 {
    ($py:ident, $f:ident, $ret_type:ty, $c:ident, $arr:ident) => {
        |c: bool, x: &'a PyAny| -> $ret_type {
            if let Ok(i) = x.extract::<numpy::PyReadonlyArray3<bool>>() {
                return $f::<bool>($py, c, i);
            }
            if let Ok(i) = x.extract::<numpy::PyReadonlyArray3<i8>>() {
                return $f::<i8>($py, c, i);
            }
            if let Ok(i) = x.extract::<numpy::PyReadonlyArray3<i16>>() {
                return $f::<i16>($py, c, i);
            }
            if let Ok(i) = x.extract::<numpy::PyReadonlyArray3<i32>>() {
                return $f::<i32>($py, c, i);
            }
            if let Ok(i) = x.extract::<numpy::PyReadonlyArray3<i64>>() {
                return $f::<i64>($py, c, i);
            }
            if let Ok(i) = x.extract::<numpy::PyReadonlyArray3<u8>>() {
                return $f::<u8>($py, c, i);
            }
            if let Ok(i) = x.extract::<numpy::PyReadonlyArray3<u16>>() {
                return $f::<u16>($py, c, i);
            }
            if let Ok(i) = x.extract::<numpy::PyReadonlyArray3<u32>>() {
                return $f::<u32>($py, c, i);
            }
            if let Ok(i) = x.extract::<numpy::PyReadonlyArray3<u64>>() {
                return $f::<u64>($py, c, i);
            }
            if let Ok(i) = x.extract::<numpy::PyReadonlyArray3<f32>>() {
                return $f::<f32>($py, c, i);
            }
            if let Ok(i) = x.extract::<numpy::PyReadonlyArray3<f64>>() {
                return $f::<f64>($py, c, i);
            }
            Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "Unsupported numpy dtype",
            ))
        }($c, $arr)
    };
}

// dim 4
#[macro_export]
macro_rules! numpy_dispatch_x4 {
    ($py:ident, $f:ident, $ret_type:ty, $c:ident, $arr:ident) => {
        |c: bool, x: &'a PyAny| -> $ret_type {
            if let Ok(i) = x.extract::<numpy::PyReadonlyArray4<bool>>() {
                return $f::<bool>($py, c, i);
            }
            if let Ok(i) = x.extract::<numpy::PyReadonlyArray4<i8>>() {
                return $f::<i8>($py, c, i);
            }
            if let Ok(i) = x.extract::<numpy::PyReadonlyArray4<i16>>() {
                return $f::<i16>($py, c, i);
            }
            if let Ok(i) = x.extract::<numpy::PyReadonlyArray4<i32>>() {
                return $f::<i32>($py, c, i);
            }
            if let Ok(i) = x.extract::<numpy::PyReadonlyArray4<i64>>() {
                return $f::<i64>($py, c, i);
            }
            if let Ok(i) = x.extract::<numpy::PyReadonlyArray4<u8>>() {
                return $f::<u8>($py, c, i);
            }
            if let Ok(i) = x.extract::<numpy::PyReadonlyArray4<u16>>() {
                return $f::<u16>($py, c, i);
            }
            if let Ok(i) = x.extract::<numpy::PyReadonlyArray4<u32>>() {
                return $f::<u32>($py, c, i);
            }
            if let Ok(i) = x.extract::<numpy::PyReadonlyArray4<u64>>() {
                return $f::<u64>($py, c, i);
            }
            if let Ok(i) = x.extract::<numpy::PyReadonlyArray4<f32>>() {
                return $f::<f32>($py, c, i);
            }
            if let Ok(i) = x.extract::<numpy::PyReadonlyArray4<f64>>() {
                return $f::<f64>($py, c, i);
            }
            Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "Unsupported numpy dtype",
            ))
        }($c, $arr)
    };
}

// dim dyn
#[macro_export]
macro_rules! numpy_dispatch_xd {
    ($py:ident, $f:ident, $ret_type:ty, $c:ident, $arr:ident) => {
        |c: bool, x: &'a PyAny| -> $ret_type {
            if let Ok(i) = x.extract::<numpy::PyReadonlyArrayDyn<bool>>() {
                return $f::<bool>($py, c, i);
            }
            if let Ok(i) = x.extract::<numpy::PyReadonlyArrayDyn<i8>>() {
                return $f::<i8>($py, c, i);
            }
            if let Ok(i) = x.extract::<numpy::PyReadonlyArrayDyn<i16>>() {
                return $f::<i16>($py, c, i);
            }
            if let Ok(i) = x.extract::<numpy::PyReadonlyArrayDyn<i32>>() {
                return $f::<i32>($py, c, i);
            }
            if let Ok(i) = x.extract::<numpy::PyReadonlyArrayDyn<i64>>() {
                return $f::<i64>($py, c, i);
            }
            if let Ok(i) = x.extract::<numpy::PyReadonlyArrayDyn<u8>>() {
                return $f::<u8>($py, c, i);
            }
            if let Ok(i) = x.extract::<numpy::PyReadonlyArrayDyn<u16>>() {
                return $f::<u16>($py, c, i);
            }
            if let Ok(i) = x.extract::<numpy::PyReadonlyArrayDyn<u32>>() {
                return $f::<u32>($py, c, i);
            }
            if let Ok(i) = x.extract::<numpy::PyReadonlyArrayDyn<u64>>() {
                return $f::<u64>($py, c, i);
            }
            if let Ok(i) = x.extract::<numpy::PyReadonlyArrayDyn<f32>>() {
                return $f::<f32>($py, c, i);
            }
            if let Ok(i) = x.extract::<numpy::PyReadonlyArrayDyn<f64>>() {
                return $f::<f64>($py, c, i);
            }
            Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "Unsupported numpy dtype",
            ))
        }($c, $arr)
    };
}
