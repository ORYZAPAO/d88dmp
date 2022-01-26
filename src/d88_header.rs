//348848 = 0x2b0(ヘッダ) + 40(トラック数) x 2(面) x 16(セクタ/トラック) x
//                                                   (0x10(セクタヘッダ) + 0x100(セクタデータ))

#[repr(C)]
#[derive(Debug)]
///  Header Information at D88 File
///
///  D88ファイルのヘッダ情報
///
pub struct D88_Header {
    pub disk_name: [u8; 17],
    pub reserved: [u8; 9],
    pub write_protect: u8,
    pub disk_type: u8,
    pub disk_size: u32,
    pub track_tbl: [u32; 164],
}

#[repr(C)]
#[derive(Debug)]
///  Sector Header Information at D88 File
///
///  D88ファイルのセクタのヘッダ情報
///
pub struct D88_SectorHdr {
    pub track: u8,    // Track
    pub side: u8,     // Side
    pub sec: u8,      // Sector
    pub sec_size: u8, // Sector Size
    pub number_of_sector: u16,
    pub density: u8,
    pub deleted_mark: u8,
    pub status: u8,
    pub reserved: [u8; 5],
    pub size_of_data: u16,
}
