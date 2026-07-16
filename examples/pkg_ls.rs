//! List or extract files from a .pkg archive.
//! Usage: pkg_ls <pkg> [prefix-filter]           — list
//!        pkg_ls <pkg> --extract <path> <out>    — extract one file

use matrixgame_rs::matrix_lib::base::pack::PkgArchive;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let pkg = PkgArchive::from_bytes(std::fs::read(&args[1]).expect("read pkg")).expect("parse");
    if args.get(2).map(|s| s.as_str()) == Some("--extract") {
        let data = pkg.read_file(&args[3]).expect("read file");
        std::fs::write(&args[4], &data).expect("write");
        eprintln!("extracted {} ({} bytes)", args[3], data.len());
        return;
    }
    let filter = args.get(2).cloned().unwrap_or_default().to_uppercase();
    for f in pkg.list_files() {
        if filter.is_empty() || f.to_uppercase().contains(&filter) {
            println!("{f}");
        }
    }
}
