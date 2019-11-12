use std::fs::{self, DirEntry};
use std::io;
use std::path::Path;

//pub fn visit_dirs(dir: &Path, cb: &impl Fn(&DirEntry)) -> io::Result<()> {
pub fn visit_dirs(dir: &Path, cb: &dyn Fn(&DirEntry)) -> io::Result<()> {
    if !dir.is_dir() {
        return Ok(());
    }
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            visit_dirs(&path, cb)?;
        } else {
            cb(&entry);
        }
    }
    Ok(())
}
