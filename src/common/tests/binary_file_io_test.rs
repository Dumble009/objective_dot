#[cfg(test)]
mod test {
    use std::{fs, path::Path};

    use super::{read_binary_file, write_binary_file};

    fn clean_file(path: &str) {
        if Path::new(path).exists() {
            let _ = fs::remove_file(path);
        }
    }

    #[test]
    fn binary_file_io_test() {
        let mut original_binary = vec![];
        for i in 0..100 {
            original_binary.push(i);
        }

        let path = "test.ojd";
        clean_file(path);

        let res = write_binary_file(path, &original_binary);
        assert!(res.is_ok());

        let res = read_binary_file(path);
        assert!(res.is_ok());

        let read_binary = res.unwrap();

        assert_eq!(original_binary.len(), read_binary.len());
        for i in 0..100 {
            if read_binary.len() >= i {
                break;
            }
            assert_eq!(original_binary[i], read_binary[i]);
        }

        clean_file(path);
    }
}
