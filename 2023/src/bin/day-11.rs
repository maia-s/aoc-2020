use std::ops::Index;

use aoc_2023::{aoc, str_block};

const INPUT: &str = include_str!("day-11.txt");

#[allow(dead_code)]
const INPUT_EX: &str = str_block! {"
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
"};

aoc! {
    struct Day11 {
        galaxies: Vec<(usize, usize)>,
        exp_rows: Vec<usize>,
        exp_cols: Vec<usize>,
    }

    self(input = INPUT) {
        let mut galaxies = Vec::new();
        let map = Map(input.lines().enumerate().map(
            |(y, line)| {
                let line = line.as_bytes().to_owned();
                for x in line.iter().enumerate().filter(|(_, &b)| b == b'#').map(|(x, _)| x) {
                    galaxies.push((x, y));
                }
                line
            }
        ).collect());
        let exp_rows = map.rows().enumerate().filter_map(
            |(i, row)| row.iter().all(|&c| c == b'.').then_some(i)
        ).collect();
        let exp_cols = map.cols().enumerate().filter_map(
            |(i, col)| col.iter().all(|&c| c == b'.').then_some(i)
        ).collect();
        Ok(Self {
            galaxies,
            exp_rows,
            exp_cols,
        })
    }

    part1 usize {
        Ok(self.dist_all(1))
    }

    part2 usize {
        Ok(self.dist_all(999_999))
    }

    test day11_example(INPUT_EX, 374);
    test day11(INPUT, 9536038, 447744640566);
}

impl Day11 {
    fn dist_all(&self, exp: usize) -> usize {
        self.galaxies[..self.galaxies.len() - 1]
            .iter()
            .enumerate()
            .map(|(i, &ga)| {
                self.galaxies[i + 1..]
                    .iter()
                    .map(|&gb| self.dist(ga, gb, exp))
                    .sum::<usize>()
            })
            .sum()
    }

    fn dist(&self, (ax, ay): (usize, usize), (bx, by): (usize, usize), exp: usize) -> usize {
        self.dist_part(ax, bx, &self.exp_cols, exp) + self.dist_part(ay, by, &self.exp_rows, exp)
    }

    fn dist_part(&self, a: usize, b: usize, exps: &[usize], exp: usize) -> usize {
        let (a, b) = (a.min(b), a.max(b));
        let mut d = b - a;
        for i in exps.iter() {
            if (a..b).contains(i) {
                d += exp;
            }
        }
        d
    }
}

struct Map(Vec<Vec<u8>>);

impl Index<(usize, usize)> for Map {
    type Output = u8;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.0[index.1][index.0]
    }
}

impl Map {
    fn rows(&self) -> impl Iterator<Item = &Vec<u8>> {
        self.0.iter()
    }

    fn cols(&self) -> impl Iterator<Item = Cols> {
        let mut col = 0;
        let col_len = self.0[0].len();
        std::iter::from_fn(move || {
            if col < col_len {
                let item = Cols { map: self, col };
                col += 1;
                Some(item)
            } else {
                None
            }
        })
    }
}

struct Cols<'a> {
    map: &'a Map,
    col: usize,
}

impl<'a> Cols<'a> {
    fn iter(&self) -> impl Iterator<Item = &'a u8> {
        Col {
            map: self.map,
            col: self.col,
            row: 0,
        }
    }
}

struct Col<'a> {
    map: &'a Map,
    col: usize,
    row: usize,
}

impl<'a> Iterator for Col<'a> {
    type Item = &'a u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.row < self.map.0.len() {
            let item = &self.map[(self.col, self.row)];
            self.row += 1;
            Some(item)
        } else {
            None
        }
    }
}