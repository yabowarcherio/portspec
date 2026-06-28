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
    #[arg(value_name = "SPEC", required_unless_present = "preset")]
    specs: Vec<String>,

    /// Print the number of ports instead of listing them.
    #[arg(short, long, conflicts_with_all = ["ranges", "contains"])]
    count: bool,

    /// Print the normalized range form (e.g. `1-20,443`) instead of listing.
    #[arg(short, long, conflicts_with_all = ["count", "contains"])]
    ranges: bool,

    /// Emit a JSON summary (spec, count, ranges) instead of listing.
    #[arg(long, conflicts_with_all = ["count", "ranges", "contains"])]
    json: bool,

    /// Keep only ports also present in this spec (intersection).
    #[arg(long, value_name = "SPEC")]
    intersect: Option<String>,

    /// Remove the ports in this spec from the result (difference).
    #[arg(long, value_name = "SPEC")]
    difference: Option<String>,

    /// Invert the result over the whole port space (0-65535).
    #[arg(long)]
    invert: bool,

    /// Test whether a port is covered; exit 0 if so, 1 otherwise.
    #[arg(long, value_name = "PORT")]
    contains: Option<u16>,

    /// Stop after listing this many ports (0 = no limit).
    #[arg(short, long, value_name = "N", default_value_t = 0)]
    limit: u64,

    /// List ports from highest to lowest.
    #[arg(short = 'R', long)]
    reverse: bool,

    /// In default listing mode, also print the service name for each port
    /// from the built-in table (empty string when unknown), as `PORT<TAB>NAME`.
    #[arg(long, conflicts_with_all = ["count", "ranges", "json", "contains"])]
    resolve: bool,

    /// Replace the input specs with a curated preset: `top-100` or `top-1000`
    /// (both TCP). Combines with --intersect/--difference/--invert as usual.
    #[arg(long, value_name = "NAME", conflicts_with = "specs")]
    preset: Option<String>,

    /// Interpret the input as an nmap-style T:/U: spec and emit
    /// `tcp PORT` / `udp PORT` lines (one per port, ascending). Conflicts
    /// with most other flags; useful for piping into per-proto downstream
    /// tools.
    #[arg(long)]
    tagged: bool,
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

    if cli.tagged {
        // Tagged mode: combine all input specs via TaggedSpec union, then
        // emit one `tcp PORT` / `udp PORT` line per port.
        let mut combined = portspec::TaggedSpec::new();
        for s in &specs {
            match portspec::TaggedSpec::from_str(s) {
                Ok(t) => combined = combined.union(&t),
                Err(e) => {
                    eprintln!("portspec: {s:?}: {e}");
                    return ExitCode::from(2);
                }
            }
        }
        let stdout = io::stdout();
        let mut out = stdout.lock();
        for (proto, port) in combined.iter() {
            if writeln!(out, "{proto} {port}").is_err() {
                return ExitCode::SUCCESS;
            }
        }
        return ExitCode::SUCCESS;
    }

    // Parse and union every spec into one. --preset replaces the input list.
    let mut combined = PortSpec::new();
    if let Some(name) = &cli.preset {
        match portspec::preset(name) {
            Ok(spec) => combined = spec,
            Err(_) => {
                eprintln!(
                    "portspec: unknown preset {name:?} (try top-100 / top-1000)"
                );
                return ExitCode::from(2);
            }
        }
    } else {
        for s in &specs {
            match PortSpec::from_str(s) {
                Ok(spec) => combined = combined.union(&spec),
                Err(e) => {
                    eprintln!("portspec: {s:?}: {e}");
                    return ExitCode::from(2);
                }
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

    if let Some(other) = &cli.difference {
        match PortSpec::from_str(other) {
            Ok(spec) => combined = combined.difference(&spec),
            Err(e) => {
                eprintln!("portspec: {other:?}: {e}");
                return ExitCode::from(2);
            }
        }
    }

    if cli.invert {
        combined = combined.complement();
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

    if cli.json {
        let pairs: Vec<[u16; 2]> = combined
            .ranges()
            .iter()
            .map(|r| [r.start(), r.end()])
            .collect();
        let value = serde_json::json!({
            "spec": combined.to_string(),
            "count": combined.count(),
            "ranges": pairs,
        });
        let _ = writeln!(out, "{}", serde_json::to_string_pretty(&value).unwrap());
        return ExitCode::SUCCESS;
    }

    // Default: list ports, one per line. With --resolve, append the service
    // name from the built-in table separated by a TAB.
    let iter: Box<dyn Iterator<Item = u16>> = if cli.reverse {
        Box::new(combined.iter().collect::<Vec<_>>().into_iter().rev())
    } else {
        Box::new(combined.iter().collect::<Vec<_>>().into_iter())
    };
    for (printed, port) in iter.enumerate() {
        if cli.limit != 0 && printed as u64 >= cli.limit {
            break;
        }
        let line_res = if cli.resolve {
            let name = portspec::service_for(port).unwrap_or("");
            writeln!(out, "{port}\t{name}")
        } else {
            writeln!(out, "{port}")
        };
        if line_res.is_err() {
            // Broken pipe (e.g. piped into `head`): stop quietly.
            return ExitCode::SUCCESS;
        }
    }

    ExitCode::SUCCESS
}
