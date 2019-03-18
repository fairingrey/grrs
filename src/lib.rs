use std::io::prelude::*;
use std::io::Lines;

use exitfailure::ExitFailure;

pub fn find_matches<B: BufRead + Sized>(
    lines: Lines<B>,
    pattern: &str,
    mut handle: impl Write,
) -> Result<(), ExitFailure> {
    for line in lines {
        let line = line?;
        if line.contains(pattern) {
            writeln!(handle, "{}", line)?;
        }
    }
    Ok(())
}
