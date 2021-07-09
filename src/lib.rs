use laz::las::file::{read_vlrs_and_get_laszip_vlr, QuickHeader};
use laz::las::laszip::LasZipDecompressor;
use wasm_bindgen::prelude::*;
use std::io::Seek;
extern crate console_error_panic_hook;

#[wasm_bindgen]
pub struct WasmQuickHeader {
    pub major: u8,
    pub minor: u8,
    pub offset_to_points: u32,
    pub num_vlrs: u32,
    pub point_format_id: u8,
    pub point_size: u16,
    pub num_points: u64,
    pub header_size: u16,
}

impl From<QuickHeader> for WasmQuickHeader {
    fn from(other: QuickHeader) -> WasmQuickHeader {
        WasmQuickHeader {
            major: other.major,
            minor: other.minor,
            offset_to_points: other.offset_to_points,
            num_vlrs: other.num_vlrs,
            point_format_id: other.point_format_id,
            point_size: other.point_size,
            num_points: other.num_points,
            header_size: other.header_size,
        }
    }
}

#[wasm_bindgen]
pub fn get_header(buf: js_sys::Uint8Array)  -> std::result::Result<WasmQuickHeader, JsValue> {
    // initialize debugging
    console_error_panic_hook::set_once();

    // copy header bytes into wasm memory
    let mut body = vec![0; buf.length() as usize];
    buf.copy_to(&mut body[..]);
    // cursor to wrap the bytes
    let mut cursor = std::io::Cursor::new(body);
    let hdr = QuickHeader::read_from(&mut cursor).unwrap();
    Ok(WasmQuickHeader::from(hdr))
}

#[wasm_bindgen]
pub struct WasmLasZipDecompressor {
    decompressor: LasZipDecompressor<'static, std::io::Cursor<Vec<u8>>>,
}

impl WasmLasZipDecompressor {
    pub fn new(source: Vec<u8>) -> Self {        
        let mut cursor = std::io::Cursor::new(source);

        let hdr = QuickHeader::read_from(&mut cursor).unwrap();
        
        cursor.seek(std::io::SeekFrom::Start(hdr.header_size as u64));
        let laz_vlr = read_vlrs_and_get_laszip_vlr(&mut cursor, &hdr);

        cursor.seek(std::io::SeekFrom::Start(hdr.offset_to_points as u64));
        let decomp = LasZipDecompressor::new(cursor, laz_vlr.expect("Compressed data, but no Laszip Vlr found")).unwrap();

        Self {
            decompressor: decomp,
        }
    }

    pub fn decompress_many(&mut self, out: &mut [u8]) -> std::io::Result<()> {
        Ok(self.decompressor.decompress_many(out)?)
    }
}

#[wasm_bindgen]
pub fn init_decompressor(buf: js_sys::Uint8Array)  -> WasmLasZipDecompressor  {
    WasmLasZipDecompressor::new(buf.to_vec())
}

#[wasm_bindgen]
pub fn decompress_many(decompressor: &mut WasmLasZipDecompressor, out: &mut [u8]) {
    decompressor.decompressor.decompress_many(out);
} 