use std::path::{Path, PathBuf};
use rocket::response::NamedFile;

#[get("/files/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("assets/").join(file)).ok()
}