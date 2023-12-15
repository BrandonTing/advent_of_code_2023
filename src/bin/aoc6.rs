fn get_inputs() -> &'static str {
    return "Time:      7  15   30
Distance:  9  40  200";
}

struct Race {
    time: usize,
    distance: usize,
}

impl Race {
    fn get_win_con_amounts(self: &Self) -> usize {
        let mut amount: usize = 0;
        for i in 0..=self.time {
            if i * (self.time - i) > self.distance {
                amount += 1;
            }
        }
        return amount;
    }
}

fn get_info_for_race(line: &str, info_type: &str) -> usize {
    line.split_once(info_type)
        .unwrap()
        .1
        .split(" ")
        .collect::<Vec<_>>()
        .iter()
        .filter(|x| x.parse::<usize>().is_ok())
        .fold("".to_string(), |sum, num_str| sum + &num_str)
        .parse::<usize>()
        .unwrap()
}

fn parse_string_to_race(str: &'static str) -> Race {
    let (time_info_str, distance_info_str) = str.split_once("\n").unwrap();
    let time = get_info_for_race(time_info_str, "Time:");
    let distance = get_info_for_race(distance_info_str, "Distance:");
    return Race { time, distance };
}

fn main() {
    let race = parse_string_to_race(get_inputs());
    // let mutiply = races
    //     .iter()
    //     .map(|x| x.get_win_cons().len())
    //     .fold(1, |sum, count| {
    //         println!("count: {:?}", count);
    //         sum * count
    //     });
    // println!("mutiply: {:?}", mutiply);
    let win_con_amount = race.get_win_con_amounts();
    println!("win_con_amount: {:?}", win_con_amount);
}
