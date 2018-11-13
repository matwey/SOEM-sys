extern crate bindgen;
extern crate cmake;

use std::env;
use std::format;
use std::path::PathBuf;

fn main() {
	let dst = cmake::Config::new("SOEM").build();

	println!("cargo:rustc-link-search=native={}/lib", dst.display());
	println!("cargo:rustc-link-lib=static=soem");
	println!("cargo:include={}/include", dst.display());

	let bindings = bindgen::Builder::default()
		.clang_arg(format!("-I{}/include", dst.display()))
		.header("wrapper.h")
		.generate()
		.expect("Unable to generate bindings");

	let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
	bindings.write_to_file(out_path.join("bindings.rs"))
		.expect("Couldn't write bindings!");
}

