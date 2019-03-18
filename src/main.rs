use std::fs::File;
use std::io::BufReader;
use std::io::{self, prelude::*};
use structopt::StructOpt;

use exitfailure::ExitFailure;
use failure::ResultExt;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> Result<(), ExitFailure> {
    let args = Cli::from_args();

    let f = File::open(&args.path)
        .with_context(|_| format!("could not read file `{:?}`", &args.path))?;
    let reader = BufReader::new(f);
    let lines = reader.lines();

    let stdout = io::stdout();
    let handle = stdout.lock();

    grrs::find_matches(lines, &args.pattern, handle)?;

    Ok(())
}

#[test]
fn find_a_match() {
    let mut result = Vec::new();
    let lines = b"lorem ipsum\ndolor sit amet".lines();
    let _ = grrs::find_matches(lines, "lorem", &mut result);
    assert_eq!(result, b"lorem ipsum\n");
}
