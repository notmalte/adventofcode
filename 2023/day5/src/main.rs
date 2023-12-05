use itertools::Itertools;
use rayon::prelude::*;
use std::{env, fs};

const DEBUG: bool = false;

fn main() {
    let current_dir = env::current_dir().unwrap();

    let input_file = current_dir.join("day5/input.txt");

    println!("Reading input from: {}", input_file.display());

    let input = fs::read_to_string(input_file).unwrap();

    println!("Part 1: {}", part1(&input));

    println!("Part 2: {}", part2(&input));
}

#[derive(Debug)]
struct MapEntry {
    dst_start: usize,
    src_start: usize,
    range: usize,
}

#[derive(Debug)]
struct Map {
    entries: Vec<MapEntry>,
}

fn parse_seeds(seeds: &str) -> Vec<usize> {
    seeds
        .split_once(": ")
        .unwrap()
        .1
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect()
}

fn parse_map(section: &str) -> Map {
    let lines = section.lines().skip(1).collect::<Vec<_>>();

    let mut entries = vec![];

    for line in lines {
        let parts = line.split_whitespace().collect::<Vec<_>>();

        let dst_start = parts[0].parse::<usize>().unwrap();
        let src_start = parts[1].parse::<usize>().unwrap();
        let range = parts[2].parse::<usize>().unwrap();

        entries.push(MapEntry {
            dst_start,
            src_start,
            range,
        });
    }

    Map { entries }
}

fn parse_seeds_and_maps(input: &str) -> (Vec<usize>, Vec<Map>) {
    let sections = input.split("\n\n").collect::<Vec<_>>();

    let seeds = parse_seeds(sections[0].trim());

    let seed_to_soil_map = parse_map(sections[1].trim());
    let soil_to_fertilizer_map = parse_map(sections[2].trim());
    let fertilizer_to_water_map = parse_map(sections[3].trim());
    let water_to_light_map = parse_map(sections[4].trim());
    let light_to_temperature_map = parse_map(sections[5].trim());
    let temperature_to_humidity_map = parse_map(sections[6].trim());
    let humidity_to_location_map = parse_map(sections[7].trim());

    let maps = vec![
        seed_to_soil_map,
        soil_to_fertilizer_map,
        fertilizer_to_water_map,
        water_to_light_map,
        light_to_temperature_map,
        temperature_to_humidity_map,
        humidity_to_location_map,
    ];

    (seeds, maps)
}

fn traverse_maps(maps: &Vec<Map>, seed: usize) -> usize {
    let mut current = seed;

    for map in maps {
        for entry in &map.entries {
            let src_end = entry.src_start + entry.range - 1;

            if current >= entry.src_start && current <= src_end {
                current = entry.dst_start + (current - entry.src_start);

                break;
            }
        }
    }

    current
}

fn part1(input: &str) -> isize {
    let (seeds, maps) = parse_seeds_and_maps(input);

    seeds
        .iter()
        .map(|s| traverse_maps(&maps, *s))
        .min()
        .unwrap() as isize
}

fn part2(input: &str) -> isize {
    let (seeds, maps) = parse_seeds_and_maps(input);

    let tuples: Vec<(_, _)> = seeds.iter().cloned().tuples().collect();

    tuples
        .par_iter()
        .map(|(seed_start, seed_range)| {
            let min = (*seed_start..(seed_start + seed_range))
                .map(|s| traverse_maps(&maps, s))
                .min()
                .unwrap();

            if DEBUG {
                println!(
                    "Found min for seed range {}..{}: {}",
                    seed_start,
                    seed_start + seed_range,
                    min
                );
            }

            min
        })
        .min()
        .unwrap() as isize
}
