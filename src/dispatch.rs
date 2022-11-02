#[macro_export]
macro_rules! numpy_dispatch {
    ($py:ident, $f:ident, $ret_type:ty, $arr:ident) => {
        |x: &'a PyAny| -> $ret_type {
            if let Ok(i) = x.extract::<numpy::PyReadonlyArray2<bool>>() {
                return $f::<bool>($py, i);
            }
            if let Ok(i) = x.extract::<numpy::PyReadonlyArray2<i8>>() {
                return $f::<i8>($py, i);
            }
            if let Ok(i) = x.extract::<numpy::PyReadonlyArray2<i16>>() {
                return $f::<i16>($py, i);
            }
            if let Ok(i) = x.extract::<numpy::PyReadonlyArray2<i32>>() {
                return $f::<i32>($py, i);
            }
            if let Ok(i) = x.extract::<numpy::PyReadonlyArray2<i64>>() {
                return $f::<i64>($py, i);
            }
            if let Ok(i) = x.extract::<numpy::PyReadonlyArray2<u8>>() {
                return $f::<u8>($py, i);
            }
            if let Ok(i) = x.extract::<numpy::PyReadonlyArray2<u16>>() {
                return $f::<u16>($py, i);
            }
            if let Ok(i) = x.extract::<numpy::PyReadonlyArray2<u32>>() {
                return $f::<u32>($py, i);
            }
            if let Ok(i) = x.extract::<numpy::PyReadonlyArray2<u64>>() {
                return $f::<u64>($py, i);
            }
            if let Ok(i) = x.extract::<numpy::PyReadonlyArray2<f32>>() {
                return $f::<f32>($py, i);
            }
            if let Ok(i) = x.extract::<numpy::PyReadonlyArray2<f64>>() {
                return $f::<f64>($py, i);
            }
            Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "Unsupported numpy dtype",
            ))
        }($arr)
    };
}
