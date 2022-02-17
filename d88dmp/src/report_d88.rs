use ansi_term::Color;
use std::io::Read;
use std::mem;

use ::D88FileIO::disk::{Sector, Track};
use ::D88FileIO::format::{D88_Header, D88_SectorHdr};
use D88FileIO::fileio::D88FileIO;

/// ReportD88
///
/// D88ファイル情報を表示。
///
pub struct ReportD88 {
    cmdline_info: clap::ArgMatches,
    pub noinfo_flg: bool,
    pub nocolor_flg: bool,
    pub d88fileio: D88FileIO,
}

impl ReportD88 {
    /// Constructor
    ///
    pub fn new(_cmdline_info: clap::ArgMatches) -> Self {
        let _noinfo_flg: bool = _cmdline_info.is_present("no-info");
        let _nocolor_flg: bool = _cmdline_info.is_present("no-color");

        Self {
            cmdline_info: _cmdline_info,
            noinfo_flg: _noinfo_flg,
            nocolor_flg: _nocolor_flg,
            d88fileio: D88FileIO::default(),
        }
    }

    /// Report
    ///
    /// # Argument
    ///   * (none)
    ///
    pub fn report(&mut self) {
        if let Some(d88_path) = self.cmdline_info.value_of("*.D88") {
            if self.noinfo_flg {
                if let Ok(fh) = std::fs::File::open(d88_path) {
                    self.report_d88_noinfo(fh);
                } else {
                    println!("File Not Found \"{}\"", d88_path);
                }
            } else {
                self.d88fileio = D88FileIO::open(d88_path);
                if self.d88fileio.is_open() {
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
            self.print_sector(sector);
        }
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
        unsafe {
            byte_img =
                mem::transmute::<D88_Header, [u8; mem::size_of::<D88_Header>()]>((*header).clone());
        }

        self.print_title_bar();
        self.print_16byte(&byte_img, 0x0000 as u64, ansi_term::Color::Green);


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

        // Report Sector Header
        //
        print!(
            "Track({}), Side({}), Sector({}), ",
            sector.header.track, sector.header.side, sector.header.sec
        );

        print!("Size({} byte/sec), ", 128 << (sector.header.sec_size));
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
    }

    /*
      /// Report Header and Data of Sector (Helper function)
        ///
        /// `report_d88` から呼び出される内部関数。
        /// セクタのヘッダ情報とデータを表示する。
        ///
        /// # Arguments
        ///   * `reader` BufferReader instance at D88 File
        ///   * `offset` Offset to a Sector at D88 Disk File
        ///
        /// # Return
        ///   * if Success, return Ok(`header`)  `header` is D88_SectorHdr instance (Sector Header Info.)
        ///   * if Error, return Err("err")
        ///
        pub fn report_sector_hdr_dat(&mut self, offset: usize) -> Result<D88_SectorHdr, &'static str> {
            // Sector Header
            //
            if let Ok(d88_sector_header) = self.d88fileio.read_sector_header() {
                // Report 16byte Data
                //
                unsafe {
                    let d88_sector_header_2 = d88_sector_header.clone();

                    let byte_img = mem::transmute::<D88_SectorHdr, [u8; mem::size_of::<D88_SectorHdr>()]>(
                        d88_sector_header_2,
                    );
                    self.print_16byte(&byte_img, offset, ansi_term::Color::Green);
                }

                // Report Sector Header
                //
                print!(
                    "Track({}), Side({}), Sector({}), ",
                    d88_sector_header.track, d88_sector_header.side, d88_sector_header.sec
                );

                print!("Size({} byte/sec), ", 128 << (d88_sector_header.sec_size));
                print!("NumOfSec({} sec/track), ", d88_sector_header.number_of_sec);
                print!(
                    "Status({}), ",
                    match d88_sector_header.status {
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
                    //match d88_sector_header.density {
                    //    0x00 => "D",  // 倍密度 // dencityの仕様がよく分からない。
                    //    0x40 => "S",  // 単密度
                    //    0x01 => "HD", // 高密度
                    //    _ => "??",
                    //}
                    d88_sector_header.density
                );
                print!(
                    "Mark({}), ",
                    match d88_sector_header.deleted_mark {
                        0x00 => "NORMAL",
                        0x10 => "DELETED",
                        _ => "??",
                    }
                );
                print!("DataSize({} byte), ", d88_sector_header.size_of_data);

                println!();

                Ok(d88_sector_header) // [u8;16] --> D88_SectorHdr
            } else {
                Err("err")
            }
        }
    */

    /*
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
      pub fn report_sector(&mut self, offset_: usize) -> usize {
            let mut offset = offset_;
            let sector_size;

            // Read Sector Header
            //
            if let Ok(d88_sector_header) = self.report_sector_hdr_dat(offset) {
                // Sector Header Info
                offset += 0x10;
                sector_size = d88_sector_header.size_of_data as usize;
            } else {
                return 0;
            }

            // Read Secrtor Data
            //
            let mut sector_buffer = vec![0; sector_size]; // Get Sector Buffer

            if let Some(ref mut reader) = self.d88fileio.reader {
                let mut rdsize = reader.read(&mut sector_buffer).unwrap();
                while rdsize < sector_size {
                    // 中途までしかリードしていなかったら、残りを読む。RustのBufReaderのデフォルトは8KB単位
                    rdsize += reader.read(&mut sector_buffer[rdsize..]).unwrap();
                }

                // Print Sector
                //
                let mut i: usize = 0;
                while i < sector_size {
                    self.print_16byte(
                        &sector_buffer[i..(i + 16)],
                        offset,
                        ansi_term::Color::RGB(150, 150, 150),
                    );
                    println!();
                    offset += 0x10;
                    i += 0x10;
                }

                0x10 + rdsize // SectorHeader(0x10) + SectorSize
            } else {
                0
            }
        }
    */
}
