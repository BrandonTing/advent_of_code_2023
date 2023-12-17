use std::{collections::HashMap, str::FromStr};

use anyhow::{anyhow, Error};

fn get_inputs() -> &'static str {
    return "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
}

struct Navigation {
    l: String,
    r: String,
}

impl FromStr for Navigation {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().replace("(", "").replace(")", "").split_once(", ") {
            Some(v) => Ok(Navigation {
                l: v.0.to_string(),
                r: v.1.to_string(),
            }),
            None => Err(anyhow!("invalid input for navigation")),
        }
    }
}

fn main() {
    let (instructions, maps) = match get_inputs().split_once("\n\n") {
        Some(v) => {
            let mut maps: HashMap<&str, Navigation> = HashMap::new();
            v.1.lines().for_each(|line| {
                let (key, navigation_str) = line.split_once(" = ").unwrap();
                match Navigation::from_str(navigation_str) {
                    Ok(v) => {
                        maps.insert(key, v);
                    }
                    Err(err) => {
                        println!("create navigation error: {:?}", err)
                    }
                }
            });
            (v.0.chars().collect::<Vec<_>>(), maps)
        }
        None => panic!("invalid input"),
    };
    let mut is_match = false;
    let mut steps: usize = 0;

    let mut current = "AAA";
    let target = "ZZZ";
    while !is_match {
        for instruction in &instructions {
            match instruction {
                'L' => current = &maps.get(current).unwrap().l,
                _ => current = &maps.get(current).unwrap().r,
            }
            steps += 1;
            if current == target {
                is_match = true;
                break;
            }
        }
    }

    println!("steps: {:?}", steps);
}
