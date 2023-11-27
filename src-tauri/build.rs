use std::{env, env::var, path::Path};
fn main() {
    if cfg!(windows) {
        let key = "PATH";
        match env::var(key) {
            Ok(_val) => {
                let output = var("OUT_DIR").unwrap();
                let path = Path::new(&output).join("..").join("..").join("..");
                println!("cargo:rustc-env=PATH={}", path.display());
            }
            Err(e) => println!("couldn't interpret {key}: {e}"),
        }
    } else {
        println!("cargo:rustc-env=LD_LIBRARY_PATH=crates/ctp-futures/v_current");
    }
    tauri_build::build()
}
