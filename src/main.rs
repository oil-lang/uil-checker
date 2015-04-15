#![feature(plugin)]
#![plugin(docopt_macros)]

extern crate uil_parsers;
extern crate uil_shared;
extern crate docopt;
extern crate term;
extern crate rustc_serialize;

use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use colored_reporter::ColoredErrorReporter;
mod colored_reporter;

docopt!(Args derive Debug, "
Usage: uil-checker [--markup <markup_file>] ([--deps <deps_file>] | [--style <style_file> --deps <deps_file>])
       uil-checker --help

Options:
  -h, --help                   Show this message.
  -m, --markup <markup_file>   Check the given markup file.
  -s, --style <style_file>     Check the given style file.
  -d, --deps <deps_file>       Check the given deps file.
");

fn main() {
    let args: Args = Args::docopt().decode().unwrap_or_else(|e| e.exit());

    let reporter = ColoredErrorReporter::new();

    if !args.flag_markup.is_empty() {
        let file = File::open(&Path::new(&args.flag_markup)).unwrap();
        let reader = BufReader::new(file);
        uil_parsers::markup::parse(reporter.clone(), reader);
    }

    if !args.flag_deps.is_empty() {
        let file = File::open(&Path::new(&args.flag_deps)).unwrap();
        let reader = BufReader::new(file);
        uil_parsers::deps::parse(reporter.clone(), reader);
    }

    if !args.flag_style.is_empty() && !args.flag_deps.is_empty() {
        let deps = {
            let file = File::open(&Path::new(&args.flag_deps)).unwrap();
            let reader = BufReader::new(file);
            uil_parsers::deps::parse(uil_parsers::EmptyErrorReporter, reader)
        };
        let file = File::open(&Path::new(&args.flag_style)).unwrap();
        let reader = BufReader::new(file);
        uil_parsers::style::parse(reporter.clone(), reader, &deps,
            &mut uil_shared::resource::create_null_manager());
    }

    else if !args.flag_style.is_empty() {
        println!("--style option can't be set without --deps one.");
    }
}
