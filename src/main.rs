use std::fs;
use std::path::PathBuf;

// 用來表示資料夾的結構體
#[derive(Debug)]
struct Folder {
    name: String,            // 資料夾名稱
    subfolders: Vec<Folder>, // 子資料夾集合
    zip_files: Vec<ZipFile>, // zip 檔案集合
}

// 用來表示 zip 檔案的結構體
#[derive(Debug)]
struct ZipFile {
    name: String, // zip 檔案名稱
}

impl Folder {
    // 遞迴地遍歷資料夾結構，為每個資料夾建立一個 Folder 結構體
    fn from_path(path: &PathBuf) -> Folder {
        let mut subfolders = Vec::new();
        let mut zip_files = Vec::new();

        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let entry_path = entry.path();
                    if entry_path.is_dir() {
                        subfolders.push(Folder::from_path(&entry_path));
                    } else if let Some(extension) = entry_path.extension() {
                        if extension == "zip" {
                            zip_files.push(ZipFile {
                                name: entry_path
                                    .file_name()
                                    .unwrap()
                                    .to_string_lossy()
                                    .to_string(),
                            });
                        }
                    }
                }
            }
        }

        Folder {
            name: path.file_name().unwrap().to_string_lossy().to_string(),
            subfolders,
            zip_files,
        }
    }
}

fn main() {
    let comic_path = PathBuf::from("C:\\comic");
    let mut folders = Vec::new();

    if comic_path.is_dir() {
        if let Ok(entries) = fs::read_dir(comic_path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let entry_path = entry.path();
                    if entry_path.is_dir() {
                        let folder = Folder::from_path(&entry_path);
                        folders.push(folder);
                    }
                }
            }
        }
    }

    println!("{:#?}", folders);
}
