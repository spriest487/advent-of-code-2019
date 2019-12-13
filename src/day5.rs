pub mod intcode;
use intcode::{Word, Computer};

fn input_one(code: Vec<Word>, input: Word) -> Computer {
    let mut computer = intcode::Computer::new(code);
    computer.in_buf.push(input);
    computer.run().expect("should run until halt");
    computer
}

fn main() {
    let output = input_one(intcode::from_str("3,0,4,0,99"), 123);
    assert_eq!(output.out_buf, [123]);

    let mut mul_computer = Computer::new(intcode::from_str("1002,4,3,4,33"));
    mul_computer.run().expect("should run to halt");
    assert_eq!(mul_computer.mem_load(4), 99);

    let input = include_str!("day5.txt");
    let test_program = intcode::from_str(input);

    const AIR_CON: Word = 1;
    let air_con_diagnostics = input_one(test_program.clone(), AIR_CON);

    println!("air conditioner diagnostic: {:?}", air_con_diagnostics.out_buf);

    // part 2

    const RADIATOR: Word = 5;
    let radiator_diagnostics = input_one(test_program, RADIATOR);

    println!("radiator diagnostic: {:?}", radiator_diagnostics.out_buf);
}