// Commad Line Analysis
use clap::{Arg, ArgMatches};

use crate::utility::ERROR;
use crate::version::*;

/// Commad Line Analysis
///
/// コマンドライン解析を行う
///
/// # Argument
///   * (none)
///
/// # Return
///   * Return clap::ArgMatches Instance
///
pub fn get_cmdline_param() -> ArgMatches {
    let match1 = clap::Command::new(TOOL_NAME)
        .version(TOOL_VERSION)
        .author(AUTHOR)
        .about("D88 Disk Image Dump.")
        .arg(
            Arg::new("*.D88")
                .help("D88 Disk Image")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("TRACK,SIDE,SECTOR")
                .help("Track(0,1,2,...), Side(0 or 1), Sector(1,2,3,...)")
                .takes_value(true)
                .long("pos")
                .short('p'),
        )
        .arg(
            // "-n, --noinfo"  No Report D88 Information
            Arg::new("no-info")
                .help("No information")
                .short('n')
                .long("no-info"),
        )
        .arg(
            // "-n, --no-color"  No Color
            Arg::new("no-color")
                .help("No Color")
                //.short('n')
                .long("no-color"),
        )
        .arg(
            // "-s, --sector"  No Color
            Arg::new("Sort by Disk Sector Order")
                .help("Sort by Disk Sector Order")
                .short('s')
                .long("sort"),
        )
        .get_matches();

    match1

    //    let matches = App::new("My Super Program")
    //        .version("1.0")
    //        .author("Kevin K. <kbknapp@gmail.com>")
    //        .about("Does awesome things")
    //        .arg(Arg::new("config")
    //            .short('c')
    //            .long("config")
    //            .value_name("FILE")
    //            .about("Sets a custom config file")
    //            .takes_value(true))
    //        .arg(Arg::new("INPUT")
    //            .about("Sets the input file to use")
    //            .required(true)
    //            .index(1))
    //        .arg(Arg::new("v")
    //            .short('v')
    //            .multiple_occurrences(true)
    //            .takes_value(true)
    //             .about("Sets the level of verbosity"))
    //        .get_matches();
    //

    //     .subcommand(App::new("test")
    //        .about("controls testing features")
    //          .version("1.3")
    //          .author("Someone E. <someone_else@other.com>")
    //          .arg(Arg::new("debug")
    //              .short('d')
    //              .about("print debug information verbosely")))
    //      .get_matches();
}

/// Str to Number(u8)
///
///
pub fn get_str_to_u8(
    s: &str,       // string
    err_mes: &str, // Error Message
) -> Result<u8, ()> {
    if let Ok(val) = s.parse() {
        Ok(val)
    } else {
        let mes = format!("{}? {}", s, err_mes);
        ERROR(mes.as_str());
        Err(())
    }
}
