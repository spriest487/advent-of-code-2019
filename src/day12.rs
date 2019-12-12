use regex::Regex;
use std::mem;
use num::integer::lcm;
mod vector3;
use vector3::*;
use std::collections::HashSet;

#[derive(Debug, Clone, Eq, PartialEq)]
struct Moon {
    pos: Vector3,
    vel: Vector3,
}

impl Moon {
    fn total_energy(&self) -> i64 {
        let potential = self.pos.x.abs() + self.pos.y.abs() + self.pos.z.abs();
        let kinetic = self.vel.x.abs() + self.vel.y.abs() + self.vel.z.abs();

        potential * kinetic
    }
}

fn apply_gravity<AxisFunc: Fn(&mut Vector3) -> &mut i64>(
    moon_a: &mut Moon,
    moon_b: &mut Moon,
    axis: AxisFunc
) {
    let a = axis(&mut moon_a.pos);
    let b = axis(&mut moon_b.pos);
    let diff = *b - *a;
    let signum = diff.signum();

    *axis(&mut moon_a.vel) += signum;
    *axis(&mut moon_b.vel) -= signum;
}

fn apply_gravity_between<AxisFunc: Copy + Fn(&mut Vector3) -> &mut i64>(
    moons: &mut [Moon],
    axis: AxisFunc
) {
    let moon_count = moons.len();
    let pairs = (0..moon_count)
        .flat_map(move |a_index| (a_index..moon_count)
            .filter_map(move |b_index| if a_index != b_index {
                Some((a_index, b_index))
            } else {
                None
            }));

    for (mut a_index, mut b_index) in pairs {
        if a_index > b_index {
            mem::swap(&mut a_index, &mut b_index)
        }
        let (moons_l, moons_r) = moons.split_at_mut(b_index);
        let moon_a = &mut moons_l[a_index];
        let moon_b = &mut moons_r[0];

        apply_gravity(moon_a, moon_b, axis);
    }
}

fn apply_velocity(moons: &mut [Moon]) {
    for moon in moons {
        moon.pos += moon.vel;
    }
}

fn find_cycle<AxisFunc: Copy + Fn(&mut Vector3) -> &mut i64>(
    mut moons: Vec<Moon>,
    axis: AxisFunc
) -> usize {
    let mut vals = HashSet::new();
    for i in 0.. {
        apply_gravity_between(&mut moons, axis);
        apply_velocity(&mut moons);

        let mut current = Vec::with_capacity(moons.len() * 2);
        for moon in &mut moons {
            let pos = *axis(&mut moon.pos);
            let vel = *axis(&mut moon.vel);

            current.push(pos);
            current.push(vel);
        }

        if !vals.insert(current) {
            return i;
        }
    }
    unreachable!()
}

fn main() {
    let input = include_str!("day12.txt");
    let vector_pattern = Regex::new("^<x=(.+), y=(.+), z=(.+)>$").unwrap();

    let moons: Vec<_> = input.lines()
        .map(|line| {
            let captures = vector_pattern.captures(line).unwrap();
            let x = captures[1].parse().unwrap();
            let y = captures[2].parse().unwrap();
            let z = captures[3].parse().unwrap();

            let pos = Vector3::new(x, y, z);
            let vel = Vector3::zero();
            Moon { pos, vel }
        })
        .collect();

    let mut moons_1000_steps = moons.clone();
    for i in 1..=1000 {
        apply_gravity_between(&mut moons_1000_steps, |v| &mut v.x);
        apply_gravity_between(&mut moons_1000_steps, |v| &mut v.y);
        apply_gravity_between(&mut moons_1000_steps, |v| &mut v.z);
        apply_velocity(&mut moons_1000_steps);

        println!("after {} steps", i);
        for moon in &moons_1000_steps {
            println!("pos={:?}, vel={:?}", moon.pos, moon.vel);
        }
        println!();
    }

    // should output 8742
    let total_energy: i64 = moons_1000_steps.iter().map(|m| m.total_energy()).sum();
    println!("total energy: {}", total_energy);

    let x_cycle = find_cycle(moons.clone(), |v| &mut v.x);
    println!("x cycles after {}", x_cycle);
    let y_cycle = find_cycle(moons.clone(), |v| &mut v.y);
    println!("y cycles after {}", y_cycle);
    let z_cycle = find_cycle(moons.clone(), |v| &mut v.z);
    println!("z cycles after {}", z_cycle);

    println!("cycles after {}", lcm(lcm(x_cycle, y_cycle), z_cycle));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn gravity_works() {
        let mut moon_a = Moon {
            pos: Vector3::new(-1, 1, 1),
            vel: Vector3::zero(),
        };

        let mut moon_b = Moon {
            pos: Vector3::new(1, -1, 1),
            vel: Vector3::zero(),
        };

        apply_gravity(&mut moon_a, &mut moon_b, |v| &mut v.x);
        apply_gravity(&mut moon_a, &mut moon_b, |v| &mut v.y);
        apply_gravity(&mut moon_a, &mut moon_b, |v| &mut v.z);

        assert_eq!(moon_a.vel, Vector3::new(1, -1, 0));
        assert_eq!(moon_b.vel, Vector3::new(-1, 1, 0));
    }
}