use std::{fs, io::Error};

pub fn read_file(file_name: &str) -> Result<Vec<String>, Error> {
    fs::read_to_string(file_name).map(|raw| raw.trim().split("\n").map(String::from).collect())
}

#[cfg(test)]
mod tests {
    use crate::aoc::filereader::read_file;

    #[test]
    fn test_read_file() {
        let input = read_file("input/test.txt");
        assert_eq!(5, input.unwrap().len())
    }
}
