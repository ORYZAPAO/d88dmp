use ansi_term::Color;
use std::process;
//use super::report_d88::ReportD88;
use crate::report_d88::ReportD88;

/// Print Error Message
///
#[allow(non_snake_case)]
pub fn ERROR(err_mes: &str) {
    println!("[ERROR] {}", err_mes);
    process::exit(0);
}

impl ReportD88 {
    /// Print D88 File Header Titke Bar(Helper function)
    ///
    /// タイトルバーを表示(D88ファイルヘッダ)
    ///
    pub fn print_d88_file_header_title_bar(&self) {
        println!("D88 File Header Summary");
    }

    /// Print Track Offset Table Bar(Helper function)
    ///
    /// タイトルバーを表示(Track)
    ///
    pub fn print_track_offset_table_bar(&self) {
        println!("Track Offset Table");
        println!();
        println!("Track No.");
        println!("hex dec   +0     +1     +2     +3     +4     +5     +6     +7    ");
        print!("--- ----  ------ ------ ------ ------ ------ ------ ------ ------");
    }

    /// Print Sector Summary Bar(Helper function)
    ///
    /// タイトルバーを表示(Sector)
    ///
    pub fn print_sector_summary_bar(&self) {
        println!("Sector Summary");
        println!();
        println!("Track    Side Sector   Data      Header Parameter");
        println!("hex dec  No.  No. Num  Offset   ");
        println!("--- ---- ---  --- ---  -------  ---------------------------------- ...");
    }

    /// Print Offset Bar(Helper function)
    ///
    /// タイトルバーを表示(Sector)
    ///
    pub fn print_offset_bar(&self) {
        println!("Offset  +0 +1 +2 +3 +4 +5 +6 +7 +8 +9 +a +b +c +d +e +f                 ");
        println!("------  -----------------------------------------------                 ");
    }

    /// Print 16byte (Helper function)
    ///
    /// 16byte表示
    /// D88ファイルのオフセット情報`offset` と 16byte生データ`buf16`から、16byteを整形して表示する。  
    ///
    /// # Argument
    ///
    ///   * `buf16` Slice to 16 byte Buffer
    ///   * `offset` Offset at D88 Disk File
    ///   * `color` Color  
    ///
    /// # Return
    ///
    ///   * Return the value of `offset` plus 16.
    ///
    pub fn print_16byte(&self, buf16: &[u8], offset: u64, color: ansi_term::Color) -> u64 {
        self.print_16byte_len(buf16, offset, color, 16)
    }

    /// Print 16byte (Helper function)
    ///
    /// 16byte表示
    /// D88ファイルのオフセット情報`offset` と 16byte生データ`buf16`から、(length)byteを整形して表示する。  
    ///
    /// # Argument
    ///
    ///   * `buf16` Slice to 16 byte Buffer
    ///   * `offset` Offset at D88 Disk File
    ///   * `color` Color  
    ///   * `length`
    ///
    /// # Return
    ///
    ///   * Return the value of `offset` plus 16.
    ///
    #[allow(clippy::format_in_format_args)]
    pub fn print_16byte_len(
        &self,
        buf16: &[u8],
        offset: u64,
        color: ansi_term::Color,
        length: usize,
    ) -> u64 {
        let mut char_pat = [
            '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
        ];

        assert!(length <= 16); // length = 0-16

        // Offset Address
        //
        if !self.nocolor_flg {
            print!("{}  ", Color::Cyan.paint(&(format!("{:06x}", offset))));
        } else {
            //#[allow(clippy::format_in_format_args)]
            print!("{}  ", format!("{:06x}", offset));
        }

        // 16 byte
        //
        let mut byte16_str = String::from("");
        for i in 0..length {
            unsafe {
                if libc::isprint(buf16[i] as libc::c_int) != 0 {
                    char_pat[i] = buf16[i] as char;
                }
            }

            byte16_str.push_str(&(format!("{:02x} ", buf16[i])));
        }
        //print!("{}", Color::White.paint(byte16_str));

        if !self.nocolor_flg {
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
        offset + length as u64
    }
}
