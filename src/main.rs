use std::io;
use std::path::PathBuf;
use std::process::ExitCode;

use clap::Parser;
use rs_xsheets2stats::Task;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Count rows in each sheet (default)
    #[arg(long)]
    count_rows: bool,

    /// Count total bytes of the workbook file
    #[arg(long)]
    count_bytes: bool,

    /// Count all cells in each sheet
    #[arg(long)]
    count_cells_all: bool,

    /// Count non-empty cells in each sheet
    #[arg(long)]
    count_cells_non_empty: bool,

    /// Path to the Excel workbook (.xlsx)
    path: PathBuf,
}

impl Cli {
    fn to_task(&self) -> Task {
        if self.count_bytes {
            Task::CountBytes
        } else if self.count_cells_all {
            Task::CountCellsAll
        } else if self.count_cells_non_empty {
            Task::CountCellsNonEmpty
        } else {
            Task::CountRows
        }
    }
}

fn sub() -> Result<(), io::Error> {
    let cli = Cli::parse();
    let task = cli.to_task();
    task.book2stats2stdout(&cli.path)?;
    Ok(())
}

fn main() -> ExitCode {
    sub().map(|_| ExitCode::SUCCESS).unwrap_or_else(|e| {
        eprintln!("{e}");
        ExitCode::FAILURE
    })
}
