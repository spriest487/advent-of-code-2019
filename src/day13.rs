use std::collections::HashMap;

mod intcode;
mod point;
use point::*;
use std::cmp::Ordering;

const BLOCK: intcode::Word = 2;
const PADDLE: intcode::Word = 3;
const BALL: intcode::Word = 4;

enum JoyInput {
    Neutral,
    Left,
    Right,
}

struct Game {
    computer: intcode::Computer,
    screen: HashMap<Point, intcode::Word>,
    score: intcode::Word,
}

impl Game {
    fn new(code: Vec<intcode::Word>) -> Self {
        Self {
            computer: intcode::Computer::new(code),
            screen: HashMap::new(),
            score: 0,
        }
    }

    fn play_for_free(&mut self) {
        self.computer.mem_store(0, 2);
    }

    fn run<Joystick: Fn(&Self) -> JoyInput>(&mut self, joystick: Joystick) {
        loop {
            let result = self.computer.run();

            for output in self.computer.out_buf.chunks(3) {
                let pos = Point::new(output[0], output[1]);
                let id = output[2];

                if pos == Point::new(-1, 0) {
                    self.score = id;
                } else {
                    self.screen.insert(pos, id);
                }
            }
            self.computer.out_buf.clear();

            match result {
                Err(intcode::ExecError::InputBlocked) => {
                    let input = joystick(self);
                    self.computer.in_buf.push(match input {
                        JoyInput::Neutral => 0,
                        JoyInput::Left => -1,
                        JoyInput::Right => 1,
                    });
                }

                Ok(()) => {
                    break;
                }
            }
        }
    }
}

fn main() {
    let input = include_str!("day13.txt");
    let code = intcode::from_str(input);
    let mut game = Game::new(code.clone());

    // run in demo mode without putting any quarters in
    game.run(|_state| panic!("no joystick input expected"));

    // 320
    println!("blocks after exit: {}", game.screen.values().filter(|id| **id == BLOCK).count());

    let mut game = Game::new(code);
    game.play_for_free();

    game.run(|state| {
        let paddle_tile = state.screen.iter()
            .find(|(_pos, id)| **id == PADDLE);

        let ball_tile = state.screen.iter()
            .find(|(_pos, id)| **id == BALL);

        if paddle_tile.is_none() || ball_tile.is_none() {
            return JoyInput::Neutral;
        }

        let (ball_pos, _) = ball_tile.unwrap();
        let (paddle_pos, _) = paddle_tile.unwrap();

        match paddle_pos.x.cmp(&ball_pos.x) {
            Ordering::Equal => JoyInput::Neutral,
            Ordering::Less => JoyInput::Right,
            Ordering::Greater => JoyInput::Left,
        }
    });

    println!("score after exit: {}", game.score);
}