#![windows_subsystem = "console"]
#[macro_use]
extern crate vm_bindings;
extern crate num;
#[macro_use]
extern crate num_traits;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

pub(crate) mod platform;
mod runtime;
pub use runtime::*;

use clap::{App, AppSettings, Arg};
use std::sync::mpsc::channel;
use std::sync::Arc;
use vm_bindings::InterpreterConfiguration;

fn main() {
    env_logger::init();

    let matches = App::new("Virtual Machine")
        .version("1.0")
        .author("feenk gmbh. <contact@feenk.com>")
        .setting(AppSettings::AllowExternalSubcommands)
        .arg(
            Arg::new("image")
                .value_name("image")
                .index(1)
                .required(true)
                .help("A path to an image file to run"),
        )
        .arg(
            Arg::new("interactive")
                .long("interactive")
                .help("Start image in the interactive (UI) mode"),
        )
        .arg(
            Arg::new("worker")
                .long("worker")
                .help("Start image in the worker thread"),
        )
        .arg(
            Arg::new("no-error-handling")
                .long("no-error-handling")
                .help("Disable error handling by the virtual machine"),
        )
        .get_matches();

    let image_path = match validate_user_image_file(matches.value_of("image")) {
        None => {
            eprintln!("Could not find an .image file");
            return;
        }
        Some(path) => path,
    };

    let mut extra_args: Vec<String> = vec![];
    if let Some((external, sub_m)) = matches.subcommand() {
        extra_args.push(external.to_owned());
        if let Some(values) = sub_m.values_of("") {
            for each in values {
                extra_args.push(each.to_owned());
            }
        }
    }

    let mut configuration = InterpreterConfiguration::new(image_path);
    configuration.set_interactive_session(matches.is_present("interactive"));
    configuration.set_is_worker_thread(matches.is_present("worker"));
    configuration.set_should_handle_errors(!matches.is_present("no-error-handling"));
    configuration.set_extra_arguments(extra_args);
    Constellation::run(configuration);
}
