use itertools::*;
mod intcode;
use intcode::{Word, ExecError, Computer};

fn new_amp(code: Vec<Word>, setting: Word) -> Computer {
    let mut computer = Computer::new(code);
    computer.in_buf.push(setting);
    computer
}

fn run_amp(amp: &mut Computer) -> Option<Word> {
    match amp.run() {
        Err(ExecError::InputBlocked) => None,

        Ok(()) => {
            assert_eq!(amp.out_buf.len(), 1);
            Some(amp.out_buf[0])
        },
    }
}

fn main() {
    let mut max_output = 0;
    let mut max_combination = Vec::new();

    let input = include_str!("day7.txt");
    let code = intcode::from_str(input);

    fn create_amps(code: &[Word], setting: &[Word]) -> [Computer; 5] {
        [
            new_amp(code.to_vec(), setting[0]),
            new_amp(code.to_vec(), setting[1]),
            new_amp(code.to_vec(), setting[2]),
            new_amp(code.to_vec(), setting[3]),
            new_amp(code.to_vec(), setting[4]),
        ]
    }

    for combination in (0 as Word..=4).permutations(5) {
        let mut amps = create_amps(&code, &combination);
        amps[0].in_buf.push(0);

        for i in 0..5 {
            let out = run_amp(&mut amps[i]).expect("should run until halt in part 1");

            if i == amps.len() - 1 {
                if out > max_output {
                    max_output = out;
                    max_combination = combination.clone();
                }
            } else {
                amps[i + 1].in_buf.push(out);
            }
        }
    }

    println!("max output (setting {:?}): {} (combination)", max_combination, max_output);

    for combination in (5 as Word..=9).permutations(5) {
        let mut amps = create_amps(&code, &combination);
        let mut final_values = [None; 5];
        amps[0].in_buf.push(0);

        let out = 'feedback_loop: loop {
            for i in 0..5 {
                if final_values[i].is_some() {
                    continue;
                }

                let result = run_amp(&mut amps[i]);

                let next_i = (i + 1) % 5;
                let out: Vec<_> = amps[i].out_buf.drain(0..).collect();
                amps[next_i].in_buf.extend(out);

                if let Some(final_val) = result {
                    final_values[i] = Some(final_val);
                    if final_values.iter().all(Option::is_some) {
                        break 'feedback_loop final_values[4].unwrap();
                    }
                }
            }
        };

        if out > max_output {
            max_output = out;
            max_combination = combination.clone();
        }
    };

    println!("last output of feedback loop (setting {:?}): {}", max_combination, max_output);
}