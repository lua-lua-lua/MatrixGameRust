//! Dump the record/item structure of a CMAP inside a pkg.
//! Usage: cmap_dump <pkg> <path-to-cmap>

use matrixgame_rs::matrix_lib::base::pack::PkgArchive;
use matrixgame_rs::matrix_lib::base::storage::Storage;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let pkg = PkgArchive::from_bytes(std::fs::read(&args[1]).expect("read pkg")).expect("parse");
    let data = pkg.read_file(&args[2]).expect("read cmap");
    let stor = Storage::from_bytes(&data).expect("storage");
    stor.dump_structure();
}
