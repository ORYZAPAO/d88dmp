use ansi_term::Color;
use std::mem;
use std::path::Path;

use ::D88FileIO::disk::{Sector, Track};
use ::D88FileIO::format::{D88_Header, D88_SectorHdr};
use D88FileIO::fileio::D88FileIO;

use crate::utility::{get_str_to_u8, ERROR};

/// ReportD88
///
/// D88ファイル情報を表示。
///
#[derive(Clone, Copy)]
pub struct Position {
    track: u8,
    side: u8,
    sector: u8,
}

pub struct ReportD88 {
    //cmdline_info: clap::ArgMatches,
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

    /// Constructor
    ///
    pub fn new(_cmdline_info: clap::ArgMatches) -> Self {
        let _path = if let Some(path) = _cmdline_info.value_of("*.D88") {
            Some(path.to_string())
        } else {
            None
        };

        let _noinfo_flg: bool = _cmdline_info.is_present("no-info");
        let _nocolor_flg: bool = _cmdline_info.is_present("no-color");
        let _sort_by_sector: bool = _cmdline_info.is_present("Sort by Sector Order");

        let _position = if let Some(pos) = _cmdline_info.value_of("TRACK,SIDE,SECTOR") {
            let pos_str: Vec<&str> = pos.split(',').collect();

            let mut track: Result<u8,()>  = get_str_to_u8(pos_str[0], "Not Track Number");
            let mut side: Result<u8,()>   = get_str_to_u8(pos_str[1], "Not Side Number");
            let mut sector: Result<u8,()> = get_str_to_u8(pos_str[2], "Not Sector Number");
            Some(Position {
                track: track.unwrap(),
                side: side.unwrap(),
                sector: sector.unwrap(),
            })
        } else {
            None
        };

        Self {
            path: _path,
            //cmdline_info: _cmdline_info,
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
        let _ = self.report_d88_header();

        for track in self.d88fileio.disk.track_tbl.iter() {
            self.report_track(track);
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
        self.print_track_bar();
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

        self.print_title_bar();
        self.print_16byte(&byte_img, 0x0000_u64, ansi_term::Color::Green);

        // Report File Header
        //
        let disk_name;
        unsafe {
            disk_name = String::from_utf8_unchecked(header.disk_name.to_vec());
        }
        print!("Name({}), ", disk_name);
        print!(
            "WriteProtect({}), ",
            match header.write_protect {
                0x10 => "Protected",
                0x00 => "None",
                _ => "!! Illegal !!",
            }
        );
        print!(
            "Type({} Disk), ",
            match header.disk_type {
                0x00 => "2D",
                0x10 => "2DD",
                0x20 => "2HD",
                _ => "??",
            }
        );
        print!("DiskSize({} byte)", header.disk_size);
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
            "Track({}), Side({}), Sector({}), ",
            sector.header.track, sector.header.side, sector.header.sector
        );

        print!("Size({} byte/sec), ", 128 << (sector.header.sector_size));
        print!("NumOfSec({} sec/track), ", sector.header.number_of_sec);
        print!(
            "Status({}), ",
            match sector.header.status {
                0x00 => "OK",           // 正常
                0x10 => "DELETED",      // 削除済みデータ
                0xa0 => "ID CRC Err",   // ID CRC エラー
                0xb0 => "Data CRC Err", // データ CRC エラー
                0xe0 => "No Addr Mark", // アドレスマークなし
                0xf0 => "No Data Mark", // データマークなし
                _ => "??",
            }
        );
        print!(
            "Density({:02x}h), ",
            //match sector.header.density {
            //    0x00 => "D",  // 倍密度 // dencityの仕様がよく分からない。
            //    0x40 => "S",  // 単密度
            //    0x01 => "HD", // 高密度
            //    _ => "??",
            //}
            sector.header.density
        );
        print!(
            "Mark({}), ",
            match sector.header.deleted_mark {
                0x00 => "NORMAL",
                0x10 => "DELETED",
                _ => "??",
            }
        );
        print!("DataSize({} byte), ", sector.header.size_of_data);

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
