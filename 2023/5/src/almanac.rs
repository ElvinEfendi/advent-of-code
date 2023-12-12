#[derive(Debug, Clone, PartialEq)]
struct Range {
    start: i64,
    end: i64,
}

#[derive(Debug)]
struct Mapping {
    source: Range,
    delta: i64,
}

#[derive(Debug)]
struct Map {
    mappings: Vec<Mapping>,
}

impl Map {
    fn apply(&self, range: &Range) -> Vec<Range> {
        let mut result = Vec::new();
        let mut overlap_source_ranges = Vec::new();

        for mapping in &self.mappings {
            let overlap_start = mapping.source.start.max(range.start);
            let overlap_end = mapping.source.end.min(range.end);
            if overlap_start <= overlap_end {
                overlap_source_ranges.push(Range {
                    start: overlap_start,
                    end: overlap_end,
                });
                result.push(Range {
                    start: overlap_start + mapping.delta,
                    end: overlap_end + mapping.delta,
                });
            }
        }

        if overlap_source_ranges.is_empty() {
            result.push(range.clone());
        } else {
            overlap_source_ranges.sort_unstable_by_key(|r| r.start);
            if overlap_source_ranges.len() == 1 {
                if overlap_source_ranges[0].start > range.start {
                    result.push(Range {
                        start: range.start,
                        end: overlap_source_ranges[0].start - 1,
                    });
                }
                if overlap_source_ranges[0].end < range.end {
                    result.push(Range {
                        start: overlap_source_ranges[0].end + 1,
                        end: range.end,
                    });
                }
            } else {
                for overlap_source_range in overlap_source_ranges.windows(2) {
                    if overlap_source_range[0].start > range.start {
                        result.push(Range {
                            start: range.start,
                            end: overlap_source_range[0].start - 1,
                        });
                    }
                    if overlap_source_range[0].end + 1 < overlap_source_range[1].start {
                        result.push(Range {
                            start: overlap_source_range[0].end + 1,
                            end: overlap_source_range[1].start - 1,
                        });
                    }
                }
            }
        }

        result
    }
}

pub struct Almanac {
    seeds: Vec<Range>,
    maps: Vec<Map>,
}

impl From<&str> for Almanac {
    fn from(s: &str) -> Self {
        let mut lines = s.lines();

        let mut seeds = Vec::new();
        let seeds_line = lines.next().unwrap().split_once(": ").unwrap().1;
        for pair in seeds_line.split_whitespace().collect::<Vec<&str>>().chunks(2) {
            let start = pair[0].parse::<i64>().unwrap();
            let end = start + pair[1].parse::<i64>().unwrap() - 1;
            seeds.push(Range { start, end });
        }

        let mut maps = Vec::new();
        let mut mappings = Vec::new();
        while let Some(line) = lines.next() {
            if line.contains("map") {
                if !mappings.is_empty() {
                    maps.push(Map { mappings });
                    mappings = Vec::new();
                }
            } else if !line.trim().is_empty() {
                let mut entries = line.split_whitespace();
                let destination_range_start = entries.next().unwrap().parse::<i64>().unwrap();
                let source_range_start = entries.next().unwrap().parse::<i64>().unwrap();
                let range_length = entries.next().unwrap().parse::<i64>().unwrap();

                let mapping = Mapping {
                    source: Range {
                        start: source_range_start,
                        end: source_range_start + range_length - 1,
                    },
                    delta: destination_range_start - source_range_start,
                };
                mappings.push(mapping);
            }
        }
        if !mappings.is_empty() {
            maps.push(Map { mappings });
        }

        Almanac {
            seeds,
            maps,
        }
    }
}

impl Almanac {
    pub fn find_lowest_location(&self) -> i64 {
        let mut lowest_location = i64::MAX;

        for seed in &self.seeds {
            let mut ranges = vec![seed.clone()];

            for map in &self.maps {
                let mut output_ranges = Vec::new();
                for range in &ranges {
                    output_ranges.append(&mut map.apply(range));
                }
                ranges = output_ranges;
            }

            for range in ranges {
                if range.start < lowest_location {
                    lowest_location = range.start;
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
    fn test_map_apply() {
        // full overlap
        let map = Map {
            mappings: vec![
                Mapping {
                    source: Range { start: 5, end: 10 },
                    delta: 2,
                },
            ],
        };
        let range = Range { start: 5, end: 10 };
        let result = map.apply(&range);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].start, 7);
        assert_eq!(result[0].end, 12);

        // partial overlap
        let map = Map {
            mappings: vec![
                Mapping {
                    source: Range { start: 5, end: 10 },
                    delta: 2,
                },
            ],
        };
        let range = Range { start: 7, end: 12 };
        let result = map.apply(&range);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].start, 9);
        assert_eq!(result[0].end, 12);
        assert_eq!(result[1].start, 11);
        assert_eq!(result[1].end, 12);

        // partial overlap across multiple mappings
        let map = Map {
            mappings: vec![
                Mapping {
                    source: Range { start: 5, end: 10 },
                    delta: 2,
                },
                Mapping {
                    source: Range { start: 15, end: 20 },
                    delta: 3,
                },
            ],
        };
        let range = Range { start: 7, end: 18 };
        let result = map.apply(&range);
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].start, 9);
        assert_eq!(result[0].end, 12);
        assert_eq!(result[1].start, 18);
        assert_eq!(result[1].end, 21);
        assert_eq!(result[2].start, 11);
        assert_eq!(result[2].end, 14);

        // no overlap
        let map = Map {
            mappings: vec![
                Mapping {
                    source: Range { start: 5, end: 10 },
                    delta: 2,
                },
            ],
        };
        let range = Range { start: 1, end: 4 };
        let result = map.apply(&range);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].start, 1);
        assert_eq!(result[0].end, 4);
    }

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
        assert_eq!(almanac.seeds[0].start, 79);
        assert_eq!(almanac.seeds[0].end, 92);
        assert_eq!(almanac.seeds[1].start, 55);
        assert_eq!(almanac.seeds[1].end, 67);
        assert_eq!(almanac.maps.len(), 2);
        assert_eq!(almanac.maps[0].mappings.len(), 2);
        assert_eq!(almanac.maps[0].mappings[0].source.start, 98);
        assert_eq!(almanac.maps[0].mappings[0].source.end, 99);
        assert_eq!(almanac.maps[0].mappings[0].delta, -48);
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
