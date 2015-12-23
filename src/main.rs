extern crate gdk;
extern crate getopts;
extern crate gtk;
extern crate hyper;
extern crate pango;
extern crate select;
extern crate term;

mod phoronix;
use phoronix::cli;
use phoronix::gui;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut opts = getopts::Options::new();

    opts.optflag("g", "gui", "display in a GTK3 GUI");
    opts.optflag("h", "help", "prints this help menu");
    opts.optflag("n", "no-color", "prints without colors");

    let matches = opts.parse(&args[1..]).unwrap();

    if matches.opt_present("h") { print_help(); return; }

    match matches.opt_present("g") {
        true => gui::launch(),
        false => {
            match matches.opt_present("n") {
                true => cli::print(),
                false => cli::print_colored(),
            }
        }
    }
}

fn print_help() {
    println!("Prints the latest information from Phoronix.");
    println!("    -g, --gui      : launches a GTK3 GUI instead of outputting to the terminal");
    println!("    -h, --help     : prints this help menu");
    println!("    -n, --no-color : prints without colors");
}
