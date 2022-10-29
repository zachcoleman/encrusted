use numpy::*;
use pyo3::prelude::*;
use pyo3::types::PyBytes;

// old JSON serde (slow)
// #[pyfunction]
// fn encode_mask(_py: Python, arr: PyReadonlyArray2<u8>) -> String{
//     json!({
//         "mask": arr.to_owned_array()
//     }).to_string()
// }

#[pyfunction]
fn encode_mask(py: Python, arr: PyReadonlyArray2<u8>) -> PyObject {
    let bytes = bincode::serialize(&arr.to_owned_array()).unwrap();
    PyBytes::new(py, &bytes).into()
}

#[pyfunction]
fn decode_mask<'a>(py: Python<'a>, bytes: &'a PyAny) -> PyResult<&'a PyArray2<u8>> {
    let bytes: Vec<u8> = bytes.extract::<Vec<u8>>().unwrap();
    let arr: ndarray::Array2<u8> = bincode::deserialize(&bytes).unwrap();
    Ok(PyArray2::from_array(py, &arr))
}

#[pymodule]
fn encrusted(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(encode_mask, m)?)?;
    m.add_function(wrap_pyfunction!(decode_mask, m)?)?;
    Ok(())
}
