extern crate bindgen;
extern crate cc;
extern crate pkg_config;

use std::env;
use std::path::PathBuf;

fn bindgen_vncserver() {
    let libvncserver = pkg_config::probe_library("libvncserver").unwrap();

    let link_paths = libvncserver.link_paths[0].to_str().unwrap();
    let lib_path = env::current_dir().unwrap().join(link_paths);

    println!("cargo:rustc-link-search={}", lib_path.display());
    println!("cargo:rustc-link-lib=dylib=vncserver");

    let header_file = match env::var("LIBVNCSERVER_HEADER_FILE") {
        Ok(header_file) => header_file,
        Err(_) => format!(
            "{}/rfb/rfb.h",
            libvncserver.include_paths[0].to_str().unwrap(),
        ),
    };

    let bindings = bindgen::Builder::default()
        .header(header_file)
        .generate()
        .expect("unable to generate rfb bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("rfb.rs"))
        .expect("couldn't write bindings!");
}

fn main() {
    bindgen_vncserver();
}
