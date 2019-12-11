mod point;
use point::*;
use num::integer::gcd;
use std::collections::{HashSet, HashMap};
use nalgebra::{self as na};

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

    fn contains(&self, point: &Point) -> bool {
        point.x >= 0
            && point.x < self.width
            && point.y >= 0
            && point.y < self.height
    }

    fn rays_to_visible(&self, from: &Point) -> HashSet<Point> {
        let mut rays = HashSet::new();
        for other in self.asteroids.iter() {
            if *from == *other {
                continue;
            }

            let diff = *other - *from;
            let gcd = gcd(diff.x, diff.y);
            let ray = Point::new(diff.x / gcd, diff.y / gcd);

            rays.insert(ray);
        }
        rays
    }

    fn fire_laser(&mut self, from: &Point, times: usize) -> Vec<Point> {
        let mut vaporized_asteroids = Vec::new();
        let mut rays = Vec::new();

        'laser: loop {
            rays.clear();
            rays.extend(self.rays_to_visible(from).into_iter());
            rays.sort_by(|a, b| {
                ray_angle(a).partial_cmp(&ray_angle(b)).unwrap()
            });

            for ray in &rays {
                // vaporize the first asteroid this ray intersects
                let mut next = *from + *ray;
                'shot: while self.contains(&next) {
                    if let Some(vaporized_index) = self.asteroids.iter().position(|a| *a == next) {
                        let asteroid = self.asteroids.remove(vaporized_index);
                        vaporized_asteroids.push(asteroid);

                        if vaporized_asteroids.len() == times {
                            break 'laser;
                        }
                        break 'shot;
                    }

                    next += *ray;
                }
            }
        };

        vaporized_asteroids
    }
}

fn ray_angle(ray: &Point) -> f64 {
    let up = na::Vector3::new(0.0, -1.0, 0.0);
    let plane = na::Vector3::new(0.0, 0.0, 1.0);

    let ray_dir = na::Vector3::new(ray.x as f64, ray.y as f64, 0.0).normalize();

    let mut angle = ray_dir.angle(&up).to_degrees();
    let normal = up.cross(&ray_dir);
    if normal.dot(&plane) < 0.0 {
        angle = 360.0 - angle;
    }

    angle
}

fn main() {
    let input = include_str!("day10.txt");
    let map = Map::new(input);

    let mut rays = HashMap::new();
    for a in map.asteroids.iter() {
        let visible = map.rays_to_visible(a);
        rays.entry(*a)
            .or_insert_with(HashSet::new)
            .extend(visible);
    }

    // best asteroid is the one that can see most other asteroids
    let (best_asteroid, num_visible) = rays.iter()
        .map(|(ast, rays)| (*ast, rays.len()))
        .max_by_key(|(_ast, ray_count)| *ray_count)
        .unwrap();

    println!("asteroid with most visible ({}): {}", best_asteroid, num_visible);

    let mut map = map;

    let vaporized = map.fire_laser(&best_asteroid, 200);
    let last_hit = vaporized.last().unwrap();
    let output_val = last_hit.x * 100 + last_hit.y;

    println!("last hit asteroid after {} shots: {} ({})", vaporized.len(), last_hit, output_val);
}