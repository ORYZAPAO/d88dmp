use std::fs;
use std::io::BufReader;

mod cli;
mod report_d88;

fn main() {
    // Command Line Analysis
    let cmdline_info = cli::cli();

    if let Some(d88_name) = cmdline_info.value_of("*.D88") {
        if let Ok(fh) = fs::File::open(d88_name) {
            let mut f = BufReader::new(fh);

            unsafe {
                report_d88::report_d88(&mut f, &cmdline_info);
            }
        } else {
            println!(" Not Found \"{}\"", d88_name);
        }
    }
}
