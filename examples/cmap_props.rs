//! Dump properties and strings from a CMAP.
//! Usage: cmap_props <pkg> <path-to-cmap>

use matrixgame_rs::matrix_lib::base::pack::PkgArchive;
use matrixgame_rs::matrix_lib::base::storage::Storage;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let pkg = PkgArchive::from_bytes(std::fs::read(&args[1]).expect("read pkg")).expect("parse");
    let data = pkg.read_file(&args[2]).expect("read cmap");
    let stor = Storage::from_bytes(&data).expect("storage");

    if let (Some(names), Some(vals)) = (
        stor.get_buf("properties", "Name"),
        stor.get_buf("properties", "Value"),
    ) {
        println!("=== properties ===");
        for i in 0..names.arrays_count() {
            println!("{} = {}", names.get_as_wstr(i), vals.get_as_wstr(i));
        }
    }
    if let Some(strings) = stor.get_buf("strings", "String") {
        println!("=== strings ({}) ===", strings.arrays_count());
        for i in 0..strings.arrays_count() {
            println!("[{}] {}", i, strings.get_as_wstr(i));
        }
    }
}
