use std::fs;
use std::fs::DirEntry;
use std::path::Path;

use crate::lib::config::CommandEntry;

pub fn handle_command_newest_directory(
    return_value: &Option<String>,
    command: &CommandEntry,
) -> Option<String> {
    if return_value.is_some() {
        return get_newest_directory(&return_value.clone().unwrap());
    } else {
        return get_newest_directory(&command.args[0]);
    }
}

pub fn handle_command_newest_file(
    return_value: &Option<String>,
    command: &CommandEntry,
) -> Option<String> {
    let mut arg: Option<String> = None;

    /*
    In this context, we're checking to see if return_value is populated.
    If it is, then we assume that the arg[0] is the file extension to search
    Else, we assume that arg[0] is the path, and arg[1] is the extension
    */

    if command.args.len() > 1 {
        arg = Some(command.args[1].clone());
    } else {
        arg = Some(command.args[0].clone());
    }

    if return_value.is_some() {
        return get_newest_file_in_dir(&return_value.clone().unwrap(), arg);
    } else {
        return get_newest_file_in_dir(&command.args[0], arg);
    }
}

fn get_newest_directory(path: &String) -> Option<String> {
    if !Path::new(path).exists() {
        eprintln!("Could not find path \"{}\"", path);
    }

    // Get all the sub-directories in the path argument
    let sub_dirs = fs::read_dir(path).unwrap();
    let latest_path = sub_dirs
        .into_iter()
        .filter(|dir| dir.as_ref().unwrap().file_type().unwrap().is_dir())
        .max_by_key(|d| d.as_ref().unwrap().metadata().unwrap().modified().unwrap())
        .unwrap()
        .unwrap()
        .path();

    let latest_dir = latest_path.as_path().to_str();

    if latest_dir.is_some() {
        return Some(String::from(latest_dir.unwrap()));
    }

    None
}

fn get_newest_file_in_dir(path: &String, file_ext: Option<String>) -> Option<String> {
    if !Path::new(path).exists() {
        eprintln!("Could not find path \"{}\"", path);
        return None;
    }

    let sub_files = fs::read_dir(path)
        .unwrap()
        .filter(|f| f.as_ref().unwrap().file_type().unwrap().is_file());

    if file_ext.is_some() {
        let latest_path = sub_files
            .filter(|f| {
                let path = f.as_ref().unwrap().path();
                let ext = Path::new(path.to_str().unwrap()).extension();

                if ext.is_some() {
                    let result = String::from(ext.unwrap().to_str().unwrap())
                        == String::from(file_ext.as_ref().unwrap());
                    return result;
                }

                false
            })
            .max_by_key(|f| f.as_ref().unwrap().metadata().unwrap().modified().unwrap())
            .unwrap()
            .unwrap()
            .path();

        let latest_file = latest_path.as_path().to_str();

        if latest_file.is_some() {
            return Some(String::from(latest_file.unwrap()));
        }

        return None;
    }

    let latest_path = sub_files
        .max_by_key(|f| f.as_ref().unwrap().metadata().unwrap().modified().unwrap())
        .unwrap()
        .unwrap()
        .path();

    let latest_file = latest_path.as_path().to_str();

    if latest_file.is_some() {
        return Some(String::from(latest_file.unwrap()));
    }

    None

    //    if file_ext.is_some() {
    //        return Option::from(
    //            sub_files
    //                .filter(|f| {
    //                    let path = f.as_ref().unwrap().path();
    //                    let ext = Path::new(path.to_str().unwrap()).extension();
    //
    //                    if ext.is_some() {
    //                        let result = String::from(ext.unwrap().to_str().unwrap())
    //                            == String::from(file_ext.as_ref().unwrap());
    //                        println!("{}", result);
    //                        return result;
    //                    }
    //
    //                    false
    //                })
    //                .max_by_key(|f| f.as_ref().unwrap().metadata().unwrap().modified().unwrap())
    //                .unwrap()
    //                .unwrap(),
    //        );
    //    }
    //
    //    Option::from(
    //        sub_files
    //            .max_by_key(|f| f.as_ref().unwrap().metadata().unwrap().modified().unwrap())
    //            .unwrap()
    //            .unwrap(),
    //    )
}
