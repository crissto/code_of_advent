use std::fs;

pub fn read_input(day: i8) -> String {
    let result = fs::read_to_string(format!("src/day{}/input.txt", day))
        .expect("Should have been able to read the file");
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = read_input(1);
        assert!(result.len() > 0)
    }
}
