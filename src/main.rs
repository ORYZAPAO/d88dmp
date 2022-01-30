mod cli;
mod d88_header;
mod report_d88;
mod report_d88_noinfo;
mod utility;
mod version;

fn main() {
    // Command Line Analysis
    let cmdline_info = cli::get_cmdline_param();

    let mut rpt_d88 = report_d88::ReportD88::new(cmdline_info);

    rpt_d88.report();
}
