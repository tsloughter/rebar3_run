#![feature(plugin)]
#![plugin(rustler_codegen)]

#[macro_use]
extern crate rustler;

use std::process::Command;
use std::os::unix::prelude::CommandExt;
use rustler::{atom, NifEnv, NifTerm, NifError, NifDecoder};

rustler_export_nifs!(
    "rebar3_run",
    [("console", 1, console)],
    None
);

fn console<'a>(env: &'a NifEnv, args: &Vec<NifTerm>) -> Result<NifTerm<'a>, NifError> {
    let path: String = try!(args[0].decode());

    // Run release with console
    let _ = Command::new(path).arg("console").exec();


    Ok(atom::get_atom_init("ok").to_term(env))
}
