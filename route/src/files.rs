use std::path::{Path, PathBuf};
use rocket::response::NamedFile;
use rocket::get;

#[get("/<file..>")]
pub fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("assets/").join(file)).ok()
}