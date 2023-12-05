use rayon::prelude::*;

#[derive(Debug)]
struct Mapper {
    range_mappers: Vec<RangeMapper>
}

impl Mapper {
    fn map(&self, from: usize) -> usize {
        if let Some(range_mapper) = self.range_mappers.iter().find(|rm| rm.is_in_range(from)) {
            range_mapper.map(from)
        } else {
            from.clone()
        }
    }
}

#[derive(Debug)]
struct RangeMapper{
    source_start: usize,
    destination_start: usize,
    length: usize
}

impl RangeMapper {
    fn is_in_range(&self, from: usize) -> bool {
        from >= self.source_start && from < self.source_start + self.length
    }

    fn map(&self, from: usize) -> usize {
        if !self.is_in_range(from) {
            panic!("Incorrect range mapping")
        }
        self.destination_start + from - self.source_start
    }
}

fn main() {
    println!("Answer part 1,  lowest location number: {}", part_1(include_str!("../input.txt")));
    println!("Answer part 2,  lowest location number: {}", part_2(include_str!("../input.txt")));
}

fn parse_input(input: &str) -> (Vec<usize>, Vec<Mapper>){
    fn parse_seeds(s: &str) -> Vec<usize> {
        s.split(": ").skip(1).next().unwrap().split_whitespace().map(|si| si.parse::<usize>().unwrap()).collect()
    }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               
    fn parse_mapper(s: &str) -> Mapper {
        Mapper {
            range_mappers:
            s.lines().skip(1)
                .map(|s| {
                    let mut it = s.split_ascii_whitespace();
                    let destination_start = it.next().unwrap().parse::<usize>().unwrap();
                    let source_start = it.next().unwrap().parse::<usize>().unwrap();
                    let length = it.next().unwrap().parse::<usize>().unwrap();
                    RangeMapper{source_start, destination_start, length}
                }).collect()
        }
    }

    let mut splits = input.split("\n\n");
    let seeds = parse_seeds(splits.next().unwrap());
    let seed_to_soil = parse_mapper(splits.next().unwrap());
    let soil_to_fertilizer = parse_mapper(splits.next().unwrap());
    let fertilizer_to_water = parse_mapper(splits.next().unwrap());
    let water_to_light = parse_mapper(splits.next().unwrap());
    let light_to_temperature = parse_mapper(splits.next().unwrap());
    let temperature_to_humidity = parse_mapper(splits.next().unwrap());
    let humidity_to_location = parse_mapper(splits.next().unwrap());
    (seeds, vec!(seed_to_soil, soil_to_fertilizer, fertilizer_to_water, water_to_light, light_to_temperature, temperature_to_humidity, humidity_to_location))
}

fn find_lowest_location_no(seeds: impl Iterator<Item = usize>, mappers: &Vec<Mapper>) -> usize {
    seeds
        .map(|s|
             mappers
                .iter()
                .fold(s, |from, m| 
                    m.map(from)))
        .min().unwrap()
}

fn part_1(input: &str) -> usize {
    let (seeds, mappers) = parse_input(input);
    find_lowest_location_no(seeds.into_iter(), &mappers)
}

fn part_2(input: &str) -> usize {
    fn transform_seeds(seeds: Vec<usize>) -> Vec<std::ops::Range<usize>> {
        seeds.rchunks(2)
            .map(|chunk| {
                let start_range = chunk[0];
                let end_range = chunk[0] + chunk[1];
                start_range..end_range
            }).collect()
    }

    let (seeds, mappers) = parse_input(input);
    let seed_ranges = transform_seeds(seeds);
    seed_ranges.into_par_iter().map(|seed_range| find_lowest_location_no(seed_range.into_iter(), &mappers)).min().unwrap()
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(include_str!("../test.txt")), 35)
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(include_str!("../test.txt")), 46)
    }
}