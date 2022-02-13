//348848 = 0x2b0(ヘッダ) + 40(トラック数) x 2(面) x 16(セクタ/トラック) x
//                                                   (0x10(セクタヘッダ) + 0x100(セクタデータ))

pub const MAX_SECTOR: u16 = 164;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
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

impl Default for D88_Header {
    fn default() -> Self {
        Self {
            disk_name: [0u8; 17],
            reserved: [0u8; 9],
            write_protect: 0u8,
            disk_type: 0u8,
            disk_size: 0u32,
            track_tbl: [0u32; MAX_SECTOR as usize],
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
///  Sector Header Information at D88 File
///
///  D88ファイルのセクタのヘッダ情報
///
pub struct D88_SectorHdr {
    pub track: u8,    // Track
    pub side: u8,     // Side
    pub sec: u8,      // Sector
    pub sec_size: u8, // Sector Size
    pub number_of_sec: u16,
    pub density: u8,
    pub deleted_mark: u8,
    pub status: u8,
    pub reserved: [u8; 5],
    pub size_of_data: u16,
}
