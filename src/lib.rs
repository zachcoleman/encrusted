use pyo3::prelude::*;

mod codegen;
mod decode;
mod dispatch;
mod encode;

#[pymodule]
fn _encrusted_ext(_py: Python, m: &PyModule) -> PyResult<()> {
    // encode can dispatch to one type
    m.add_function(wrap_pyfunction!(encode::py_encode_mask, m)?)?;

    // export all the decodes
    make_module_exports!(decode_1, m);
    make_module_exports!(decode_2, m);
    make_module_exports!(decode_3, m);
    make_module_exports!(decode_4, m);
    make_module_exports!(decode_dyn, m);

    Ok(())
}
