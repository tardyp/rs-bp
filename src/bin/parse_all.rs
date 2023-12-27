use android_bp::BluePrint;
use core::panic;
use std::fs::File;
use std::io::Read;
use std::path::Path;

fn main() {
    let arg1 = std::env::args().nth(1).unwrap();
    let dir_root = Path::new(&arg1);
    walk_dir(dir_root);
}
fn walk_dir(dir: &Path) {
    for entry in dir.read_dir().unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            walk_dir(&path);
        } else {
            let file_name = path.file_name().unwrap().to_str().unwrap();
            if file_name == "Android.bp" {
                // read the file
                let mut file = File::open(&path).unwrap();
                let mut contents = String::new();
                file.read_to_string(&mut contents).unwrap();
                // parse the file
                let result = BluePrint::parse(&contents);
                match result {
                    Ok(_) => {
                        // println!("{}: {:#?}", path.to_string_lossy(), blueprint);
                    }
                    Err(e) => {
                        println!("{}: {}", path.to_string_lossy(), e);
                        panic!();
                    }
                }
            }
        }
    }
}
