use std::{
    sync::{Arc, Mutex},
    thread::{self},
    vec,
};

use anyhow::{anyhow, Error};
fn get_inputs() -> &'static str {
    return "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
}

struct SourceDestinationMap {
    destination_range_start: usize,
    source_range_start: usize,
    range_counts: usize,
}

impl SourceDestinationMap {
    fn convert(self: &Self, source: usize) -> Option<usize> {
        if self.source_range_start <= source && self.source_range_start + self.range_counts > source
        {
            return Some(self.destination_range_start + source - self.source_range_start);
        }
        return None;
    }
}
struct SourceDestinationMaps {
    maps: Vec<SourceDestinationMap>,
}

impl SourceDestinationMaps {
    fn convert(self: &Self, source: usize) -> usize {
        let mut result = source;
        for map in &self.maps {
            let mapping_result = map.convert(source);
            if mapping_result.is_some() {
                result = mapping_result.unwrap();
                break;
            }
        }
        return result;
    }
}

struct Operation {
    seed_to_soil_maps: SourceDestinationMaps,
    soil_to_fertilizer_maps: SourceDestinationMaps,
    fertilizer_to_water_maps: SourceDestinationMaps,
    water_to_light_maps: SourceDestinationMaps,
    light_to_temperature_maps: SourceDestinationMaps,
    temperature_to_humidity_maps: SourceDestinationMaps,
    humidity_to_location_maps: SourceDestinationMaps,
}

fn parse_maps_from_input(lines: &str) -> Result<SourceDestinationMaps, Error> {
    match lines.split_once("map:\n") {
        Some((_, map_lines)) => {
            let maps = map_lines
                .lines()
                .map(|x| {
                    let map_configs: Vec<_> =
                        x.split(" ").map(|x| x.parse::<usize>().unwrap()).collect();
                    return SourceDestinationMap {
                        destination_range_start: map_configs[0],
                        source_range_start: map_configs[1],
                        range_counts: map_configs[2],
                    };
                })
                .collect::<Vec<_>>();
            return Ok(SourceDestinationMaps { maps });
        }
        None => {
            return Err(anyhow!("invalid map lines"));
        }
    };
}

fn unzip_tuple_seed_vec(tuple_list: (usize, usize)) -> Vec<usize> {
    (tuple_list.0..(tuple_list.0 + tuple_list.1)).collect::<Vec<_>>()
}

fn convert_vec_to_tuple_vec(list: Vec<usize>) -> Option<Vec<(usize, usize)>> {
    if list.len() < 2 || list.len() % 2 == 1 {
        return None;
    }
    let mut tuple_list: Vec<(usize, usize)> = vec![];
    let mut tuple: (usize, usize) = (0, 0);
    let mut is_tuple_head = true;
    list.iter().for_each(|&x| match is_tuple_head {
        true => {
            tuple.0 = x;
            is_tuple_head = false;
        }
        _ => {
            tuple.1 = x;
            is_tuple_head = true;
            tuple_list.push(tuple);
            tuple = (0, 0);
        }
    });
    return Some(tuple_list);
}

fn parse_inputs() -> Result<(Vec<(usize, usize)>, Operation), Error> {
    let inputs = get_inputs();
    let parts: Vec<_> = inputs.split("\n\n").collect();
    if parts.len() != 8 {
        return Err(anyhow!("Invalid inputs"));
    }
    let seeds: Vec<_> = match parts[0].split_once("seeds: ") {
        Some(v) => {
            convert_vec_to_tuple_vec(v.1.split(" ").flat_map(|x| x.parse::<usize>()).collect())
                .unwrap()
        }

        None => return Err(anyhow!("invalid seeds line")),
    };
    let seed_to_soil_maps: SourceDestinationMaps = match parse_maps_from_input(parts[1]) {
        Ok(maps) => maps,
        Err(err) => return Err(err),
    };
    let soil_to_fertilizer_maps: SourceDestinationMaps = match parse_maps_from_input(parts[2]) {
        Ok(maps) => maps,
        Err(err) => return Err(err),
    };

    let fertilizer_to_water_maps: SourceDestinationMaps = match parse_maps_from_input(parts[3]) {
        Ok(maps) => maps,
        Err(err) => return Err(err),
    };

    let water_to_light_maps: SourceDestinationMaps = match parse_maps_from_input(parts[4]) {
        Ok(maps) => maps,
        Err(err) => return Err(err),
    };

    let light_to_temperature_maps: SourceDestinationMaps = match parse_maps_from_input(parts[5]) {
        Ok(maps) => maps,
        Err(err) => return Err(err),
    };

    let temperature_to_humidity_maps: SourceDestinationMaps = match parse_maps_from_input(parts[6])
    {
        Ok(maps) => maps,
        Err(err) => return Err(err),
    };

    let humidity_to_location_maps: SourceDestinationMaps = match parse_maps_from_input(parts[7]) {
        Ok(maps) => maps,
        Err(err) => return Err(err),
    };

    return Ok((
        seeds,
        Operation {
            seed_to_soil_maps,
            soil_to_fertilizer_maps,
            fertilizer_to_water_maps,
            water_to_light_maps,
            light_to_temperature_maps,
            temperature_to_humidity_maps,
            humidity_to_location_maps,
        },
    ));
}

fn main() {
    let (seeds, operation) = parse_inputs().unwrap();
    let operation_arc = Arc::new(Mutex::new(operation));
    let min_location_number: Arc<Mutex<usize>> = Arc::new(Mutex::new(0 as usize));
    let mut handles: Vec<thread::JoinHandle<()>> = vec![];

    for seed_tuple in seeds {
        let seeds = unzip_tuple_seed_vec(seed_tuple);
        let counter = Arc::clone(&min_location_number);
        let operation_arc_clone = Arc::clone(&operation_arc);
        let handle = thread::spawn(move || {
            let mut num: std::sync::MutexGuard<'_, usize> = counter.lock().unwrap();
            let operation_thread = operation_arc_clone.lock().unwrap();
            seeds
                .iter()
                .map(|seed| operation_thread.seed_to_soil_maps.convert(*seed))
                .map(|soil| operation_thread.soil_to_fertilizer_maps.convert(soil))
                .map(|fertilizer| {
                    operation_thread
                        .fertilizer_to_water_maps
                        .convert(fertilizer)
                })
                .map(|water| operation_thread.water_to_light_maps.convert(water))
                .map(|light| operation_thread.light_to_temperature_maps.convert(light))
                .map(|temperature| {
                    operation_thread
                        .temperature_to_humidity_maps
                        .convert(temperature)
                })
                .map(|humidity| operation_thread.humidity_to_location_maps.convert(humidity))
                .for_each(|x| {
                    if *num == 0 {
                        *num = x;
                    } else if x < *num {
                        *num = x;
                    }
                })
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *min_location_number.lock().unwrap());
    // let location_numbers = operation
    //     .seeds
    //     .iter()
    //     .map(|seed| operation.seed_to_soil_maps.convert(*seed))
    //     .map(|soil| operation.soil_to_fertilizer_maps.convert(soil))
    //     .map(|fertilizer| operation.fertilizer_to_water_maps.convert(fertilizer))
    //     .map(|water| operation.water_to_light_maps.convert(water))
    //     .map(|light| operation.light_to_temperature_maps.convert(light))
    //     .map(|temperature| operation.temperature_to_humidity_maps.convert(temperature))
    //     .map(|humidity| operation.humidity_to_location_maps.convert(humidity))
    //     .collect::<Vec<usize>>();
    // let min_location_number: &usize = location_numbers.iter().min().unwrap();
}
