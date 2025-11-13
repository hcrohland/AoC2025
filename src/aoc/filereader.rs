use std::fs;

pub fn read_file(file_name: &str) -> Vec<String> {
    fs::read_to_string(file_name)
        .expect("File reading error")
        .trim()
        .split("\n")
        .map(|s| String::from(s))
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::aoc::filereader::read_file;

    #[test]
    fn test_read_file() {
        let input = read_file("input/test.txt");
        assert_eq!(5, input.len())
    }
}
