mod intcode;
mod point;
use intcode::*;
use point::*;
use std::collections::HashMap;

const UP: Word = 0;
const RIGHT: Word = 1;
const DOWN: Word = 2;
const LEFT: Word = 3;

const TURN_LEFT: Word = 0;
const TURN_RIGHT: Word = 1;

const BLACK: Word = 0;
const WHITE: Word = 1;

fn paint(code: Vec<Word>, starting_tile: Word) -> HashMap<Point, Word> {
    let mut robot = Computer::new(code);

    let mut tiles = HashMap::new();
    tiles.insert(Point::zero(), starting_tile);

    let mut current_pos = Point::zero();
    let mut current_facing = UP;

    while let Err(ExecError::InputBlocked) = robot.run() {
        for command in robot.out_buf.windows(2) {
            let paint = command[0];
            assert!(paint == BLACK || paint == WHITE);
            tiles.insert(current_pos, paint);

            current_facing = match command[1] {
                TURN_LEFT => match current_facing {
                    UP => LEFT,
                    facing => facing - 1,
                },
                TURN_RIGHT => match current_facing {
                    LEFT => UP,
                    facing => facing + 1,
                }
                bad => panic!("invalid facing: {}", bad),
            };

            match current_facing {
                UP => current_pos.y -= 1,
                RIGHT => current_pos.x += 1,
                DOWN => current_pos.y += 1,
                LEFT => current_pos.x -= 1,
                _ => unreachable!(),
            }
        }
        robot.out_buf.clear();

        let current_color = tiles.get(&current_pos).cloned().unwrap_or(BLACK);
        robot.in_buf.push(current_color);
    }

    tiles
}

fn main() {
    let input = include_str!("day11.txt");
    let code = from_str(input);

    let panels_from_black = paint(code.clone(), BLACK);
    println!("painted {} panels", panels_from_black.len());

    let panels_from_white = paint(code.clone(), WHITE);
    println!("painted {} panels", panels_from_white.len());

    let (min, max) = panels_from_white.keys()
        .fold((Point::zero(), Point::zero()), |(min, max), point| {
            let min_x = min.x.min(point.x);
            let min_y = min.y.min(point.y);
            let max_x = max.x.max(point.x);
            let max_y = max.y.max(point.y);
            (Point::new(min_x, min_y), Point::new(max_x, max_y))
        });
    let w = (max.x - min.x) + 1;
    let h = (max.y - min.y) + 1;

    // prints ABCLFUHJ
    visualize_points(w, h, |point| {
        let panel_pos = min + *point;
        if panels_from_white.get(&panel_pos).cloned() == Some(WHITE) {
            '#'
        } else {
            '.'
        }
    });
}