/// Report D88 File
use std::io::{BufReader, Read};
use std::io::{Seek, SeekFrom};
use std::mem;

use crate::format::{D88_Header, D88_SectorHdr};

///
///
///
#[allow(dead_code)]
pub struct D88FileIO;

impl D88FileIO {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self
    }

    /// Read D88 Header (Helper function)
    ///
    /// D88ファイルのヘッダ情報を返す
    ///
    /// # Argument
    ///
    ///   * `reader` BufferReader instance at D88 File
    ///
    /// # Return
    ///
    ///   * Result<D88_Header, ()>
    ///
    #[allow(dead_code)]
    pub fn read_d88_header(
        &mut self,
        reader: &mut BufReader<std::fs::File>,
    ) -> Result<D88_Header, ()> {
        let mut buf: [u8; mem::size_of::<D88_Header>()] = [0; mem::size_of::<D88_Header>()]; // Header Buffer

        //if let Ok(_) = reader.seek(SeekFrom::Start(0)) {
        if reader.seek(SeekFrom::Start(0)).is_ok() {
            if let Ok(read_size) = reader.read(&mut buf) {
                if read_size != mem::size_of::<D88_Header>() {
                    return Err(());
                }
                unsafe {
                    return Ok(mem::transmute::<
                        [u8; mem::size_of::<D88_Header>()],
                        D88_Header,
                    >(buf));
                }
            }
        }
        Err(())
    }

    /// Read Sector Header (Helper function)
    ///
    /// セクタのヘッダ情報を返す
    ///
    /// # Argument
    ///
    ///   * `reader` BufferReader instance at D88 File
    ///
    /// # Return
    ///
    ///   * Result<D88_SectirHdr, ()>
    ///
    #[allow(dead_code)]
    pub fn read_sector_header(
        &mut self,
        reader: &mut BufReader<std::fs::File>,
    ) -> Result<D88_SectorHdr, ()> {
        let mut buf: [u8; mem::size_of::<D88_SectorHdr>()] = [0; mem::size_of::<D88_SectorHdr>()]; // Header Buffer

        if let Ok(read_size) = reader.read(&mut buf) {
            if read_size != mem::size_of::<D88_SectorHdr>() {
                return Err(());
            }
            unsafe {
                Ok(mem::transmute::<
                    [u8; mem::size_of::<D88_SectorHdr>()],
                    D88_SectorHdr,
                >(buf))
            }
        } else {
            Err(())
        }
    }
} //

#[cfg(test)]
mod test {
    #[test]
    fn test_read_d88_header_00() {
        assert!(true);
    }
}
