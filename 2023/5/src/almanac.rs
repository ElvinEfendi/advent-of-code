use std::collections::HashMap;

#[derive(Debug)]
struct MapEntry {
    source_range_start: u32,
    destination_range_start: u32,
    range_length: u32,
}

struct SeedEntry {
    range_start: u32,
    range_length: u32,
}

pub struct Almanac {
    seeds: Vec<SeedEntry>,
    maps: HashMap<String, Vec<MapEntry>>,
}

impl From<&str> for Almanac {
    fn from(s: &str) -> Self {
        let mut seeds = Vec::new();
        let mut maps = HashMap::new();

        let mut lines = s.lines();

        let seeds_line = lines.next().unwrap().split_once(": ").unwrap().1;
        for pair in seeds_line.split_whitespace().collect::<Vec<&str>>().chunks(2) {
            seeds.push(SeedEntry {
                range_start: pair[0].parse::<u32>().unwrap(),
                range_length: pair[1].parse::<u32>().unwrap(),
            });
        }

        let mut map_name = "seed-to-soil";
        while let Some(line) = lines.next() {
            if line.contains("map:") {
                map_name = line.split_whitespace().next().unwrap();
            } else if !line.trim().is_empty() {
                let mut entries = line.split_whitespace();
                let destination_range_start = entries.next().unwrap().parse::<u32>().unwrap();
                let source_range_start = entries.next().unwrap().parse::<u32>().unwrap();
                let range_length = entries.next().unwrap().parse::<u32>().unwrap();

                let map_entry = MapEntry {
                    source_range_start,
                    destination_range_start,
                    range_length,
                };

                maps.entry(map_name.to_string())
                    .or_insert_with(Vec::new)
                    .push(map_entry);
            }
        }

        Almanac {
            seeds,
            maps,
        }
    }
}

impl Almanac {
    fn find_in_map(&self, map_name: &str, value: u32) -> u32 {
        let map = self.maps.get(map_name).unwrap();

        for entry in map {
            if value < entry.source_range_start {
                continue;
            }

            let delta = value - entry.source_range_start;
            if delta < entry.range_length {
                return entry.destination_range_start + delta;
            }
        }

        value
    }

    pub fn find_lowest_location(&self) -> u32 {
        let mut lowest_location = self.maps.get("humidity-to-location").unwrap()[0].destination_range_start;

        for seed in &self.seeds {
            for i in 0..seed.range_length {
                let soil = self.find_in_map("seed-to-soil", seed.range_start + i);
                let fertilizer = self.find_in_map("soil-to-fertilizer", soil);
                let water = self.find_in_map("fertilizer-to-water", fertilizer);
                let light = self.find_in_map("water-to-light", water);
                let temperature = self.find_in_map("light-to-temperature", light);
                let humidity = self.find_in_map("temperature-to-humidity", temperature);
                let location = self.find_in_map("humidity-to-location", humidity);

                if location < lowest_location {
                    lowest_location = location;
                }
            }
        }

        lowest_location
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_almanac_from_str() {
        let input = "\
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15";
        let almanac = Almanac::from(input);

        assert_eq!(almanac.seeds.len(), 2);
        assert_eq!(almanac.seeds[0].range_start, 79);
        assert_eq!(almanac.seeds[0].range_length, 14);
        assert_eq!(almanac.seeds[1].range_start, 55);
        assert_eq!(almanac.seeds[1].range_length, 13);

        assert_eq!(almanac.maps.len(), 2);
        assert_eq!(almanac.maps.get("seed-to-soil").unwrap().len(), 2);
        assert_eq!(almanac.maps.get("soil-to-fertilizer").unwrap().len(), 3);

        assert_eq!(almanac.maps.get("seed-to-soil").unwrap()[0].source_range_start, 98);
        assert_eq!(almanac.maps.get("seed-to-soil").unwrap()[0].destination_range_start, 50);
        assert_eq!(almanac.maps.get("seed-to-soil").unwrap()[0].range_length, 2);
    }

    #[test]
    fn test_find_lowest_location() {
        let input = "\
seeds: 79 14 55 13

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
        let almanac = Almanac::from(input);

        assert_eq!(almanac.find_lowest_location(), 46);
    }
}
