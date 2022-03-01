use std::io::{BufReader, Read};
use std::io::{Seek, SeekFrom};
use std::mem;

use crate::format::D88_SectorHdr;
///

#[derive(Default, Debug)]
pub struct Sector {
    pub offset: u64,
    pub header: D88_SectorHdr,
    pub data: Vec<u8>,
}
impl Sector {
    /// Read Sector
    ///
    /// # Argument
    ///
    ///   * `reader` &mut BufReader<std::fs::File>
    ///
    /// # Return
    ///
    ///   * Ok(usize)  Sector Data Size (with Sector Header)
    ///   * Err(())    
    ///
    #[allow(clippy::result_unit_err)]
    pub fn preset(
        &mut self,
        reader: &mut BufReader<std::fs::File>,
        offset: u64,
    ) -> Result<u64, ()> {
        if offset == 0 {
            return Err(());
        }

        if reader.seek(SeekFrom::Start(offset)).is_err() {
            return Err(());
        }

        let mut buf: [u8; mem::size_of::<D88_SectorHdr>()] = [0; mem::size_of::<D88_SectorHdr>()]; // Header Buffer
        if let Ok(read_size) = reader.read(&mut buf) {
            if read_size != mem::size_of::<D88_SectorHdr>() {
                return Err(());
            }

            let d88_sector_header;
            unsafe {
                d88_sector_header =
                    mem::transmute::<[u8; mem::size_of::<D88_SectorHdr>()], D88_SectorHdr>(buf);
            }
            let sector_offset = offset + mem::size_of::<D88_SectorHdr>() as u64;

            let ret_sector_size =
                mem::size_of::<D88_SectorHdr>() + ((128 << d88_sector_header.sector_size) as usize);

            let mut sector_data: Vec<u8> = vec![0; d88_sector_header.size_of_data.into()];
            if reader.seek(SeekFrom::Start(sector_offset)).is_err() {
                return Err(());
            }
            if reader.read(&mut sector_data).is_err() {
                return Err(());
            }

            //
            self.offset = sector_offset;
            self.header = d88_sector_header;
            self.data = sector_data;

            Ok(ret_sector_size as u64)
        } else {
            Err(())
        }
    }

    ///
    ///
    pub fn get_track(&self) -> String {
        format!("Track({})", self.header.track)
    }

    pub fn get_side(&self) -> String {
        format!("Side({})", self.header.side)
    }

    pub fn get_sector(&self) -> String {
        format!("Sector({})", self.header.sector)
    }

    pub fn get_sector_size(&self) -> String {
        format!("Size({} byte/sec)", 128 << (self.header.sector_size))
    }

    pub fn get_num_of_sector(&self) -> String {
        format!("NumOfSector({} sec/track)", self.header.number_of_sec)
    }

    pub fn get_status(&self) -> String {
        format!(
            "Status({})",
            match self.header.status {
                0x00 => "OK",           // 正常
                0x10 => "DELETED",      // 削除済みデータ
                0xa0 => "ID CRC Err",   // ID CRC エラー
                0xb0 => "Data CRC Err", // データ CRC エラー
                0xe0 => "No Addr Mark", // アドレスマークなし
                0xf0 => "No Data Mark", // データマークなし
                _ => "??",
            }
        )
    }

    pub fn get_density(&self) -> String {
        format!(
            "Density({})",
            match self.header.density {
                0x00 => "D",  // 倍密度 // dencityの仕様がよく分からない。
                0x40 => "S",  // 単密度
                0x01 => "HD", // 高密度
                _ => "??",
            } //"Density({:02x}h), ",
              //self.header.density
        )
    }

    pub fn get_mark(&self) -> String {
        format!(
            "Mark({})",
            match self.header.deleted_mark {
                0x00 => "NORMAL",
                0x10 => "DELETED",
                _ => "??",
            }
        )
    }

    pub fn get_data_size(&self) -> String {
        format!("DataSize({} byte), ", self.header.size_of_data)
    }
}
