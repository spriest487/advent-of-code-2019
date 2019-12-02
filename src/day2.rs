const ADD: usize = 1;
const MUL: usize = 2;
const HCF: usize = 99;

fn exec_intcode(intcode: &mut [usize]) {
    let mut pc = 0;
    loop {
        match intcode[pc] {
            ADD => {
                let a_pos = intcode[pc + 1];
                let b_pos = intcode[pc + 2];
                let out_pos = intcode[pc + 3];

                intcode[out_pos] = intcode[a_pos] + intcode[b_pos];
            }

            MUL => {
                let a_pos = intcode[pc + 1];
                let b_pos = intcode[pc + 2];
                let out_pos = intcode[pc + 3];

                intcode[out_pos] = intcode[a_pos] * intcode[b_pos];
            }

            HCF => break,
            bad => panic!("invalid opcode {}", bad),
        }

        pc += 4;
    }
}

fn exec_command(initial: &[usize], noun: usize, verb: usize) -> usize {
    let mut intcode = initial.to_vec();
    intcode[1] = noun;
    intcode[2] = verb;

    exec_intcode(&mut intcode);

    intcode[0]
}

fn main() {
    let input = include_str!("day2.txt");
    let intcode: Vec<usize> = input.split(",")
        .map(|int| int.parse().unwrap())
        .collect();

    const GRAVITY: usize = 12;
    const RESTORE: usize = 2;
    let gravity_result = exec_command(&intcode, GRAVITY, RESTORE);

    println!("value in position 0 after restoring gravity: {}", gravity_result);

    const TARGET_STATE: usize = 19690720;

    for noun in 0..=99 {
        for verb in 0..=99 {
            if exec_command(&intcode, noun, verb) == TARGET_STATE {
                let command_code = 100 * noun + verb;

                println!("command code for result {}: {}", TARGET_STATE, command_code);

                break;
            }
        }
    }
}