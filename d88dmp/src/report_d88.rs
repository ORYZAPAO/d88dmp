use ansi_term::Color;
use std::mem;
use std::path::Path;

use ::D88FileIO::format::{D88_Header, D88_SectorHdr};
use ::D88FileIO::sector::Sector;
use ::D88FileIO::track::Track;
use D88FileIO::fileio::D88FileIO;

use crate::cli::get_str_to_u8;

/// ReportD88
///
/// D88ファイル情報を表示。
///
pub struct Position {
    track: u8,
    side: u8,
    sector: u8,
}

pub struct ReportD88 {
    pub path: Option<String>,
    pub noinfo_flg: bool,
    pub nocolor_flg: bool,
    pub sort_by_sector: bool,
    pub position: Option<Position>,
    pub d88fileio: D88FileIO,
}

impl ReportD88 {
    /// Constructor
    ///
    pub fn new(_cmdline_info: clap::ArgMatches) -> Self {
        // Get Command Line Option
        //
        let _path = if let Some(path) = _cmdline_info.value_of("*.D88") {
            Some(path.to_string())
        } else {
            None
        };

        let _noinfo_flg: bool = _cmdline_info.is_present("no-info");
        let _nocolor_flg: bool = _cmdline_info.is_present("no-color");
        let mut _sort_by_sector: bool = _cmdline_info.is_present("Sort by Disk Sector Order");

        let _position = if let Some(pos) = _cmdline_info.value_of("TRACK,SIDE,SECTOR") {
            let pos_str: Vec<&str> = pos.split(',').collect();

            let track: Result<u8, ()> = get_str_to_u8(pos_str[0], "Not Track Number");
            let side: Result<u8, ()> = get_str_to_u8(pos_str[1], "Not Side Number");
            let sector: Result<u8, ()> = get_str_to_u8(pos_str[2], "Not Sector Number");

            // Sort by Sector Order
            _sort_by_sector = true;

            Some(Position {
                track: track.unwrap(),
                side: side.unwrap(),
                sector: (sector.unwrap() - 1),
            })
        } else {
            None
        };

        //
        //
        Self {
            path: _path,
            noinfo_flg: _noinfo_flg,
            nocolor_flg: _nocolor_flg,
            sort_by_sector: _sort_by_sector,
            position: _position,
            d88fileio: D88FileIO::default(),
        }
    }

    /// Report
    ///
    /// # Argument
    ///   * (none)
    ///
    pub fn report(&mut self) {
        if let Some(ref d88_path) = self.path {
            if self.noinfo_flg {
                if let Ok(fh) = std::fs::File::open(Path::new(d88_path)) {
                    self.report_d88_noinfo(fh);
                } else {
                    println!("File Not Found \"{}\"", d88_path);
                }
            } else {
                self.d88fileio = D88FileIO::open(Path::new(d88_path));
                if self.d88fileio.is_open() {
                    //
                    if self.sort_by_sector || self.position.is_some() {
                        self.d88fileio.sector_sort();
                    }

                    //
                    self.report_d88();
                } else {
                    println!("File Not Found \"{}\"", d88_path);
                }
            }
        }
    }

    /// Report D88 File
    ///
    /// D88ファイル情報を表示する。
    ///
    /// # Argument
    ///
    ///  * `reader` BufferReader Instance at D88 File
    ///
    /// # Return
    ///  * (none)
    ///
    pub fn report_d88(&self) {
        if let Some(ref position) = self.position {
            // Report One Sector
            let sector = &self.d88fileio.disk.track_tbl
                [((position.track * 2) + position.side) as usize]
                .sector_tbl[position.sector as usize];

            self.print_offset_bar();
            self.report_sector(sector);
        } else {
            // Report All
            let _ = self.report_d88_header();
            for track in self.d88fileio.disk.track_tbl.iter() {
                self.report_track(track);
            }
        }
    }

