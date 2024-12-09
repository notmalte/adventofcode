use crate::{malformed, Answer};

pub fn run(input: &str) -> Answer {
    Answer {
        part1: part1(input),
        part2: part2(input),
    }
}

fn part1(input: &str) -> String {
    let disk = Disk::from(input);
    let block_count = disk.blocks.iter().filter(|b| b.is_some()).count();

    let gaps_to_fill = disk
        .blocks
        .iter()
        .enumerate()
        .filter_map(|(i, b)| (b.is_none() && i < block_count).then_some(i));

    let fillers = disk
        .blocks
        .iter()
        .enumerate()
        .rev()
        .filter_map(|(i, b)| b.map(|b| (i, b)));

    let mut defrag = disk.clone();
    for (gap, (i, b)) in gaps_to_fill.zip(fillers) {
        defrag.blocks[gap] = Some(b);
        defrag.blocks[i] = None;
    }

    defrag.checksum()
}

fn part2(input: &str) -> String {
    let mut disk = Disk::from(input);

    let max_id = disk.blocks.iter().filter_map(|b| *b).max().unwrap();

    for id in (0..=max_id).rev() {
        let Some((span_start, span_end)) = disk.find_span(id) else {
            continue;
        };

        let length = span_end - span_start;

        let Some((gap_start, gap_end)) = disk.find_gap(length) else {
            continue;
        };

        if gap_end > span_start {
            continue;
        }

        for i in 0..(span_end - span_start) {
            disk.blocks[span_start + i] = None;
            disk.blocks[gap_start + i] = Some(id);
        }
    }

    disk.checksum()
}

#[derive(Debug, Clone)]
struct Disk {
    blocks: Vec<Option<usize>>,
}

impl Disk {
    fn from(input: &str) -> Self {
        let mut blocks = Vec::new();

        for (i, c) in input.trim().chars().enumerate() {
            let n = c.to_digit(10).unwrap_or_else(|| malformed("2024", "08")) as usize;
            let id = (i % 2 == 0).then_some(i / 2);

            blocks.append(&mut vec![id; n]);
        }

        Self { blocks }
    }

    fn checksum(&self) -> String {
        self.blocks
            .iter()
            .enumerate()
            .filter_map(|(i, b)| b.map(|id| i * id))
            .sum::<usize>()
            .to_string()
    }

    fn find_span(&self, id: usize) -> Option<(usize, usize)> {
        let mut iter = self.blocks.iter().enumerate();

        if let Some(start) = iter.find(|(_, b)| **b == Some(id)).map(|(i, _)| i) {
            let end = iter
                .find(|(_, b)| **b != Some(id))
                .map_or(self.blocks.len(), |(end, _)| end);

            Some((start, end))
        } else {
            None
        }
    }

    fn find_gap(&self, length: usize) -> Option<(usize, usize)> {
        let mut iter = self.blocks.iter().enumerate();

        loop {
            if let Some(start) = iter.find(|(_, b)| b.is_none()).map(|(i, _)| i) {
                let end = iter
                    .find(|(_, b)| b.is_some())
                    .map_or(self.blocks.len(), |(end, _)| end);

                if end - start >= length {
                    return Some((start, end));
                }
            } else {
                return None;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("2333133121414131402"), "1928");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("2333133121414131402"), "2858");
    }
}
