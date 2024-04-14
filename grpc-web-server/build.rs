// SPDX-License-Identifier: Apache-2.0

use std::{env, io::BufRead, path::PathBuf, process::Command};

use tonic_build::configure;

/// Generate Rust code from the proto files.
///
/// The build script uses `buf` to list all the proto files in the `proto` directory and then
/// compiles them using `tonic_build`.
///
/// `buf` is needed to run this build script.
fn main() -> anyhow::Result<()> {
    let proto_out_dir = {
        let mut out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
        out_dir.push("proto");
        out_dir
    };

    let buf_ls_files = Command::new("buf")
        .current_dir("proto")
        .arg("ls-files")
        .output()?;

    let protos = buf_ls_files
        .stdout
        .lines()
        .filter_map(|line| line.ok().filter(|s| !s.is_empty()));

    Command::new("buf")
        .args(["export", "proto", "-o"])
        .arg(&proto_out_dir)
        .output()?;

    let proto_files: Vec<_> = protos.map(|proto| proto_out_dir.join(proto)).collect();

    configure()
        .emit_rerun_if_changed(false)
        .compile(&proto_files, &[&proto_out_dir])?;

    println!("cargo:rerun-if-changed=proto");
    println!("cargo:rerun-if-changed=build.rs");

    Ok(())
}
