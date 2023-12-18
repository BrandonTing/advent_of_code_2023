fn get_inputs() -> &'static str {
    return "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
}

#[derive(Clone)]
struct Sequence {
    value: Vec<Vec<i32>>,
}

impl From<&str> for Sequence {
    fn from(s: &str) -> Self {
        let mut sequence: Vec<Vec<i32>> = vec![];
        let init_nums = s
            .split(" ")
            .flat_map(|x| x.parse::<i32>())
            .collect::<Vec<_>>();
        sequence.push(init_nums);
        let mut is_clear = false;
        let mut current_row_index: usize = 0;
        // let mut deltas: Vec<usize> = vec![];
        while !is_clear {
            let row = &sequence[current_row_index];
            let mut current_row_delta: Vec<i32> = vec![];
            let mut prev: i32 = row[0];
            for i in 1..=row.len() - 1 {
                current_row_delta.push(row[i] - prev);
                prev = row[i]
            }
            if current_row_delta.iter().all(|x| *x == 0) {
                is_clear = true;
            }
            sequence.push(current_row_delta);
            current_row_index += 1;
        }
        return Sequence { value: sequence };
    }
}

impl Sequence {
    fn get_row_next(self: &Self, row_index: usize) -> i32 {
        let len = self.value.len();
        match row_index + 1 == len {
            true => *self.value.get(row_index).unwrap().get(0).unwrap(),
            false => {
                let row = self.value.get(row_index).unwrap();
                let prev_row = self.value.get(row_index + 1).unwrap();
                let last_value_of_prev_row = prev_row.get(prev_row.len() - 1).unwrap();

                *row.get(row.len() - 1).unwrap() + *last_value_of_prev_row
            }
        }
    }
    fn get_next(self: &mut Self) -> i32 {
        let len = self.value.len();
        for row_index in 0..len {
            let cloned_self = self.clone();
            let row = &mut self.value[len - 1 - row_index];
            row.push(cloned_self.get_row_next(len - 1 - row_index));
        }
        println!("self: {:?}", self.value);
        return *self.value[0].last().unwrap();
    }
}

fn main() {
    let sum = get_inputs()
        .lines()
        .map(|x| Sequence::from(x).get_next())
        .fold(0, |sum, count| sum + count);

    println!("sum: {:?}", sum);
}
