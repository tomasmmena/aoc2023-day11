use std::env;
use std::fs;
use std::io::{self, BufRead};

#[derive(PartialEq, Clone)]
enum Space {
    Empty,
    Galaxy
}

struct CompressedSpace {
    x_factors: Vec<usize>,
    y_factors: Vec<usize>,
    space: Vec<Vec<Space>>
}

impl CompressedSpace {
    fn parse(path: String) -> Self {
        let space: Vec<Vec<Space>> = parse_space(path);
        CompressedSpace {
            x_factors: vec![1; space[0].len()],
            y_factors: vec![1; space.len()],
            space: space
        }
    }

    fn set_empty_factor(&mut self, distance: usize) {
        for (i, row) in self.space.iter().enumerate() {
            if row.iter().all(|s| *s == Space::Empty) { self.y_factors[i] = distance }
        }
        for j in 0..self.space[0].len() {
            if (0..self.space.len()).all(|i| self.space[i][j] == Space::Empty) { self.x_factors[j] = distance }
        }
    }

    fn manhattan_distance(&self, one: &(usize, usize), other: &(usize, usize)) -> usize {
        self.y_factors[one.0.min(other.0)..one.0.max(other.0)].iter().sum::<usize>() + self.x_factors[one.1.min(other.1)..one.1.max(other.1)].iter().sum::<usize>()
    }

    fn total_distance(&self) -> usize {
        let galaxies = find_galaxies(&self.space);
        let mut total_distance: usize = 0;
        for one_index in 0..galaxies.len() {
            for other_index in one_index..galaxies.len() {
                total_distance += self.manhattan_distance(&galaxies[one_index], &galaxies[other_index]) 
            }
        }
        total_distance
    }

}


/// This function expands a matrix of space in place.
fn expand(space: &mut Vec<Vec<Space>>) {
    let mut i: usize = 0;
    while i < space.len() {
        if space[i].iter().all(|s| *s == Space::Empty) {
            space.insert(i + 1, vec![Space::Empty; space[i].len()]);
            i += 1; // skip newly inserted space
        }
        i += 1;
    }

    let mut j: usize = 0;
    while j < space[0].len() {
        if (0..space.len()).all(|i| space[i][j] == Space::Empty) {
            for row in space.iter_mut() {
                row.insert(j + 1, Space::Empty);
            }
            j += 1; // skip newly inserted space
        }
        j += 1;
    }
}

fn find_galaxies(space: &Vec<Vec<Space>>) -> Vec<(usize, usize)> {
    space
        .iter()
        .enumerate()
        .flat_map(|(i, row)| row
            .iter()
            .enumerate()
            .filter(|(_j, s)| **s == Space::Galaxy)
            .map(move |(j, _s)| (i, j)))
        .collect()
}

/// Manhattan distance in 2D space.
fn manhattan_distance(one: &(usize, usize), other: &(usize, usize)) -> usize {
    one.0.max(other.0) - one.0.min(other.0) + one.1.max(other.1) - one.1.min(other.1)
}

fn parse_space(path: String) -> Vec<Vec<Space>> {
    let space: Vec<Vec<Space>> = io::BufReader::new(
        fs::File::open(path).expect("Could not open file!"))
        .lines()
        .map(|line| {
            let text = line.expect("Falied to read line!");
            text.chars().map(|c| {
                match c {
                    '.' => Space::Empty,
                    '#' => Space::Galaxy,
                    _ => panic!("Invalic character!")
                }
            }).collect::<Vec<Space>>()
        })
        .collect();
    space
}


fn main() {
    let path = env::args().nth(1).expect("Missing required parameter path!");

    let mut space = parse_space(path.clone());

    println!("Space of {} x {}.", space[0].len(), space.len());
    expand(&mut space);
    println!("Space of {} x {}.", space[0].len(), space.len());

    let galaxies = find_galaxies(&space);
    println!("{} galaxies found.", galaxies.len());

    let mut total_distance: usize = 0;
    for one_index in 0..galaxies.len() {
        for other_index in one_index..galaxies.len() {
            total_distance += manhattan_distance(&galaxies[one_index], &galaxies[other_index]) 
        }
    }
    println!("Total distance (2): {}", total_distance);

    let mut c_space = CompressedSpace::parse(path);
    c_space.set_empty_factor(1_000_000);
    println!("Total distance (1M): {}", c_space.total_distance());

}

#[cfg(test)]
mod tests {
    use crate::{parse_space, manhattan_distance, find_galaxies, expand};

    #[test]
    fn test_expand() {
        let mut space = parse_space(String::from("test/space1.txt"));
        assert_eq!(space.len(), 10);
        assert_eq!(space[0].len(), 10);

        expand(&mut space);
        assert_eq!(space.len(), 12);
        assert_eq!(space[0].len(), 13);
    }

    #[test]
    fn test_find_galaxies() {
        let space = parse_space(String::from("test/space1.txt"));
        let galaxies = find_galaxies(&space);
        assert_eq!(galaxies.len(), 9);
        assert!(galaxies.contains(&(0, 3)));
        assert!(galaxies.contains(&(4, 6)));
        assert!(!galaxies.contains(&(5, 5)));
    }

    #[test]
    fn test_manhattan_distance() {
        assert_eq!(manhattan_distance(&(0, 0), &(0, 0)), 0);
        assert_eq!(manhattan_distance(&(0, 0), &(10, 10)), 20);
        assert_eq!(manhattan_distance(&(7, 19), &(12, 10)), 14);
    }
}
