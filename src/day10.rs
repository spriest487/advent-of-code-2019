mod point;
use point::*;
use num::integer::gcd;
use std::collections::{HashSet, HashMap};
use itertools::Itertools;

#[allow(unused)]
struct Map {
    width: i64,
    height: i64,

    asteroids: Vec<Point>,
}

impl Map {
    fn new(input: &str) -> Self {
        let mut width = 0;
        let mut y = 0;
        let mut asteroids = Vec::new();
        for line in input.lines() {
            width = line.len();
            for (x, char) in line.chars().enumerate() {
                if char == '#' {
                    asteroids.push(Point::new(x as _, y));
                }
            }

            y += 1;
        }

        Self {
            width: width as i64,
            height: y,
            asteroids,
        }
    }
}

fn main() {
    let input = include_str!("day10.txt");
    let map = Map::new(input);

    let mut rays = HashMap::new();
    for pair in map.asteroids.iter().permutations(2) {
        let diff = *pair[0] - *pair[1];
        let gcd = gcd(diff.x, diff.y);
        let ray = Point::new(diff.x / gcd, diff.y / gcd);

        rays.entry(*pair[0]).or_insert_with(HashSet::new).insert(ray);
    }

    // best asteroid is the one that can see most other asteroids
    let (best_asteroid, num_visible) = rays.iter()
        .map(|(ast, rays)| (*ast, rays.len()))
        .max_by_key(|(_ast, ray_count)| *ray_count)
        .unwrap();

    println!("asteroid with most visible ({}): {}", best_asteroid, num_visible);
}