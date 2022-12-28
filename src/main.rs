mod app;
mod dir;
mod merge;

use std::path::PathBuf;

use app::App;
use clap::Parser;
use dir::FileList;
use merge::Merge;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = App::parse();

    let mut file_list = FileList::new();
    file_list.list(PathBuf::from(app.dir), "xlsx")?;

    let mut merge = Merge::new(PathBuf::from(app.file));
    for path in file_list.get_files() {
        merge.merge_file(path)?;
    }
    merge.write()?;

    Ok(())
}
