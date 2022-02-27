use std::io::BufReader;

use crate::format::MAX_SECTOR;
use crate::sector::Sector;

///
#[derive(Default, Debug)]
pub struct Track {
    pub number_of_sector: u16,
    pub sector_tbl: Vec<Sector>,
}

impl Track {
    /// Read Track
    ///
    /// # Argument
    ///
    ///   * `reader` &mut BufReader<std::fs::File>
    ///
    /// # Return
    ///
    ///   * Ok(usize)  Track Data Size
    ///   * Err(())    
    ///
    #[allow(clippy::result_unit_err)]
    pub fn preset(
        &mut self,
        reader: &mut BufReader<std::fs::File>,
        offset_: u64,
    ) -> Result<usize, ()> {
        if offset_ == 0 {
            return Err(());
        }
        let mut offset = offset_;

        let mut sec_count: u16 = 0;
        let mut track_size: usize = 0;

        #[allow(unused_assignments)]
        let mut number_of_sector: u16 = 0;

        loop {
            let mut sector = Sector::default();

            if let Ok(sec_size) = sector.preset(reader, offset) {
                track_size += sec_size as usize;

                number_of_sector = sector.header.number_of_sec;

                self.sector_tbl.push(sector);

                sec_count += 1;
                if (sec_count >= number_of_sector) || (sec_count >= MAX_SECTOR) {
                    break;
                }

                //
                offset += sec_size;
            } else {
                return Err(());
            }
        }

        self.number_of_sector = number_of_sector;
        Ok(track_size)
    }

    /// Sector Table Sort by Sector Order
    ///
    /// # Argument
    ///
    ///   * (none)
    ///
    /// # Return
    ///
    ///   * (none)
    ///
    pub fn sector_sort(&mut self) {
        // Sort Sector Table
        self.sector_tbl
            .sort_by(|a: &Sector, b: &Sector| a.header.sector.cmp(&b.header.sector));
    }

    /// Sector Table Sort by File Offset Order
    ///
    /// # Argument
    ///
    ///   * (none)
    ///
    /// # Return
    ///
    ///   * (none)
    ///
    pub fn file_offset_sort(&mut self) {
        // Sort Sector Table
        self.sector_tbl
            .sort_by(|a: &Sector, b: &Sector| a.offset.cmp(&b.offset));
    }
}
