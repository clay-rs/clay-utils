use std::{fs, cmp::max, ffi::OsStr};
use clay_core::{self, buffer::Image};


fn from_os(os_str: &OsStr) -> String {
    os_str.to_string_lossy().into_owned()
}

pub fn save_screenshot(image: &Image, lossless: bool) -> clay_core::Result<String> {
    fs::create_dir_all("screenshots")?;
    let maxn = fs::read_dir("screenshots")?
    .filter_map(|f_| f_.ok().map(|f| f.path()).and_then(|p| {
        p.file_stem().map(|s| from_os(s))
        .and_then(|n| n.parse::<i32>().ok())
    }))
    .fold(0, |b, n| max(b, n)) + 1;

    let ext = if lossless { "png" } else { "jpg" };

    let filename = format!("screenshots/{:04}.{}", maxn, ext);
    image.save_to_file(&filename)?;
    Ok(filename)
}
