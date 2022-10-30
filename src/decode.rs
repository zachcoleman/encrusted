use base64::{decode};
use bincode::{deserialize};
use numpy::*;
use pyo3::prelude::*;

#[pyfunction]
#[pyo3(name = "_decode_mask_bool")]
#[pyo3(text_signature = "(byte_str: str, /)")]
pub fn py_decode_mask_bool<'a>(py: Python<'a>, byte_str: String) -> PyResult<&'a PyArray2<bool>> {
    let bytes: Vec<u8> = decode(byte_str).unwrap();
    let arr: ndarray::Array2<bool> = deserialize(&bytes).unwrap();
    Ok(PyArray2::from_array(py, &arr))
}

#[pyfunction]
#[pyo3(name = "_decode_mask_u8")]
#[pyo3(text_signature = "(byte_str: str, /)")]
pub fn py_decode_mask_u8<'a>(py: Python<'a>, byte_str: String) -> PyResult<&'a PyArray2<u8>> {
    let bytes: Vec<u8> = decode(byte_str).unwrap();
    let arr: ndarray::Array2<u8> = deserialize(&bytes).unwrap();
    Ok(PyArray2::from_array(py, &arr))
}

#[pyfunction]
#[pyo3(name = "_decode_mask_u16")]
#[pyo3(text_signature = "(byte_str: str, /)")]
pub fn py_decode_mask_u16<'a>(py: Python<'a>, byte_str: String) -> PyResult<&'a PyArray2<u16>> {
    let bytes: Vec<u8> = decode(byte_str).unwrap();
    let arr: ndarray::Array2<u16> = deserialize(&bytes).unwrap();
    Ok(PyArray2::from_array(py, &arr))
}

#[pyfunction]
#[pyo3(name = "_decode_mask_u32")]
#[pyo3(text_signature = "(byte_str: str, /)")]
pub fn py_decode_mask_u32<'a>(py: Python<'a>, byte_str: String) -> PyResult<&'a PyArray2<u32>> {
    let bytes: Vec<u8> = decode(byte_str).unwrap();
    let arr: ndarray::Array2<u32> = deserialize(&bytes).unwrap();
    Ok(PyArray2::from_array(py, &arr))
}

#[pyfunction]
#[pyo3(name = "_decode_mask_u64")]
#[pyo3(text_signature = "(byte_str: str, /)")]
pub fn py_decode_mask_u64<'a>(py: Python<'a>, byte_str: String) -> PyResult<&'a PyArray2<u64>> {
    let bytes: Vec<u8> = decode(byte_str).unwrap();
    let arr: ndarray::Array2<u64> = deserialize(&bytes).unwrap();
    Ok(PyArray2::from_array(py, &arr))
}

#[pyfunction]
#[pyo3(name = "_decode_mask_i8")]
#[pyo3(text_signature = "(byte_str: str, /)")]
pub fn py_decode_mask_i8<'a>(py: Python<'a>, byte_str: String) -> PyResult<&'a PyArray2<i8>> {
    let bytes: Vec<u8> = decode(byte_str).unwrap();
    let arr: ndarray::Array2<i8> = deserialize(&bytes).unwrap();
    Ok(PyArray2::from_array(py, &arr))
}

#[pyfunction]
#[pyo3(name = "_decode_mask_i16")]
#[pyo3(text_signature = "(byte_str: str, /)")]
pub fn py_decode_mask_i16<'a>(py: Python<'a>, byte_str: String) -> PyResult<&'a PyArray2<i16>> {
    let bytes: Vec<u8> = decode(byte_str).unwrap();
    let arr: ndarray::Array2<i16> = deserialize(&bytes).unwrap();
    Ok(PyArray2::from_array(py, &arr))
}

#[pyfunction]
#[pyo3(name = "_decode_mask_i32")]
#[pyo3(text_signature = "(byte_str: str, /)")]
pub fn py_decode_mask_i32<'a>(py: Python<'a>, byte_str: String) -> PyResult<&'a PyArray2<i32>> {
    let bytes: Vec<u8> = decode(byte_str).unwrap();
    let arr: ndarray::Array2<i32> = deserialize(&bytes).unwrap();
    Ok(PyArray2::from_array(py, &arr))
}

#[pyfunction]
#[pyo3(name = "_decode_mask_i64")]
#[pyo3(text_signature = "(byte_str: str, /)")]
pub fn py_decode_mask_i64<'a>(py: Python<'a>, byte_str: String) -> PyResult<&'a PyArray2<i64>> {
    let bytes: Vec<u8> = decode(byte_str).unwrap();
    let arr: ndarray::Array2<i64> = deserialize(&bytes).unwrap();
    Ok(PyArray2::from_array(py, &arr))
}


#[pyfunction]
#[pyo3(name = "_decode_mask_f32")]
#[pyo3(text_signature = "(byte_str: str, /)")]
pub fn py_decode_mask_f32<'a>(py: Python<'a>, byte_str: String) -> PyResult<&'a PyArray2<f32>> {
    let bytes: Vec<u8> = decode(byte_str).unwrap();
    let arr: ndarray::Array2<f32> = deserialize(&bytes).unwrap();
    Ok(PyArray2::from_array(py, &arr))
}

#[pyfunction]
#[pyo3(name = "_decode_mask_f64")]
#[pyo3(text_signature = "(byte_str: str, /)")]
pub fn py_decode_mask_f64<'a>(py: Python<'a>, byte_str: String) -> PyResult<&'a PyArray2<f64>> {
    let bytes: Vec<u8> = decode(byte_str).unwrap();
    let arr: ndarray::Array2<f64> = deserialize(&bytes).unwrap();
    Ok(PyArray2::from_array(py, &arr))
}