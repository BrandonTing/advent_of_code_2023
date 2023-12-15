fn get_inputs() -> &'static str {
    return "Time:      7  15   30
Distance:  9  40  200";
}

struct Race {
    time: usize,
    distance: usize,
}

struct WinCon {
    hold: usize,
    run: usize,
}

impl Race {
    fn get_win_cons(self: &Self) -> Vec<WinCon> {
        let mut possible_win_cons = vec![];
        println!("distance {:?}", self.distance);
        for i in 0..=self.time {
            println!("time {:?}", i);
            println!("run {:?}", i * (self.time - i));
            if i * (self.time - i) > self.distance {
                possible_win_cons.push(WinCon {
                    hold: i,
                    run: self.time - i,
                })
            }
        }
        return possible_win_cons;
    }
}

fn get_info_for_each_race(line: &str, info_type: &str) -> Vec<usize> {
    line.split_once(info_type)
        .unwrap()
        .1
        .split(" ")
        .collect::<Vec<_>>()
        .iter()
        .flat_map(|x| x.parse::<usize>())
        .collect()
}

fn parse_string_to_race(str: &'static str) -> Vec<Race> {
    let (time_info_str, distance_info_str) = str.split_once("\n").unwrap();
    let times = get_info_for_each_race(time_info_str, "Time:");
    let distances = get_info_for_each_race(distance_info_str, "Distance:");
    let races: Vec<Race> = times
        .iter()
        .enumerate()
        .map(|(i, time)| Race {
            time: *time,
            distance: distances[i],
        })
        .collect();
    return races;
}

fn main() {
    let races = parse_string_to_race(get_inputs());
    let mutiply = races
        .iter()
        .map(|x| x.get_win_cons().len())
        .fold(1, |sum, count| {
            println!("count: {:?}", count);
            sum * count
        });
    println!("mutiply: {:?}", mutiply);
}
