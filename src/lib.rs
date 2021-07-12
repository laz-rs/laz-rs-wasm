use laz::las::file::{read_vlrs_and_get_laszip_vlr, QuickHeader};
use laz::las::laszip::LasZipDecompressor;
use wasm_bindgen::prelude::*;
use std::io::Seek;
extern crate console_error_panic_hook;

#[wasm_bindgen]
#[derive(Copy, Clone)]
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
pub struct WasmLasZipDecompressor {
    decompressor: LasZipDecompressor<'static, std::io::Cursor<Vec<u8>>>,
    pub header: WasmQuickHeader
}

#[wasm_bindgen]
impl WasmLasZipDecompressor {
    #[wasm_bindgen(constructor)]
    pub fn new(buf: js_sys::Uint8Array) -> Self {      
        console_error_panic_hook::set_once();
          
        let mut cursor = std::io::Cursor::new(buf.to_vec());

        let hdr = QuickHeader::read_from(&mut cursor).unwrap_throw();
        
        cursor.seek(std::io::SeekFrom::Start(hdr.header_size as u64)).unwrap_throw();
        let laz_vlr = read_vlrs_and_get_laszip_vlr(&mut cursor, &hdr);

        cursor.seek(std::io::SeekFrom::Start(hdr.offset_to_points as u64)).unwrap_throw();
        let decomp = LasZipDecompressor::new(cursor, laz_vlr.expect("Compressed data, but no Laszip Vlr found")).unwrap_throw();

        Self {
            decompressor: decomp,
            header: WasmQuickHeader::from(hdr)
        }
    }

    pub fn decompress_many(&mut self, out: &mut [u8]) -> Result<(), JsValue> {
        self.decompressor.decompress_many(out).unwrap_throw();
        Ok(())
    }
}
