use std::{collections::HashMap, str::FromStr};

use anyhow::{anyhow, Error};

fn get_inputs() -> &'static str {
    return "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
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

fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

fn main() {
    let (instructions, maps, nodes) = match get_inputs().split_once("\n\n") {
        Some(v) => {
            let mut maps: HashMap<&str, Navigation> = HashMap::new();
            let mut starting_nodes: Vec<&str> = vec![];
            v.1.lines().for_each(|line| {
                let (key, navigation_str) = line.split_once(" = ").unwrap();
                match Navigation::from_str(navigation_str) {
                    Ok(v) => {
                        if key.ends_with("A") {
                            starting_nodes.push(key);
                        }
                        maps.insert(key, v);
                    }
                    Err(err) => {
                        println!("create navigation error: {:?}", err)
                    }
                }
            });
            (v.0.chars().collect::<Vec<_>>(), maps, starting_nodes)
        }
        None => panic!("invalid input"),
    };
    println!("nodes: {:?}", nodes);
    let steps_to_z_for_nodes = nodes
        .iter()
        .map(|x| {
            let mut is_match = false;
            let mut steps: usize = 0;
            let mut current = *x;
            while !is_match {
                for instruction in &instructions {
                    // println!("instruction: {:?}", instruction);
                    let map: &Navigation = maps.get(current).unwrap();
                    current = match instruction {
                        'L' => map.l.as_str(),
                        _ => map.r.as_str(),
                    };
                    steps += 1;
                    is_match = current.ends_with("Z");
                    if is_match {
                        break;
                    }
                }
            }
            return steps;
        })
        .collect::<Vec<_>>();

    println!("steps_to_z_for_nodes: {:?}", steps_to_z_for_nodes);
    let lcm_of_steps = lcm(&steps_to_z_for_nodes);
    println!("lcm_of_steps: {:?}", lcm_of_steps);
}
