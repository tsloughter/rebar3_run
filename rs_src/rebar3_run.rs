#[macro_use]
extern crate ruster_unsafe;
extern crate libc;

use std::str;
use std::ffi::CStr;
use std::process::Command;
use std::mem::uninitialized;
use libc::types::os::arch::c95::c_uchar;

use ruster_unsafe::*;

nif_init!(b"rebar3_run\0", None, None, None, None, nif!(b"console\0", 1, console));

extern "C" fn console(env: *mut ErlNifEnv,
                  _argc: c_int,
                  args: *const ERL_NIF_TERM) -> ERL_NIF_TERM {
    // Get path to release start script from args
    let path = unsafe {
        let cmd:*mut c_uchar = uninitialized();
        let mut length:u32 = uninitialized();

        enif_get_list_length(env, *args.offset(0), &mut length);
        enif_get_string(env, *args.offset(0), cmd, length+1, ErlNifCharEncoding::ERL_NIF_LATIN1);

        CStr::from_ptr(cmd as *const i8)
    };

    let command = str::from_utf8(path.to_bytes()).unwrap().to_string();

    // Run release with console
    let _ = Command::new(command).arg("console").status().unwrap_or_else(|e| {
            panic!("failed to execute process: {}", e)
    });

    unsafe {
        // return 'ok'
        enif_make_atom(env, b"ok\0" as *const u8)
    }
}
