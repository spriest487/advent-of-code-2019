mod intcode;
use intcode::{Computer, Word};

fn exec_command(initial: Vec<Word>, noun: Word, verb: Word) -> Word {
    let mut code = initial.to_vec();
    code[1] = noun;
    code[2] = verb;

    let mut computer = Computer::new(code);
    computer.run().expect("should run until halt");
    computer.mem_load(0)
}

fn main() {
    let input = include_str!("day2.txt");
    let code = intcode::from_str(input);

    const GRAVITY: Word = 12;
    const RESTORE: Word = 2;
    let gravity_result = exec_command(code.clone(), GRAVITY, RESTORE);
    assert_eq!(4138687, gravity_result);

    println!(
        "value in position 0 after restoring gravity: {}",
        gravity_result
    );

    const TARGET_STATE: Word = 19690720;

    for noun in 0..=99 {
        for verb in 0..=99 {
            if exec_command(code.clone(), noun, verb) == TARGET_STATE {
                let command_code = 100 * noun + verb;

                println!("command code for result {}: {}", TARGET_STATE, command_code);
                assert_eq!(command_code, 6635);

                break;
            }
        }
    }
}
