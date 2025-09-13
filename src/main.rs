mod constants;
mod utils;

use crate::constants::AC_EXECUTABLES;
use crate::utils::patch_executables;
use std::thread;
use std::time::Duration;

fn main() {
    println!("Assassins Creed Stutter Fix v0.1 - with love by laazarus");
    println!("Finding AC Executables...");

    patch_executables(AC_EXECUTABLES);
    println!("You can now take a backup of your original exes, and rename these patched executables to {}", AC_EXECUTABLES.join(", "));
    println!("✅ Done, Successfully patched executables, for more instructions check out README");
    println!("GitHub - ");

    thread::sleep(Duration::from_secs(5));
}
