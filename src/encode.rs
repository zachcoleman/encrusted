use numpy::*;
use pyo3::prelude::*;

use flate2::write::DeflateEncoder;
use flate2::Compression;
use std::io::prelude::*;

use crate::numpy_dispatch;

#[pyfunction]
#[pyo3(name = "_encode_mask")]
#[pyo3(text_signature = "(arr: np.ndarray, /)")]
pub fn py_encode_mask<'a>(py: Python<'a>, arr: &'a PyAny) -> PyResult<String> {
    numpy_dispatch!(py, encode_mask, PyResult<String>, arr)
}

fn encode_mask<T>(_py: Python, arr: PyReadonlyArray2<T>) -> PyResult<String>
where
    T: Copy + Clone + numpy::Element + serde::ser::Serialize,
{
    let bytes = bincode::serialize(&arr.to_owned_array()).unwrap();
    let mut e = DeflateEncoder::new(Vec::new(), Compression::fast());
    e.write_all(&bytes[..]).unwrap(); 
    Ok(base64::encode(e.finish().unwrap()))
    // Ok(base64::encode(bytes))
}
