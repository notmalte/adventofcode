use std::{
    collections::{HashMap, HashSet},
    env, fs,
};

fn main() {
    let current_dir = env::current_dir().unwrap();

    let input_file = current_dir.join("day7/input.txt");

    println!("Reading input from: {}", input_file.display());

    let input = fs::read_to_string(input_file).unwrap();

    println!("Input: {}", input);

    println!("Part 1: {}", part1(&input));

    println!("Part 2: {}", part2(&input));
}

fn build_absolute_directories_and_files(input: &str) -> (HashSet<String>, HashMap<String, usize>) {
    let mut current_path: Vec<String> = vec![];

    let mut directories: HashSet<String> = HashSet::new();
    directories.insert("".to_string());

    let mut files: HashMap<String, usize> = HashMap::new();

    for line in input.lines() {
        if line.starts_with('$') {
            let command = line.strip_prefix("$ ").unwrap();

            if command.starts_with("cd ") {
                let path = command.strip_prefix("cd ").unwrap();

                if path == "/" {
                    current_path = vec![];
                } else if path == ".." {
                    current_path.pop();
                } else {
                    current_path.push(path.to_string());

                    let new_path = format!("/{}", current_path.join("/"));

                    directories.insert(new_path);
                }
            }
        } else if line.starts_with("dir") {
            let dir_path = line.strip_prefix("dir ").unwrap();

            let mut path_vec = current_path.clone();
            path_vec.push(dir_path.to_string());

            let path = format!("/{}", path_vec.join("/"));

            directories.insert(path);
        } else {
            let (size, name) = line.split_once(' ').unwrap();

            let mut path_vec = current_path.clone();
            path_vec.push(name.to_string());

            let path = format!("/{}", path_vec.join("/"));

            if !files.contains_key(&path) {
                files.insert(path.clone(), size.parse().unwrap());
            }
        }
    }

    (directories, files)
}

fn part1(input: &str) -> usize {
    let (directories, files) = build_absolute_directories_and_files(input);

    let mut total_size_of_all_at_most_100k = 0;

    for dir in directories {
        let mut total_size = 0;

        let dir_trailing_slash = format!("{}/", dir);

        for (path, size) in &files {
            if path.starts_with(&dir_trailing_slash) {
                total_size += size;
            }
        }

        if total_size <= 100_000 {
            total_size_of_all_at_most_100k += total_size;
        }
    }

    total_size_of_all_at_most_100k
}

fn part2(input: &str) -> usize {
    let (directories, files) = build_absolute_directories_and_files(input);

    let mut total_size = 0;
    for size in files.values() {
        total_size += size;
    }

    let disk_size = 70_000_000;

    let want_free_space = 30_000_000;

    let free_space = disk_size - total_size;

    let need_to_remove = want_free_space - free_space;

    let mut min_to_remove = Result::Err("Impossible to remove enough space");

    for dir in directories {
        let mut total_size = 0;

        let dir_trailing_slash = format!("{}/", dir);

        for (path, size) in &files {
            if path.starts_with(&dir_trailing_slash) {
                total_size += size;
            }
        }

        if total_size >= need_to_remove
            && (min_to_remove.is_err() || total_size < min_to_remove.unwrap())
        {
            min_to_remove = Result::Ok(total_size);
        }
    }

    min_to_remove.unwrap()
}
