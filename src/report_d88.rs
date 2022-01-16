/// Report D88 File
use std::fs;
use std::io::{BufReader, Read};
use std::mem;
//extern crate libc;
//use libc;
use ansi_term::Color;

//348848 = 0x2b0(ヘッダ) + 40(トラック数) x 2(面) x 16(セクタ/トラック) x
//                                                   (0x10(セクタヘッダ) + 0x100(セクタデータ))

#[repr(C)]
#[derive(Debug)]
///  Header Information at D88 File
///
///  D88ファイルのヘッダ情報
///
pub struct D88_Header {
    disk_name: [u8; 17],
    reserved: [u8; 9],
    write_protect: u8,
    disk_type: u8,
    disk_size: u32,
    track_tbl: [u32; 164],
}

#[repr(C)]
#[derive(Debug)]
///  Sector Header Information at D88 File
///
///  D88ファイルのセクタのヘッダ情報
///
pub struct D88_SectorHdr {
    track: u8,    // Track
    side: u8,     // Side
    sec: u8,      // Sector
    sec_size: u8, // Sector Size
    number_of_sector: u16,
    density: u8,
    deleted_mark: u8,
    status: u8,
    reserved: [u8; 5],
    size_of_data: u16,
}

/// ReportD88
///
/// D88ファイル情報を表示。
///
pub struct ReportD88 {
    cmdline_info: clap::ArgMatches,
}

impl ReportD88 {
    /// Constructor
    ///
    pub fn new(cmdline_info: clap::ArgMatches) -> Self {
        Self {
            cmdline_info: cmdline_info,
        }
    }