    pub fn report_track(&self, track: &Track) {
        for sector in track.sector_tbl.iter() {
            self.report_sector(sector);
        }
    }

    pub fn report_sector(&self, sector: &Sector) {
        self.print_sector(sector);
    }

    /// Report D88 Header (Helper function)
    ///
    /// `report_d88` から呼び出される内部関数。  
    /// D88ファイルのヘッダ情報を表示する
    ///
    /// # Argument
    ///
    ///   * `reader` BufferReader instance at D88 File
    ///
    /// # Return
    ///
    ///   * if Success, return D88 Header Size.  
    ///   * if Error, return 0
    ///
    #[allow(clippy::format_in_format_args)]
    pub fn report_d88_header(&self) -> usize {
        // Get File Header
        //
        let header = &(self.d88fileio.disk.header);

        // Output Track Table
        //
        self.print_track_offset_bar();
        for n in 0..164 {
            if (n % 8) == 0 {
                println!();

                if self.nocolor_flg {
                    print!(
                        "{}  {:05x} ",
                        (format!("{0:2x}h {0:3}d", n)),
                        header.track_tbl[n]
                    );
                } else {
                    print!(
                        "{}  {:05x} ",
                        Color::Cyan.paint(format!("{0:2x}h {0:3}d", n)),
                        header.track_tbl[n]
                    );
                }
            } else {
                print!("{:05x} ", header.track_tbl[n]);
            }
        }

        // Report File Header (Byte Image)
        //
        println!();
        println!();
        let byte_img;
        #[allow(clippy::clone_on_copy)]
        unsafe {
            byte_img =
                mem::transmute::<D88_Header, [u8; mem::size_of::<D88_Header>()]>((*header).clone());
        }

        self.print_offset_bar();
        self.print_16byte(&byte_img, 0x0000_u64, ansi_term::Color::Green);

        // Report File Header
        //
        print!(
            "{}, {}, {}, {}",
            self.d88fileio.disk.get_disk_name(),
            self.d88fileio.disk.get_disk_write_protect(),
            self.d88fileio.disk.get_disk_type(),
            self.d88fileio.disk.get_disk_size()
        );
        println!();

        mem::size_of::<D88_Header>()
    }

    /// Report Sector (Helper function)
    ///
    /// `report_d88` から呼び出される内部関数。  
    /// セクタを表示する。  
    ///
    /// # Arguments
    ///   * `reader` BufferReader instance at D88 File
    ///   * `offset_` Offset to a Sector at D88 Disk File
    ///
    /// # Return
    ///   * if Success, return next offset at D88 File
    ///   * if Error, retrun 0
    ///
    pub fn print_sector(&self, sector: &Sector) {
        let byte_img;

        // Report Sector Header (Byte Image)
        //
        #[allow(clippy::clone_on_copy)]
        unsafe {
            byte_img = mem::transmute::<D88_SectorHdr, [u8; mem::size_of::<D88_SectorHdr>()]>(
                sector.header.clone(),
            );
        }
        self.print_16byte(
            &byte_img,
            sector.offset - mem::size_of::<D88_SectorHdr>() as u64,
            ansi_term::Color::Green,
        );

        // Print Sector Header
        //
        print!(
            "{}, {}, {}, {}, {}, {}, {}, {}, {}",
            sector.get_track(),
            sector.get_side(),
            sector.get_sector(),
            sector.get_sector_size(),
            sector.get_num_of_sector(),
            sector.get_status(),
            sector.get_density(),
            sector.get_mark(),
            sector.get_data_size(),
        );
        println!();

        // Print Sector Raw Data
        //
        let mut ct = sector.header.size_of_data;
        let mut offset = sector.offset;
        let mut pt = 0;
        while ct > 0 {
            let aa = &sector.data[pt..pt + 16];
            self.print_16byte(aa, offset, ansi_term::Color::White);
            println!();
            offset += 16;
            pt += 16;
            ct -= 16;
        }
    }
}
