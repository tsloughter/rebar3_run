extern crate rustler;
extern crate rustler_codegen;

use std::process::Command;
use std::os::unix::prelude::CommandExt;
use rustler::{Atom, NifResult};

mod atoms {
    rustler::atoms! { ok }
}

#[rustler::nif]
fn console(path: String) -> NifResult<Atom> {
    // Run release with console
    let _ = Command::new(path).arg("console").exec();

    Ok(atoms::ok())
}

rustler::init!("rebar3_run", [console]);
