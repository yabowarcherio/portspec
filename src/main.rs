//! Command-line interface for `portspec`.
//!
//! ```text
//! portspec 22,80,443,8000-8002
//! portspec --count 1-1024
//! portspec --ranges 80,80,1-10,11-20
//! portspec --contains 443 1-1024
//! ```

use std::io::{self, BufRead, Write};
use std::process::ExitCode;
use std::str::FromStr;

use clap::Parser;
use portspec::PortSpec;

/// Parse and expand TCP/UDP port specifications.
#[derive(Parser, Debug)]
#[command(
    name = "portspec",
    version,
    about,
    long_about = None,
    after_help = "A SPEC is a comma-separated list of ports and ranges, e.g. \
                  22,80,443,1000-2000. Multiple SPECs are combined (union). Use \
                  `-` to read specs from stdin, one per line."
)]
struct Cli {
    /// Port specs to combine. Use `-` to read from stdin.
    #[arg(value_name = "SPEC", required = true)]
    specs: Vec<String>,

    /// Print the number of ports instead of listing them.
    #[arg(short, long, conflicts_with_all = ["ranges", "contains"])]
    count: bool,

    /// Print the normalized range form (e.g. `1-20,443`) instead of listing.
    #[arg(short, long, conflicts_with_all = ["count", "contains"])]
    ranges: bool,

    /// Keep only ports also present in this spec (intersection).
    #[arg(long, value_name = "SPEC")]
    intersect: Option<String>,

    /// Test whether a port is covered; exit 0 if so, 1 otherwise.
    #[arg(long, value_name = "PORT")]
    contains: Option<u16>,

    /// Stop after listing this many ports (0 = no limit).
    #[arg(short, long, value_name = "N", default_value_t = 0)]
    limit: u64,

    /// List ports from highest to lowest.
    #[arg(short = 'R', long)]
    reverse: bool,
}

/// Expand the spec list, replacing a `-` with lines read from stdin.
fn collect_specs(args: &[String]) -> io::Result<Vec<String>> {
    let mut out = Vec::new();
    for a in args {
        if a == "-" {
            for line in io::stdin().lock().lines() {
                let line = line?;
                let t = line.trim();
                if !t.is_empty() && !t.starts_with('#') {
                    out.push(t.to_string());
                }
            }
        } else {
            out.push(a.clone());
        }
    }
    Ok(out)
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    let specs = match collect_specs(&cli.specs) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("portspec: failed to read stdin: {e}");
            return ExitCode::from(2);
        }
    };

    // Parse and union every spec into one.
    let mut combined = PortSpec::new();
    for s in &specs {
        match PortSpec::from_str(s) {
            Ok(spec) => combined = combined.union(&spec),
            Err(e) => {
                eprintln!("portspec: {s:?}: {e}");
                return ExitCode::from(2);
            }
        }
    }

    if let Some(other) = &cli.intersect {
        match PortSpec::from_str(other) {
            Ok(spec) => combined = combined.intersection(&spec),
            Err(e) => {
                eprintln!("portspec: {other:?}: {e}");
                return ExitCode::from(2);
            }
        }
    }

    let stdout = io::stdout();
    let mut out = stdout.lock();

    if let Some(port) = cli.contains {
        return if combined.contains(port) {
            ExitCode::SUCCESS
        } else {
            ExitCode::from(1)
        };
    }

    if cli.count {
        let _ = writeln!(out, "{}", combined.count());
        return ExitCode::SUCCESS;
    }

    if cli.ranges {
        let _ = writeln!(out, "{combined}");
        return ExitCode::SUCCESS;
    }

    // Default: list ports, one per line.
    let iter: Box<dyn Iterator<Item = u16>> = if cli.reverse {
        Box::new(combined.iter().collect::<Vec<_>>().into_iter().rev())
    } else {
        Box::new(combined.iter().collect::<Vec<_>>().into_iter())
    };
    for (printed, port) in iter.enumerate() {
        if cli.limit != 0 && printed as u64 >= cli.limit {
            break;
        }
        if writeln!(out, "{port}").is_err() {
            // Broken pipe (e.g. piped into `head`): stop quietly.
            return ExitCode::SUCCESS;
        }
    }

    ExitCode::SUCCESS
}
