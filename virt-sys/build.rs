use std::error::Error;
use std::path::PathBuf;
use std::{env, process};

const LIBVIRT_VERSION: &str = "5.0.0";

fn main() {
    match run() {
        Ok(()) => {}
        Err(e) => {
            eprintln!("error: {}", e);
            process::exit(1);
        }
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-changed=wrapper.h");
    pkg_config::Config::new()
        .atleast_version(LIBVIRT_VERSION)
        .probe("libvirt")?;

    let bindings = bindgen::builder()
        .header("wrapper.h")
        .allowlist_var("^(VIR_|vir).*")
        .allowlist_type("^vir.*")
        .allowlist_function("^vir.*")
        // this is only false on esoteric platforms which libvirt does not support
        .size_t_is_usize(true)
        .generate_comments(false)
        .prepend_enum_name(false)
        .ctypes_prefix("::libc")
        .generate()
        .map_err(|_| String::from("could not generate bindings"))?;

    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    bindings.write_to_file(out_dir.join("bindings.rs"))?;

    Ok(())
}
