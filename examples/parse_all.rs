use android_bp::BluePrint;
use std::path::Path;

fn main() {
    let arg1 = std::env::args().nth(1).unwrap();
    let dir_root = Path::new(&arg1);
    let t1 = std::time::Instant::now();
    let num_parsed = walk_dir(dir_root);
    println!("{} files parsed in {:.3}s", num_parsed, t1.elapsed().as_secs_f32());
}
fn walk_dir(dir: &Path) -> usize {
    let mut num_files = 0;
    for entry in dir.read_dir().unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            if path.file_name().unwrap().to_str().unwrap() == "out" {
                continue;
            }
            if path.file_name().unwrap().to_str().unwrap().starts_with("."){
                continue;
            }
            
            num_files += walk_dir(&path);
        } else {
            let file_name = path.file_name().unwrap().to_str().unwrap();
            if file_name == "Android.bp" {
                let result = BluePrint::from_file(&path);
                match result {
                    Ok(blueprint) => {
                        num_files += 1;
                        println!("{}", path.to_string_lossy());
                        for module in blueprint.modules {
                            println!("{} {:?}", module.typ, module.get("name"));
                        }
                    }
                    Err(e) => {
                        println!("{}: {}", path.to_string_lossy(), e);
                        panic!("please report! this file is not parsed correctly");
                    }
                }
            }
        }
    }
    num_files
}
