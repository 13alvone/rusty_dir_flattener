extern crate argparse;
extern crate users;
extern crate fs_extra;

use {
    std::string::ToString,
    argparse::{ArgumentParser, Store},
    std::{process, fs},
    users::get_current_uid,
    std::os::macos::fs::MetadataExt,
    fs_extra::file::{move_file, CopyOptions},
    std::fs::DirEntry,
};

fn append_metadata(file_path: &str) -> () {
    let md = fs::metadata(file_path);
    match md {
        Ok(perm_object) => {
            let mode = perm_object.st_mode();
            let user_id = perm_object.st_uid().to_string();
            let group_id = perm_object.st_gid().to_string();
            let current_user_id = get_current_uid().to_string();
            let out = "U: ".to_owned() + &user_id +
                ", G: " + &group_id + ", CURRENT: " + &current_user_id + ", Perm: ";
            println!(": ({}{:#o})", out, mode);
        },
        Err(_) => {
            println!("[!] `{}` Failed, No Permissions", file_path);
        }
    };

    ()
}

fn initialize_file_read(in_path: &str) -> Result<fs::ReadDir, std::io::Error> {
    let dir_read = fs::read_dir(in_path);
    if dir_read.is_err() {
        return Err(dir_read.unwrap_err())
    } else {
        return Ok(dir_read.unwrap())
    }
}

fn my_is_file(in_path: &DirEntry) -> bool {
    match in_path.file_type() {
        Ok(t) => {
            let _dir = t.is_dir();
            let _file = t.is_file();
            if _file && !_dir {
                true
            } else {
                false
            }
        },
        Err(e) => {
            println!("[!] Error in `is_file()` {}", e);
            false
        }
    }
}

fn delete_empty_dirs(directory: &str, _root: &str) { 
    let dir_read_result = initialize_file_read(directory);
    match dir_read_result {
        Ok(t) => {
            for dir_entry in t {
                match dir_entry {
                    Ok(t) => {
                        if !my_is_file(&t) {
                            let start_path = t.path().as_path().to_owned();
                            // let parent_path = start_path.parent().unwrap().to_str().unwrap().to_string() + "/";
                            let del_path = start_path.to_str().unwrap().to_string();
                            let print_path = &del_path.to_string();
                            match fs::remove_dir(del_path) {
                                Ok(_) => {
                                    println!("  > Empty Directory Deleted: `{}`", &print_path);
                                },
                                Err(e) => {
                                    println!("[!] Directory Removal Failed: `{}`", e);
                                }
                            };
                        }
                    },
                    Err(_) => continue
                }
            }
        },
        Err(_) => ()
    }
}

fn recursive_flatten(directory: &str, _root: &str) { 
    let dir_read = initialize_file_read(directory);

    match dir_read {
        Ok(t) => {
            for dir_entry in t {
                match dir_entry {
                    Ok(dir_entry) => {
                        let object_name = dir_entry.file_name().to_str().unwrap().to_owned();
                        let is_dir_test = dir_entry.file_type();
                        if !is_dir_test.is_ok() {
                            println!("[!] Error Processing `{}`", object_name);
                            continue;
                        }
        
                        let is_file = !is_dir_test.unwrap().is_dir();
                        let object_path = directory.to_string() + &object_name;
                        let new_path = _root.to_string() + &object_name;
                        // let multiplier = object_path.matches("/").count() - 1;
                        // let depth = (0..multiplier).map(|_| "  ").collect::<String>();
                        if is_file && object_path != new_path {
                            let _result = mv_file(&object_path, &new_path);
                            print!("[+] {} --> {}", &object_path, &new_path);
                            append_metadata(&new_path);
                        } else {
                            // print!("{}[-] Directory: {}", depth, object_path);
                            let dir_path = object_path + "/";
                            // append_metadata(&dir_path);
                            recursive_flatten(&dir_path, _root);
                            delete_empty_dirs(&dir_path, _root);
                        }
                    },
                    Err(_) => ()
                }
        
            }
        }
        Err(_) => ()
    }
}

fn mv_file(from_path: &str, to_path: &str) -> bool {

    let my_options = CopyOptions {
        skip_exist: true,
        overwrite: false,
        buffer_size: 64000
    }; 

    let test = move_file(from_path, to_path, &my_options);
    let move_completed = match test {
        Ok(_) => {
            true
        },
        Err(_) => {
            false
        }
    };

    move_completed
}

fn main() {
    // CMD Argument Cadence
    let mut target_dir = String::new();
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Flatten a target directory's contents.");
        ap.refer(&mut target_dir)
            .add_option(&["-d", "--target_dir"], Store, "Path.");
        ap.parse_args_or_exit();
    }

    if target_dir == "" {
        println!("[!] Blank entries are not allowed, please try again...");
        process::exit(0x0100);
    }

    println!("{}", "[i] `".to_owned() + &target_dir + "` --> Flattening Starting Here.");

    if target_dir.is_ascii() {
        recursive_flatten(&target_dir, &target_dir);
        delete_empty_dirs(&target_dir, &target_dir);
    } else {
        println!("{}", "[!] Fail");
    }

}

