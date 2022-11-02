#[macro_export]
macro_rules! make_decode_fns {
    ($fn_name:ident, $numpy_type:ty, $ndarray_type:ty) => {
        // bool
        paste::paste!{
            #[pyfunction]
            #[pyo3(name = "py_" $fn_name "_bool")]
            #[pyo3(text_signature = "(byte_str: str, /)")]
            pub fn [< $fn_name _bool >]<'a>(py: Python<'a>, byte_str: String) -> PyResult<&'a $numpy_type<bool>> {
                let bytes = str_to_bytes(byte_str);
                let arr: $ndarray_type<bool> = deserialize(&bytes).unwrap();
                Ok(<$numpy_type<bool>>::from_array(py, &arr))
            }
        }
        // u8
        paste::paste!{
            #[pyfunction]
            #[pyo3(name = "py_" $fn_name "_u8")]
            #[pyo3(text_signature = "(byte_str: str, /)")]
            pub fn [< $fn_name _u8 >]<'a>(py: Python<'a>, byte_str: String) -> PyResult<&'a $numpy_type<u8>> {
                let bytes = str_to_bytes(byte_str);
                let arr: $ndarray_type<u8> = deserialize(&bytes).unwrap();
                Ok(<$numpy_type<u8>>::from_array(py, &arr))
            }
        }
        // u16
        paste::paste!{
            #[pyfunction]
            #[pyo3(name = "py_" $fn_name "_u16")]
            #[pyo3(text_signature = "(byte_str: str, /)")]
            pub fn [< $fn_name _u16 >]<'a>(py: Python<'a>, byte_str: String) -> PyResult<&'a $numpy_type<u16>> {
                let bytes = str_to_bytes(byte_str);
                let arr: $ndarray_type<u16> = deserialize(&bytes).unwrap();
                Ok(<$numpy_type<u16>>::from_array(py, &arr))
            }
        }
        // u32
        paste::paste!{
            #[pyfunction]
            #[pyo3(name = "py_" $fn_name "_u32")]
            #[pyo3(text_signature = "(byte_str: str, /)")]
            pub fn [< $fn_name _u32 >]<'a>(py: Python<'a>, byte_str: String) -> PyResult<&'a $numpy_type<u32>> {
                let bytes = str_to_bytes(byte_str);
                let arr: $ndarray_type<u32> = deserialize(&bytes).unwrap();
                Ok(<$numpy_type<u32>>::from_array(py, &arr))
            }
        }
        // u64
        paste::paste!{
            #[pyfunction]
            #[pyo3(name = "py_" $fn_name "_u64")]
            #[pyo3(text_signature = "(byte_str: str, /)")]
            pub fn [< $fn_name _u64 >]<'a>(py: Python<'a>, byte_str: String) -> PyResult<&'a $numpy_type<u64>> {
                let bytes = str_to_bytes(byte_str);
                let arr: $ndarray_type<u64> = deserialize(&bytes).unwrap();
                Ok(<$numpy_type<u64>>::from_array(py, &arr))
            }
        }
        // i8
        paste::paste!{
            #[pyfunction]
            #[pyo3(name = "py_" $fn_name "_i8")]
            #[pyo3(text_signature = "(byte_str: str, /)")]
            pub fn [< $fn_name _i8 >]<'a>(py: Python<'a>, byte_str: String) -> PyResult<&'a $numpy_type<i8>> {
                let bytes = str_to_bytes(byte_str);
                let arr: $ndarray_type<i8> = deserialize(&bytes).unwrap();
                Ok(<$numpy_type<i8>>::from_array(py, &arr))
            }
        }
        // i16
        paste::paste!{
            #[pyfunction]
            #[pyo3(name = "py_" $fn_name "_i16")]
            #[pyo3(text_signature = "(byte_str: str, /)")]
            pub fn [< $fn_name _i16 >]<'a>(py: Python<'a>, byte_str: String) -> PyResult<&'a $numpy_type<i16>> {
                let bytes = str_to_bytes(byte_str);
                let arr: $ndarray_type<i16> = deserialize(&bytes).unwrap();
                Ok(<$numpy_type<i16>>::from_array(py, &arr))
            }
        }
        // i32
        paste::paste!{
            #[pyfunction]
            #[pyo3(name = "py_" $fn_name "_i32")]
            #[pyo3(text_signature = "(byte_str: str, /)")]
            pub fn [< $fn_name _i32 >]<'a>(py: Python<'a>, byte_str: String) -> PyResult<&'a $numpy_type<i32>> {
                let bytes = str_to_bytes(byte_str);
                let arr: $ndarray_type<i32> = deserialize(&bytes).unwrap();
                Ok(<$numpy_type<i32>>::from_array(py, &arr))
            }
        }
        // i64
        paste::paste!{
            #[pyfunction]
            #[pyo3(name = "py_" $fn_name "_i64")]
            #[pyo3(text_signature = "(byte_str: str, /)")]
            pub fn [< $fn_name _i64 >]<'a>(py: Python<'a>, byte_str: String) -> PyResult<&'a $numpy_type<i64>> {
                let bytes = str_to_bytes(byte_str);
                let arr: $ndarray_type<i64> = deserialize(&bytes).unwrap();
                Ok(<$numpy_type<i64>>::from_array(py, &arr))
            }
        }
        // f32
        paste::paste!{
            #[pyfunction]
            #[pyo3(name = "py_" $fn_name "_f32")]
            #[pyo3(text_signature = "(byte_str: str, /)")]
            pub fn [< $fn_name _f32 >]<'a>(py: Python<'a>, byte_str: String) -> PyResult<&'a $numpy_type<f32>> {
                let bytes = str_to_bytes(byte_str);
                let arr: $ndarray_type<f32> = deserialize(&bytes).unwrap();
                Ok(<$numpy_type<f32>>::from_array(py, &arr))
            }
        }
        // f64
        paste::paste!{
            #[pyfunction]
            #[pyo3(name = "py_" $fn_name "_f64")]
            #[pyo3(text_signature = "(byte_str: str, /)")]
            pub fn [< $fn_name _f64 >]<'a>(py: Python<'a>, byte_str: String) -> PyResult<&'a $numpy_type<f64>> {
                let bytes = str_to_bytes(byte_str);
                let arr: $ndarray_type<f64> = deserialize(&bytes).unwrap();
                Ok(<$numpy_type<f64>>::from_array(py, &arr))
            }
        }
    };
}

