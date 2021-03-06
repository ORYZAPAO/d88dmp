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
    pub summary_only_flg: bool,
    pub sort_by_sector: bool,
    pub verbose_flg: bool,

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
        let _summary_only_flg: bool = _cmdline_info.is_present("summary");
        let _verbose_flg: bool = _cmdline_info.is_present("verbose");
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
            summary_only_flg: _summary_only_flg,
            verbose_flg: _verbose_flg,
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
            // Summary
            if self.summary_only_flg || self.verbose_flg {
                let _ = self.report_d88_summary();
            }

            // Byte Image
            if self.summary_only_flg {
                return;
            }
            println!();
            println!();
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

    /// Report D88 Summary (Helper function)
    ///
    /// `report_d88` から呼び出される内部関数。  
    /// D88ファイルのサマリ情報を表示する
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
    pub fn report_d88_summary(&self) -> usize {
        // Get File Header
        //
        let header = &(self.d88fileio.disk.header);

        // ----------------------------------------
        // Report File Header Summary
        // ----------------------------------------
        self.print_d88_file_header_title_bar();
        println!("  {}", self.d88fileio.disk.get_disk_name());
        println!("  Protect{}", self.d88fileio.disk.get_disk_write_protect());
        println!("  {}", self.d88fileio.disk.get_disk_type());
        println!("  {}", self.d88fileio.disk.get_disk_size());
        println!();
        println!();

        // ----------------------------------------
        // Report Track Offset Table
        // ----------------------------------------
        self.print_track_offset_table_bar();
        for n in 0..164 {
            // format
            let track_num_formated = format!("{0:2x}h {0:3}d", n);
            let track_offset_formated = if header.track_offset_tbl[n] == 0 {
                "------".to_string()
            } else {
                format!("{:06x}", header.track_offset_tbl[n])
            };

            //
            if (n % 8) == 0 {
                println!();

                if self.nocolor_flg {
                    print!("{}  {} ", track_num_formated, track_offset_formated);
                } else {
                    print!(
                        "{}  {} ",
                        Color::Cyan.paint(track_num_formated),
                        track_offset_formated
                    );
                }
            } else {
                print!("{} ", track_offset_formated);
            }
        }
        println!();

        // ----------------------------------------
        // Report All Sector Summary
        // ----------------------------------------
        println!();
        println!();

        self.print_sector_summary_bar();

        for track in self.d88fileio.disk.track_tbl.iter() {
            for (sector_ct, sector) in track.sector_tbl.iter().enumerate() {
                //
                let tso_formated = if sector_ct == 0 {
                    format!(
                        "{0:02x}h {0:3}d {1:3}  {2:3} {3:3} ",
                        sector.header.track,
                        sector.header.side,
                        sector.header.sector,
                        track.number_of_sector
                    )
                } else {
                    format!("... .... ...  {:3} ... ", sector.header.sector)
                };

                if !self.nocolor_flg {
                    print!("{} ", Color::Cyan.paint(&tso_formated));
                } else {
                    print!("{} ", tso_formated);
                }

                //
                let offset_formated = format!("{:06x}h ", sector.offset);
                if !self.nocolor_flg {
                    print!("{} ", Color::Cyan.paint(offset_formated));
                } else {
                    print!("{} ", offset_formated);
                }

                println!(
                    "{}, {}, {}, {}, {}, {}",
                    sector.get_sector_size(),
                    sector.get_num_of_sector(),
                    sector.get_status(),
                    sector.get_density(),
                    sector.get_mark(),
                    sector.get_data_size(),
                );
            } // for sector in track.sector_tbl.iter() {
        } // for track in self.d88fileio.disk.track_tbl.iter() {

        // ----------------------------------------
        mem::size_of::<D88_Header>()
    }

    /// Report D88 Summary (Helper function)
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

        // ----------------------------------------
        // [ByteImage] D88 File Header
        //   16byte
        // ----------------------------------------
        let byte_img;
        #[allow(clippy::clone_on_copy)]
        unsafe {
            byte_img =
                mem::transmute::<D88_Header, [u8; mem::size_of::<D88_Header>()]>((*header).clone());
        }
        self.print_offset_bar();

        self.print_16byte(&byte_img, 0x00000_u64, ansi_term::Color::Green); /////////   0 - 16 byte
        print!("{}", self.d88fileio.disk.get_disk_name());
        println!();

        self.print_16byte(&byte_img[16..], 0x00010_u64, ansi_term::Color::Green); //   16 - 31 byte
        print!(
            "{}, {}, {}",
            self.d88fileio.disk.get_disk_write_protect(),
            self.d88fileio.disk.get_disk_type(),
            self.d88fileio.disk.get_disk_size(),
        );
        println!();

        // ----------------------------------------
        // [ByteImage] Offset to Track
        //   32bit(4byte) x 164
        // ----------------------------------------
        let mut offset = 0x00020_u64;
        let mut buf32x4 = [0u32; 4];
        for (ct, track_offset) in header.track_offset_tbl.iter().enumerate() {
            buf32x4[ct % 4] = *track_offset;
            if (ct % 4) == 3 {
                unsafe {
                    let buf8x16 = mem::transmute::<[u32; 4], [u8; 16]>(buf32x4); // 32bit x 4 --> 8bit x 16
                    self.print_16byte(&buf8x16, offset, ansi_term::Color::Yellow);
                }

                print!("TrackOffset ");
                for ofst in buf32x4.iter() {
                    let formated = if *ofst == 0 {
                        "------".to_string()
                    } else {
                        format!("{:06x}", ofst)
                    };
                    print!("{} ", formated);
                }
                println!();
                offset += 16;
            }
        }

        // ----------------------------------------
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
