use std::fs;
use std::io::{BufReader, Read};
use std::io::{Seek, SeekFrom};
use std::mem;
/// Report D88 File
use std::path::Path;

use crate::format::D88_Header;

use crate::disk::Disk;

///
///
///
#[derive(Default)]
#[allow(non_snake_case)]
pub struct D88FileIO {
    pub reader: Option<BufReader<std::fs::File>>,
    pub disk: Disk,
}

#[allow(non_snake_case)]
impl D88FileIO {
    /// Constructor
    ///
    /// Constructor
    ///
    /// # Argument
    ///
    ///   * (none)
    ///
    /// # Return
    ///
    ///   * D88FileIO
    ///
    fn _open<P: AsRef<Path>>(path: P) -> Result<BufReader<std::fs::File>, ()> {
        if let Ok(fh) = fs::File::open(path) {
            Ok(BufReader::new(fh))
        } else {
            Err(())
        }
    }

    /// Read Disk Paramater
    ///
    /// # Argument
    ///
    ///   * `reader` &mut BufReader<std::fs::File>
    ///
    /// # Return
    ///
    ///   * (none)
    ///
    fn _read_disk_parameter(reader: &mut BufReader<std::fs::File>) -> Result<Disk, ()> {
        let mut disk = Disk::default();
        if let Ok(_) = disk.preset(reader) {
            Ok(disk)
        } else {
            Err(())
        }
    }

    /// D88ファイル指定つきConstructor
    ///
    /// Constructor with *.d88 File Open
    ///
    /// # Argument
    ///
    ///   * `path` d88 File Path
    ///
    /// # Return
    ///
    ///   * D88FileIO
    ///
    pub fn open<P: AsRef<Path>>(path: P) -> Self {
        if let Ok(mut reader) = D88FileIO::_open(path) {
            if let Ok(disk) = D88FileIO::_read_disk_parameter(&mut reader) {
                return Self {
                    reader: Some(reader),
                    disk: disk,
                };
            }
        }

        Self {
            reader: None,
            disk: Disk::default(),
        }
    }

    /// Read D88 Header (Helper function)
    ///
    /// File Open Check
    ///
    /// # Argument
    ///
    ///   * (none)
    ///
    /// # Return
    ///
    ///   * `true` : File Opend
    ///   * `false`: File Not Open
    ///
    pub fn is_open(&self) -> bool {
        !self.reader.is_none()
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
        //reader: &mut BufReader<std::fs::File>,
    ) -> Result<D88_Header, ()> {
        let mut buf: [u8; mem::size_of::<D88_Header>()] = [0; mem::size_of::<D88_Header>()]; // Header Buffer

        if let Some(ref mut reader) = self.reader {
            if reader.seek(SeekFrom::Start(0)).is_err() {
                return Err(());
            }

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
} //

// ================================================================================
//
//  Test Code
//
// ================================================================================
#[cfg(test)]
mod test {
    use crate::fileio::D88FileIO;
    //use crate::format::{D88_Header, D88_SectorHdr};

    #[test]
    fn test_read_d88_header_disk_name() {
        let mut d88fileio = D88FileIO::open("../../sample/HuBASIC_Format_2D.d88");

        if let Ok(hdr) = d88fileio.read_d88_header() {
            assert_eq!(
                hdr.disk_name,
                [
                    0x62, 0x79, 0x5f, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x5f, 0x4f, 0x52, 0x59,
                    0x5a, 0x41, 0x50, 0x41 /*, 0x4f*/
                ]
            );
        }
    }

    #[test]
    fn test_read_d88_header_write_protect() {
        let mut d88fileio = D88FileIO::open("../../sample/HuBASIC_Format_2D.d88");

        if let Ok(hdr) = d88fileio.read_d88_header() {
            assert_eq!(hdr.write_protect, 0x00); // "Protected"
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_read_d88_header_disk_type() {
        let mut d88fileio = D88FileIO::open("../../sample/HuBASIC_Format_2D.d88");

        if let Ok(hdr) = d88fileio.read_d88_header() {
            assert_eq!(hdr.disk_type, 0x00); // "2D"
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_read_d88_header_disk_size() {
        let mut d88fileio = D88FileIO::open("../../sample/HuBASIC_Format_2D.d88");

        if let Ok(hdr) = d88fileio.read_d88_header() {
            assert_eq!(hdr.disk_size, 348848); // 348848
        } else {
            assert!(false);
        }
    }
}
