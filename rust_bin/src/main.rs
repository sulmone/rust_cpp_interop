use std::ffi::{CString};
use std::os::raw::{c_char, c_int};

extern "C" {
    fn app_main(num_args: c_int, args: *const *const c_char) -> c_int;
}

fn main() {
    let args = std::env::args_os()
        .map(|arg| arg.to_string_lossy().to_string())
        .map(|arg| arg.into_bytes())
        .map(|arg| unsafe { CString::from_vec_unchecked(arg) })
        .collect::<Vec<_>>();

    let rc = unsafe { app_main(args.len() as c_int, args.as_ptr() as *const *const c_char) };

    std::process::exit(rc);
}