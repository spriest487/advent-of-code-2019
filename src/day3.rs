mod point;
use point::*;

#[derive(Copy, Clone, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn parse(c: char) -> Self {
        match c {
            'U' => Direction::Up,
            'D' => Direction::Down,
            'L' => Direction::Left,
            'R' => Direction::Right,
            bad => panic!("bad direction char: {}", bad),
        }
    }

    fn unit(&self) -> Point {
        match &self {
            Direction::Up => Point { x: 0, y: -1 },
            Direction::Down => Point { x: 0, y: 1 },
            Direction::Left => Point { x: -1, y: 0 },
            Direction::Right => Point { x: 1, y: 0 },
        }
    }

    fn vertical(&self) -> bool {
        match self {
            Direction::Up | Direction::Down => true,
            _ => false,
        }
    }
}

#[derive(Debug)]
struct Segment {
    origin: Point,
    dir: Direction,
    dist: i64,
}

impl Segment {
    fn intersection(&self, other: &Segment) -> Option<Point> {
        if self.dir.vertical() == other.dir.vertical() {
            return None;
        }

        let dist_to = i64::min(
            (self.origin - other.end()).manhattan_len(),
            (self.origin - other.origin).manhattan_len(),
        );

        if dist_to > self.dist + 1 {
            return None;
        }

        for offset in 1..=self.dist {
            let point = self.origin + self.dir.unit() * offset;

            for other_offset in 1..=other.dist {
                if point == other.origin + other.dir.unit() * other_offset {
                    return Some(point);
                }
            }
        }

        None
    }

    fn end(&self) -> Point {
        self.origin + self.dir.unit() * self.dist
    }
}

fn read_path(path_str: &str) -> Vec<Segment> {
    let mut path = Vec::new();
    let mut origin = Point::zero();

    for part in path_str.split(",") {
        let dir = Direction::parse(part.chars().next().unwrap());
        let dist = part[1..].parse().unwrap();

        path.push(Segment { origin, dir, dist });
        origin += dir.unit() * dist;
    }

    path
}

fn find_time(wire: &[Segment], point: &Point) -> usize {
    let points: Vec<_> = wire.iter()
        .flat_map(|seg| {
            (1..=seg.dist).map(move |offset| {
                seg.origin + seg.dir.unit() * offset
            })
        })
        .collect();

    points.iter().position(|p| *p == *point).unwrap() + 1
}

fn intersection_closest_by<V: Ord, F: Fn(&Point) -> V>(
    a: &[Segment],
    b: &[Segment],
    f: F
) -> (Point, V) {
    let intersections = a
        .iter()
        .flat_map(move |seg| {
            b.iter().filter_map(move |b_seg| seg.intersection(b_seg))
        });

    intersections
        .map(|i| (i, f(&i)))
        .min_by(|(_, val_a), (_, val_b)| val_a.cmp(val_b))
        .unwrap()
}

fn main() {
    let input = include_str!("day3.txt");
    let mut lines = input.lines();
    let wire1 = read_path(lines.next().unwrap());
    let wire2 = read_path(lines.next().unwrap());

    let (intersection, dist) = intersection_closest_by(&wire1, &wire2, |i| {
        Point::zero().manhattan_dist(i)
    });
    println!("closest intersection by manhattan dist @ {}: distance {}", intersection, dist);

    let (intersection, dist) = intersection_closest_by(&wire1, &wire2, |i| {
        find_time(&wire1, i) + find_time(&wire2, i)
    });

    println!("closest intersection by time @ {}: time {}", intersection, dist);
}
