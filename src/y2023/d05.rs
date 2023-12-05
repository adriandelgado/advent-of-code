use std::ops::Range;

pub fn part1(input: &str) -> u64 {
    let (seeds, data) = extract_info(input);

    seeds
        .into_iter()
        .map(|seed| data.seed_to_location(seed))
        .min()
        .unwrap()
}

pub fn part2(input: &str) -> u64 {
    let (seeds, data) = extract_info(input);

    seeds
        .chunks_exact(2)
        .inspect(|_| println!("here"))
        .flat_map(|range| {
            let &[start, length] = range else {
                unreachable!()
            };

            start..(start + length)
        })
        .into_iter()
        .map(|seed| data.seed_to_location(seed))
        .min()
        .unwrap()
}

// destination source range

fn extract_info(input: &str) -> (Vec<u64>, Data) {
    let mut parts = input.split("\n\n");

    let seeds = parts.next().unwrap()[7..]
        .split(' ')
        .map(|seed| seed.parse::<u64>().unwrap())
        .collect();

    let seed_to_soil = parts
        .next()
        .unwrap()
        .split_once('\n')
        .unwrap()
        .1
        .lines()
        .map(|line| {
            let mut nums = line.split(' ');
            let dest = nums.next().unwrap().parse::<u64>().unwrap();
            let source = nums.next().unwrap().parse::<u64>().unwrap();
            let range = nums.next().unwrap().parse::<u64>().unwrap();

            (dest..(dest + range), source..(source + range))
        })
        .collect();

    let soil_to_fertilizer = parts
        .next()
        .unwrap()
        .split_once('\n')
        .unwrap()
        .1
        .lines()
        .map(|line| {
            let mut nums = line.split(' ');
            let dest = nums.next().unwrap().parse::<u64>().unwrap();
            let source = nums.next().unwrap().parse::<u64>().unwrap();
            let range = nums.next().unwrap().parse::<u64>().unwrap();

            (dest..(dest + range), source..(source + range))
        })
        .collect();

    let fertilizer_to_water = parts
        .next()
        .unwrap()
        .split_once('\n')
        .unwrap()
        .1
        .lines()
        .map(|line| {
            let mut nums = line.split(' ');
            let dest = nums.next().unwrap().parse::<u64>().unwrap();
            let source = nums.next().unwrap().parse::<u64>().unwrap();
            let range = nums.next().unwrap().parse::<u64>().unwrap();

            (dest..(dest + range), source..(source + range))
        })
        .collect();

    let water_to_light = parts
        .next()
        .unwrap()
        .split_once('\n')
        .unwrap()
        .1
        .lines()
        .map(|line| {
            let mut nums = line.split(' ');
            let dest = nums.next().unwrap().parse::<u64>().unwrap();
            let source = nums.next().unwrap().parse::<u64>().unwrap();
            let range = nums.next().unwrap().parse::<u64>().unwrap();

            (dest..(dest + range), source..(source + range))
        })
        .collect();

    let light_to_temperature = parts
        .next()
        .unwrap()
        .split_once('\n')
        .unwrap()
        .1
        .lines()
        .map(|line| {
            let mut nums = line.split(' ');
            let dest = nums.next().unwrap().parse::<u64>().unwrap();
            let source = nums.next().unwrap().parse::<u64>().unwrap();
            let range = nums.next().unwrap().parse::<u64>().unwrap();

            (dest..(dest + range), source..(source + range))
        })
        .collect();

    let temperature_to_humidity = parts
        .next()
        .unwrap()
        .split_once('\n')
        .unwrap()
        .1
        .lines()
        .map(|line| {
            let mut nums = line.split(' ');
            let dest = nums.next().unwrap().parse::<u64>().unwrap();
            let source = nums.next().unwrap().parse::<u64>().unwrap();
            let range = nums.next().unwrap().parse::<u64>().unwrap();

            (dest..(dest + range), source..(source + range))
        })
        .collect();

    let humidity_to_location = parts
        .next()
        .unwrap()
        .split_once('\n')
        .unwrap()
        .1
        .lines()
        .map(|line| {
            let mut nums = line.split(' ');
            let dest = nums.next().unwrap().parse::<u64>().unwrap();
            let source = nums.next().unwrap().parse::<u64>().unwrap();
            let range = nums.next().unwrap().parse::<u64>().unwrap();

            (dest..(dest + range), source..(source + range))
        })
        .collect();

    (
        seeds,
        Data {
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location,
        },
    )
}

// destination source range
struct Data {
    seed_to_soil: Vec<(Range<u64>, Range<u64>)>,
    soil_to_fertilizer: Vec<(Range<u64>, Range<u64>)>,
    fertilizer_to_water: Vec<(Range<u64>, Range<u64>)>,
    water_to_light: Vec<(Range<u64>, Range<u64>)>,
    light_to_temperature: Vec<(Range<u64>, Range<u64>)>,
    temperature_to_humidity: Vec<(Range<u64>, Range<u64>)>,
    humidity_to_location: Vec<(Range<u64>, Range<u64>)>,
}

impl Data {
    fn seed_to_location(&self, seed: u64) -> u64 {
        let soil = do_the_mapping(seed, &self.seed_to_soil);
        let fertilizer = do_the_mapping(soil, &self.soil_to_fertilizer);
        let water = do_the_mapping(fertilizer, &self.fertilizer_to_water);
        let light = do_the_mapping(water, &self.water_to_light);
        let temperature = do_the_mapping(light, &self.light_to_temperature);
        let humidity = do_the_mapping(temperature, &self.temperature_to_humidity);
        do_the_mapping(humidity, &self.humidity_to_location)
    }
}

fn do_the_mapping(input: u64, map: &[(Range<u64>, Range<u64>)]) -> u64 {
    for (dest, source) in map {
        if source.contains(&input) {
            let shift = input - source.start;
            return dest.start + shift;
        }
    }
    input
}
