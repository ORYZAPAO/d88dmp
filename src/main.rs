mod cli;
mod report_d88;

fn main() {
    // Command Line Analysis
    let cmdline_info = cli::cli();

    let mut rpt_d88 = report_d88::ReportD88::new(cmdline_info);

    rpt_d88.report();
}
