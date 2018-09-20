extern crate docopt;
#[macro_use]
extern crate serde_derive;

extern crate audit_filter;

use audit_filter::run;
use docopt::Docopt;

const USAGE: &str = "
audit-filter filters the output of \"npm audit --json\"

Usage:
  audit-filter [--audit=<->] [--nsp-config=<.nsprc>]
  audit-filter (-h | --help)
  audit-filter --version

Options:
  -h --help                       Show this screen.
  --version                       Show version.
  --audit=<audit>                 NPM Audit JSON file [default: -].
  --nsp-config=<config>           Default filter config [default: .nsprc].
";

#[derive(Debug, Deserialize)]
struct Args {
    flag_audit: String,
    flag_nsp_config: String,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    ::std::process::exit(match run(&args.flag_audit, &args.flag_nsp_config) {
        Ok(ref unacked_advisory_ids) if unacked_advisory_ids.is_empty() => {
            println!("No advisories found after filtering.");
            0
        }
        Ok(ref unacked_advisory_ids) if !unacked_advisory_ids.is_empty() => {
            let advisory_urls: Vec<String> = unacked_advisory_ids
                .into_iter()
                .map(|id| format!("https://nodesecurity.io/advisories/{}", id))
                .collect();
            eprintln!("Unfiltered advisories:\n  {}", advisory_urls.join("\n  "));
            1
        }
        Ok(_) => unimplemented!(), // should never haappen
        Err(err) => {
            eprintln!("{}", err);
            2
        }
    });
}