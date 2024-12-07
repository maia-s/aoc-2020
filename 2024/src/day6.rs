use crate::{Conf, Input};
use core::{
    mem::{self, transmute},
    ops::{Index, IndexMut},
};
use str_block::str_block;

pub const INPUT: Conf = Conf::new(
    Input::FileHash("a820ce3b6b89ccc820f0639477e8588fd348b294933f8868232d111dc88f65be"),
    4696,
    1443,
);

pub const EX: Conf = Conf::new(
    Input::Str(str_block! {"
        ....#.....
        .........#
        ..........
        ..#.......
        .......#..
        ..........
        .#..^.....
        ........#.
        #.........
        ......#...
    "}),
    41,
    6,
);

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
struct B8(u8);

struct Map<T> {
    map: Vec<T>,
    width: u32,
    gx: i32,
    gy: i32,
    dir: Dir,
    result: u32,
}

impl<T> Map<T> {
    #[inline(always)]
    fn height(&self) -> u32 {
        self.map.len() as u32 / self.width
    }

    #[inline(always)]
    fn coords_to_index_unchecked(&self, (x, y): (i32, i32)) -> usize {
        (y as u32 * self.width + x as u32) as usize
    }

    #[inline(always)]
    fn get(&self, (x, y): (i32, i32)) -> Option<&T> {
        ((x as u32) < self.width && (y as u32) < self.height())
            .then(|| unsafe { self.get_unchecked((x, y)) })
    }

    #[inline(always)]
    unsafe fn get_unchecked(&self, c: (i32, i32)) -> &T {
        unsafe { self.map.get_unchecked(self.coords_to_index_unchecked(c)) }
    }

    #[inline(always)]
    unsafe fn get_unchecked_mut(&mut self, c: (i32, i32)) -> &mut T {
        unsafe {
            let i = self.coords_to_index_unchecked(c);
            self.map.get_unchecked_mut(i)
        }
    }
}

impl Map<u8> {
    fn new(input: &str) -> Self {
        let mut lines = input.as_bytes().trim_ascii_end().split(|&b| b == b'\n');
        let line = lines.next().unwrap();
        let width = line.len() as _;
        let mut map = Vec::with_capacity((width * width) as usize);
        map.extend_from_slice(line);
        let mut gx = 0;
        let mut gy = 1;
        for line in lines.by_ref() {
            map.extend_from_slice(line);
            if let Some(start) = line.iter().position(|&b| b == b'^') {
                gx = start as _;
                break;
            }
            gy += 1;
        }
        for line in lines {
            map.extend_from_slice(line);
        }
        Self {
            map,
            width,
            gx,
            gy,
            dir: Dir::N,
            result: 0,
        }
    }

    fn step(&mut self) -> bool {
        let (gx, gy) = (self.gx, self.gy);
        if self[(gx, gy)] != b'X' {
            self[(gx, gy)] = b'X';
            self.result += 1;
        }
        loop {
            let (dx, dy) = self.dir.delta();
            let next = (gx + dx, gy + dy);
            if let Some(&tile) = self.get(next) {
                if tile != b'#' {
                    (self.gx, self.gy) = next;
                    return true;
                }
                self.dir = self.dir.rotate_cw();
            } else {
                return false;
            }
        }
    }
}

impl Map<B8> {
    fn new(input: &str) -> Self {
        let mut lines = input.as_bytes().trim_ascii_end().split(|&b| b == b'\n');
        let line = lines.next().unwrap();
        let width = line.len() as _;
        let mut map = Vec::with_capacity((width * width) as usize);
        map.extend(line.iter().map(|&b| B8((b == b'#') as u8 * u8::MAX)));
        let mut gx = 0;
        let mut gy = 1;
        let mut got_start = false;
        for line in lines.by_ref() {
            map.extend(line.iter().enumerate().map(|(x, &b)| {
                B8(match b {
                    b'#' => u8::MAX,
                    b'^' => {
                        got_start = true;
                        gx = x as i32;
                        0
                    }
                    _ => 0,
                })
            }));
            if got_start {
                break;
            }
            gy += 1;
        }
        for line in lines {
            map.extend(line.iter().map(|&b| B8((b == b'#') as u8 * u8::MAX)));
        }
        Self {
            map,
            width,
            gx,
            gy,
            dir: Dir::N,
            result: 0,
        }
    }

    fn loops(&self, m: &mut [B8]) -> bool {
        m.copy_from_slice(&self.map);
        let height = self.height();
        let mut dir = self.dir;
        let (mut dx, mut dy) = dir.delta();
        let (mut gx, mut gy) = (self.gx, self.gy);
        while (gx as u32) < self.width && (gy as u32) < height {
            let gtile = unsafe { m.get_unchecked_mut(self.coords_to_index_unchecked((gx, gy))) };
            if (gtile.0 as i8) < 0 {
                gx -= dx;
                gy -= dy;
                dir = dir.rotate_cw();
                (dx, dy) = dir.delta();
            } else if (gtile.0 & dir.bit()) != 0 {
                return true;
            } else {
                gtile.0 |= dir.bit();
                gx += dx;
                gy += dy;
            }
        }
        false
    }
}

impl<T> Index<(i32, i32)> for Map<T> {
    type Output = T;

    #[inline(always)]
    fn index(&self, (x, y): (i32, i32)) -> &Self::Output {
        &self.map[self.coords_to_index_unchecked((x, y))]
    }
}

impl<T> IndexMut<(i32, i32)> for Map<T> {
    #[inline(always)]
    fn index_mut(&mut self, (x, y): (i32, i32)) -> &mut Self::Output {
        let i = self.coords_to_index_unchecked((x, y));
        &mut self.map[i]
    }
}

impl<T> Index<(u32, u32)> for Map<T> {
    type Output = T;

    #[inline(always)]
    fn index(&self, (x, y): (u32, u32)) -> &Self::Output {
        &self.map[self.coords_to_index_unchecked((x as i32, y as i32))]
    }
}

impl<T> IndexMut<(u32, u32)> for Map<T> {
    #[inline(always)]
    fn index_mut(&mut self, (x, y): (u32, u32)) -> &mut Self::Output {
        let i = self.coords_to_index_unchecked((x as i32, y as i32));
        &mut self.map[i]
    }
}

#[repr(u8)]
#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq)]
enum Dir {
    N,
    E,
    S,
    W,
}

impl Dir {
    #[inline(always)]
    const fn rotate_cw(self) -> Self {
        unsafe { transmute((self as u8 + 1) & 3) }
    }

    #[inline(always)]
    const fn delta(self) -> (i32, i32) {
        [(0, -1), (1, 0), (0, 1), (-1, 0)][self as u8 as usize]
    }

    #[inline(always)]
    const fn bit(self) -> u8 {
        1 << self as u8
    }
}

pub fn part1(input: &str) -> u32 {
    let mut map = Map::<u8>::new(input);
    while map.step() {}
    map.result
}

pub fn part2(input: &str) -> u32 {
    let mut map = Map::<B8>::new(input);
    let mut scratch = vec![B8(0); map.map.len()];
    let mut candidates = 0;

    let height = map.height();
    let (mut dx, mut dy) = map.dir.delta();
    loop {
        if (unsafe { map.get_unchecked_mut((map.gx, map.gy)) }.0 as i8) < 0 {
            map.gx -= dx;
            map.gy -= dy;
            map.dir = map.dir.rotate_cw();
            (dx, dy) = map.dir.delta();
        } else {
            unsafe { map.get_unchecked_mut((map.gx, map.gy)) }.0 |= map.dir.bit();
            map.gx += dx;
            map.gy += dy;
            if (map.gx as u32) < map.width && (map.gy as u32) < height {
                let tile = mem::replace(
                    unsafe { map.get_unchecked_mut((map.gx, map.gy)) },
                    B8(u8::MAX),
                );
                candidates += (tile.0 == 0 && map.loops(&mut scratch)) as u32;
                unsafe { map.get_unchecked_mut((map.gx, map.gy)) }.0 = tile.0;
            } else {
                break;
            }
        }
    }

    candidates
}
