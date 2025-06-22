use std::fs;
use std::io::Write;
use std::{fs::File, path::Path};

pub fn read_binary_file(path: &str) -> Result<Vec<u8>, String> {
    if !Path::new(path).exists() {
        return Err(String::from(format!("File {} doesn't exist", path)));
    }

    let res = fs::read(path);
    if let Err(msg) = res {
        return Err(String::from(format!(
            "Failed to read binary. path : {}, msg : {}",
            path, msg
        )));
    } else if let Ok(data) = res {
        return Ok(data);
    }

    Err(String::from("Failed for unknown reason"))
}

pub fn write_binary_file(path: &str, binary: &Vec<u8>) -> Result<(), String> {
    let res = File::create(path);
    if let Err(msg) = res {
        return Err(String::from(format!(
            "Failed to open file. path : {}, msg : {}",
            path, msg
        )));
    } else if let Ok(mut file) = res {
        let res = file.write_all(&binary);
        if let Err(msg) = res {
            return Err(String::from(format!(
                "Failed to write_all. path : {}, msg : {}",
                path, msg
            )));
        }

        return Ok(());
    }

    Err(String::from("Failed for unknown rason"))
}

include!("tests/binary_file_io_test.rs");
