use std::io::{BufReader, Read};
use std::io::{Seek, SeekFrom};
use std::mem;

use crate::format::D88_Header;
use crate::track::Track;

///
#[derive(Default, Debug)]
pub struct Disk {
    pub header: D88_Header,
    pub track_tbl: Vec<Track>,
}

impl Disk {
    /// Read Disk
    ///
    /// # Argument
    ///
    ///   * `reader` &mut BufReader<std::fs::File>
    ///
    /// # Return
    ///
    ///   * Ok(usize)  Disk Size
    ///   * Err(())    
    ///
    #[allow(clippy::result_unit_err)]
    pub fn preset(&mut self, reader: &mut BufReader<std::fs::File>) -> Result<usize, ()> {
        if reader.seek(SeekFrom::Start(0)).is_err() {
            return Err(());
        }

        let mut buf: [u8; mem::size_of::<D88_Header>()] = [0; mem::size_of::<D88_Header>()]; // Header Buffer
        if let Ok(read_size) = reader.read(&mut buf) {
            //
            if read_size != mem::size_of::<D88_Header>() {
                return Err(());
            }

            let header;
            unsafe {
                header = mem::transmute::<[u8; mem::size_of::<D88_Header>()], D88_Header>(buf);
            }
            self.header = header;

            self.preset_track(reader) // return Ok(disk_size :usize)
        } else {
            Err(())
        }
        //
    }

    ///
    ///
    pub fn get_disk_name(&self) -> String {
        let disk_name;
        unsafe {
            disk_name = String::from_utf8_unchecked(self.header.disk_name.to_vec());
        }
        format!("Name({})", disk_name)
    }

    ///
    ///
    pub fn get_disk_write_protect(&self) -> String {
        format!(
            "({})",
            match self.header.write_protect {
                0x10 => "Protected",
                0x00 => "No Protected",
                _ => "!! Illegal !!",
            }
        )
    }

    ///
    ///
    pub fn get_disk_type(&self) -> String {
        format!(
            "Type({} Disk)",
            match self.header.disk_type {
                0x00 => "2D",
                0x10 => "2DD",
                0x20 => "2HD",
                _ => "??",
            }
        )
    }

    ///
    ///
    pub fn get_disk_size(&self) -> String {
        format!("DiskSize({} byte)", self.header.disk_size)
    }

    /// Read Track and Sector
    ///
    /// # Argument
    ///
    ///   * `reader` &mut BufReader<std::fs::File>
    ///
    /// # Return
    ///
    ///   * Ok(usize)  Disk Size
    ///   * Err(())    
    ///
    #[allow(clippy::result_unit_err)]
    pub fn preset_track(&mut self, reader: &mut BufReader<std::fs::File>) -> Result<usize, ()> {
        let mut disk_size: usize = 0;

        for track_offset in self.header.track_offset_tbl {
            let mut track = Track::default();

            if let Ok(track_size) = track.preset(reader, track_offset as u64) {
                disk_size += track_size as usize;
            } else {
                break;
            }
            self.track_tbl.push(track);
        }

        if disk_size == 0 {
            return Err(());
        }
        //assert_eq!(num_of_track, 80); // 2D Disk
        Ok(disk_size)
    }
}
