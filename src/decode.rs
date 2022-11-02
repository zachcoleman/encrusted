use base64::decode;
use bincode::deserialize;
use numpy::*;
use pyo3::prelude::*;

use flate2::write::DeflateDecoder;
use std::io::prelude::*;

use crate::make_decode_fns;

fn str_to_bytes(byte_str: String) -> Vec<u8> {
    let mut writer = Vec::new();
    let mut deflater = DeflateDecoder::new(writer);
    let bytes = decode(byte_str).unwrap();
    deflater.write_all(&bytes[..]).unwrap();
    writer = deflater.finish().unwrap();
    writer
}

make_decode_fns!(decode_1, numpy::PyArray1, ndarray::Array1);
make_decode_fns!(decode_2, numpy::PyArray2, ndarray::Array2);
make_decode_fns!(decode_3, numpy::PyArray3, ndarray::Array3);
make_decode_fns!(decode_4, numpy::PyArray4, ndarray::Array4);
make_decode_fns!(decode_dyn, numpy::PyArrayDyn, ndarray::ArrayD);