    /// Report
    ///
    /// # Argument
    ///   * (none)
    ///
    pub fn report(&mut self) {
        if let Some(d88_name) = self.cmdline_info.value_of("*.D88") {
            if let Ok(fh) = fs::File::open(d88_name) {
                let mut f = BufReader::new(fh);

                unsafe {
                    self.report_d88(&mut f);
                }
            } else {
                println!(" Not Found \"{}\"", d88_name);
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
    pub unsafe fn report_d88(&mut self, reader: &mut BufReader<std::fs::File>) {
        self.print_title_bar();

        let mut rdsize = self.report_d88_header(reader);
        let mut offset = rdsize;

        rdsize = self.report_sector(reader, offset);
        offset += rdsize;
        while rdsize != 0 {
            rdsize = self.report_sector(reader, offset);
            offset += rdsize;
            //println!("[DEBUG] -->{:x}",rdsize);
        }
    }

    /// Print Tiltle Bar(Helper function)
    ///
    /// `report_d88` から呼び出される内部関数。  
    /// タイトルバーを表示
    ///
    pub fn print_title_bar(&mut self) {
        println!("Offset+0 +1 +2 +3 +4 +5 +6 +7 +8 +9 +a +b +c +d +e +f                 ");
        println!("----- -----------------------------------------------                 ");
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
    pub fn report_d88_header(&mut self, reader: &mut BufReader<std::fs::File>) -> usize {
        let mut buf: [u8; mem::size_of::<D88_Header>()] = [0; mem::size_of::<D88_Header>()]; // Header Buffer

        // Get Command Line Option "--no-info"
        //
        let noinfo_flg: bool = self.cmdline_info.is_present("no-info");

        // File Header 
        //
        if let Ok(read_size) = reader.read(&mut buf) {
            unsafe {
                self.print_16byte(&buf, 0x0000, ansi_term::Color::Green);

                let header = mem::transmute::<[u8; mem::size_of::<D88_Header>()], D88_Header>(buf);

                if !noinfo_flg {
                    // Report D88 Header
                    //

                    //let converted: String = String::from_utf8_unchecked( header.disk_name.to_vec()).unwrap();
                    let disk_name: String = String::from_utf8_unchecked(header.disk_name.to_vec());
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
                }
            }
            println!();

            read_size
        } else {
            0
        }
    }

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
    pub unsafe fn report_sector_hdr_dat(
        &mut self,
        reader: &mut BufReader<std::fs::File>,
        offset: usize,
    ) -> Result<D88_SectorHdr, &'static str> {
        // Get Command Line Option "-n"
        //
        let noinfo_flg: bool = self.cmdline_info.is_present("no-info");

        // Buffer for D88 Sector Header
        //
        let mut d88_sector_header_buf: [u8; mem::size_of::<D88_SectorHdr>()] =
            [0; mem::size_of::<D88_SectorHdr>()];

        // Sector Header
        //
        if let Ok(rdsize) = reader.read(&mut d88_sector_header_buf) {
            if rdsize != 0 {
                // Report 16byte Data
                //
                self.print_16byte(&d88_sector_header_buf, offset, ansi_term::Color::Green);

                // Report Sector Header
                //
                let d88_sector_header = mem::transmute::<
                    [u8; mem::size_of::<D88_SectorHdr>()],
                    D88_SectorHdr,
                >(d88_sector_header_buf);

                if !noinfo_flg {
                    print!(
                        "Track({}), Side({}), Sector({}), ",
                        d88_sector_header.track, d88_sector_header.side, d88_sector_header.sec
                    );

                    print!("Size({} byte/sec), ", 128 << (d88_sector_header.sec_size));
                    print!(
                        "NumOfSec({} sec/track), ",
                        d88_sector_header.number_of_sector
                    );
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
                }

                println!();

                Ok(d88_sector_header) // [u8;16] --> D88_SectorHdr
            } else {
                Err("err")
            }
        } else {
            Err("err")
        }
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
    pub unsafe fn report_sector(
        &mut self,
        reader: &mut BufReader<std::fs::File>,
        offset_: usize,
    ) -> usize {
        let mut offset = offset_;
        let sector_size;

        // Read Header
        //
        if let Ok(d88_sector_header) = self.report_sector_hdr_dat(reader, offset) {
            // Sector Header Info
            offset += 0x10;
            sector_size = d88_sector_header.size_of_data as usize;
        } else {
            return 0;
        }

        // Read Secrtor
        //
        let mut sector_buffer = vec![0; sector_size]; // Get Sector Buffer
        let mut rdsize = reader.read(&mut sector_buffer).unwrap();
        while rdsize < sector_size {
            // 中途までしかリードしていなかったら、残りを読む。RustのBufReaderのデフォルトは8KB単位
            rdsize += reader.read(&mut sector_buffer[rdsize..]).unwrap();
            //println!("BBB rdsize {}", rdsize);
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
    }

    /// Print 16byte (Helper function)
    ///
    /// `report_d88` から呼び出される内部関数。
    /// 16byte表示
    /// D88ファイルのオフセット情報`offset` と 16byte生データ`buf16`から、16byteを整形して表示する。  
    ///
    /// # Argument
    ///
    ///   * `buf16` Slice to 16 byte Buffer
    ///   * `offset` Offset at D88 Disk File
    ///
    /// # Return
    ///
    ///   * Return the value of `offset` plus 16.
    ///
    pub fn print_16byte(&mut self, buf16: &[u8], offset: usize, color: ansi_term::Color) -> usize {
        let mut char_pat = [
            '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
        ];

        // Get Command Line Option "--no-color"
        let nocolor_flg: bool = self.cmdline_info.is_present("no-color");

        // Offset Address
        //
        if !nocolor_flg {
            print!("{} ", Color::Cyan.paint(&(format!("{:05x} ", offset))));
        } else {
            print!("{} ", format!("{:05x}", offset));
        }

        // 16 byte
        //
        let mut byte16_str = String::from("");
        for i in 0..16 {
            unsafe {
                if libc::isprint(buf16[i] as libc::c_int) != 0 {
                    char_pat[i] = buf16[i] as char;
                }
            }

            byte16_str.push_str(&(format!("{:02x} ", buf16[i])));
        }
        //print!("{}", Color::White.paint(byte16_str));

        if !nocolor_flg {
            print!("{}", color.paint(byte16_str));
        } else {
            print!("{}", byte16_str);
        }

        // Character
        //
        for p in char_pat.iter() {
            print!("{}", p);
        }
        print!(" ");

        //
        //
        offset + 16
    }
}
