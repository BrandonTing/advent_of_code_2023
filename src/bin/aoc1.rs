use anyhow::{anyhow, Error};

fn get_input() -> &'static str {
    return "1abc2
    pqr3stu8vwx
    a1b2c3d4e5f
    treb7uchet";
}

fn parse_line(line: &str) -> Result<u32, Error> {
    let mut int_str: String = line.chars().filter(|x| x.to_digit(10).is_some()).collect();
    int_str = int_str
        .chars()
        .enumerate()
        .filter(|(i, _)| *i == 0 || *i == int_str.len() - 1)
        .map(|(_, v)| v)
        .collect::<String>();
    let int_str = match int_str.len() {
        1 => int_str.repeat(2),
        _ => int_str,
    };
    return match int_str.parse() {
        Ok(v) => {
            println!("{:?}", v);
            Ok(v)
        }
        _ => Err(anyhow!("no interger exist")),
    };
}

fn main() {
    let counts: u32 = get_input()
        .lines()
        .flat_map(parse_line)
        .fold(0, |sum, count| sum + count);
    println!("counts: {:?}", counts)
}
