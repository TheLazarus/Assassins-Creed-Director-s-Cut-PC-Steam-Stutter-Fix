use crate::constants::{AC_EXECUTABLES, DEAD_UBI_SERVER_DOMAIN};
use std::fs;
use std::path::Path;

pub fn patch_executables(executables: [&str; 2]) {
    println!("Finding Executables...");

    for exec in executables {
        find_and_patch_executable(exec);
    }
}

fn find_bytes(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack
        .windows(needle.len())
        .position(|window| window == needle)
}

fn patch_executable(exec_bytes: &mut Vec<u8>, exec: &str) {
    let mut start = 0;
    while let Some(pos) = find_bytes(&exec_bytes[start..], DEAD_UBI_SERVER_DOMAIN.as_bytes()) {
        let abs_pos = start + pos;
        for byte in &mut exec_bytes[abs_pos..abs_pos + DEAD_UBI_SERVER_DOMAIN.len()] {
            *byte = 0;
        }
        start = abs_pos + DEAD_UBI_SERVER_DOMAIN.len();
    }

    let path = Path::new(exec);
    let stem = path.file_stem().unwrap().to_str().unwrap();
    let extension = path.extension().unwrap().to_str().unwrap();

    let patched_filename = format!("{}_patched.{}", stem, extension);

    fs::write(&patched_filename, exec_bytes).expect(&format!(
        "❌ Error, failed to patch the executable {}",
        patched_filename
    ));

    println!("✅ Done, patched file written to {}", patched_filename);
}

fn find_and_patch_executable(exec: &str) {
    let mut exec_bytes = fs::read(exec).expect(&format!(
        "ERROR:: Failed to read {}!
    \n 1. This program only works with the Director's Cut Steam PC Version
    \n 2. Make sure you are running this program from /steamapps/common/Assassins Creed
    \n 3. Make sure you have the following files in this directory - {}
    ",
        exec,
        AC_EXECUTABLES.join(", ")
    ));

    println!("✅ Found - {}", exec);
    println!("🔨 Patching - {}", exec);

    patch_executable(&mut exec_bytes, exec);
}