#[macro_export]
macro_rules! make_module_exports {
    ($fn_name:ident, $m:ident) => {
        paste::paste! {
            $m.add_function(wrap_pyfunction!(decode::[< $fn_name _bool >], $m)?)?;
            $m.add_function(wrap_pyfunction!(decode::[< $fn_name _u8>], $m)?)?;
            $m.add_function(wrap_pyfunction!(decode::[< $fn_name _u16>], $m)?)?;
            $m.add_function(wrap_pyfunction!(decode::[< $fn_name _u32>], $m)?)?;
            $m.add_function(wrap_pyfunction!(decode::[< $fn_name _u64>], $m)?)?;
            $m.add_function(wrap_pyfunction!(decode::[< $fn_name _i8>], $m)?)?;
            $m.add_function(wrap_pyfunction!(decode::[< $fn_name _i16>], $m)?)?;
            $m.add_function(wrap_pyfunction!(decode::[< $fn_name _i32>], $m)?)?;
            $m.add_function(wrap_pyfunction!(decode::[< $fn_name _i64>], $m)?)?;
            $m.add_function(wrap_pyfunction!(decode::[< $fn_name _f32>], $m)?)?;
            $m.add_function(wrap_pyfunction!(decode::[< $fn_name _f64>], $m)?)?;
        }
    };
}
