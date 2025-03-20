mod models;
mod services;
use std::process;

use clap::Parser;
use ctrlc;

// Ensure that only one branching heuristic is enabled
fn ensure_one_bh_enabled() {
    let features = [
        cfg!(feature = "bh_chb"),
        cfg!(feature = "bh_lrb"),
        cfg!(feature = "bh_vsids"),
    ];

    let enabled_features = features.iter().filter(|&f| *f).count();
    assert!(enabled_features == 1)
}

fn interrupt() {
    println!("Interrupted!");
    process::exit(0);
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Input may be either in plain or gzipped DIMACS.
    input_file: String,
    /// If given, write the results to this file.
    results_output_file: Option<String>,

    /// Verbosity level (0=silent, 1=some, 2=more).
    #[arg(long, default_value_t = 1)]
    verb: i8,

    /// Completely turn on/off any preprocessing.
    #[arg(short, long, default_value_t = true)]
    pre: bool,

    /// If given, stop after preprocessing and write the result to this file.
    #[arg(short, long)]
    dimacs: Option<String>,

    /// Limit on CPU time allowed in seconds.
    #[arg(short, long, default_value_t = i32::MAX)]
    cpu_lim: i32,

    /// Limit on memory usage in megabytes.
    #[arg(short, long, default_value_t = i32::MAX)]
    mem_lim: i32,

    /// If given, use the assumptions in the file.
    #[arg(short, long)]
    assumptions: Option<String>,
}

fn main() {
    ensure_one_bh_enabled();

    ctrlc::set_handler(interrupt).expect("Error setting Ctrl-C handler");

    // Interact with the user through the CLI
    let args = Args::parse();
    println!("{}", args.input_file);
}
