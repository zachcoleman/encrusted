use pyo3::prelude::*;

mod codegen;
mod decode;
mod dispatch;
mod encode;

#[pymodule]
fn _encrusted_ext(_py: Python, m: &PyModule) -> PyResult<()> {
    // export all the decodes
    make_decode_exports!(decode_1, m);
    make_decode_exports!(decode_2, m);
    make_decode_exports!(decode_3, m);
    make_decode_exports!(decode_4, m);
    make_decode_exports!(decode_dyn, m);

    // export the encodes
    make_encode_exports!(encode_1, m);
    make_encode_exports!(encode_2, m);
    make_encode_exports!(encode_3, m);
    make_encode_exports!(encode_4, m);
    make_encode_exports!(encode_dyn, m);

    Ok(())
}
