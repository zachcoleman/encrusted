use pyo3::prelude::*;

mod encode;
mod decode;
mod dispatch;

#[pymodule]
fn _encrusted_ext(_py: Python, m: &PyModule) -> PyResult<()> {
    // encode can dispatch to one type
    m.add_function(wrap_pyfunction!(encode::py_encode_mask, m)?)?;

    // export all the decodes
    m.add_function(wrap_pyfunction!(decode::py_decode_mask_bool, m)?)?;
    m.add_function(wrap_pyfunction!(decode::py_decode_mask_u8, m)?)?;
    m.add_function(wrap_pyfunction!(decode::py_decode_mask_u16, m)?)?;
    m.add_function(wrap_pyfunction!(decode::py_decode_mask_u32, m)?)?;
    m.add_function(wrap_pyfunction!(decode::py_decode_mask_u64, m)?)?;
    m.add_function(wrap_pyfunction!(decode::py_decode_mask_i8, m)?)?;
    m.add_function(wrap_pyfunction!(decode::py_decode_mask_i16, m)?)?;
    m.add_function(wrap_pyfunction!(decode::py_decode_mask_i32, m)?)?;
    m.add_function(wrap_pyfunction!(decode::py_decode_mask_i64, m)?)?;
    m.add_function(wrap_pyfunction!(decode::py_decode_mask_f32, m)?)?;
    m.add_function(wrap_pyfunction!(decode::py_decode_mask_f64, m)?)?;

    Ok(())
}
