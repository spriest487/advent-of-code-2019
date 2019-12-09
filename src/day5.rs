pub mod intcode;
use intcode::Word;

fn input_one(mut code: Vec<Word>, input: Word) -> Vec<Word> {
    let mut in_buf = vec![input];
    let mut out_buf = Vec::new();

    intcode::exec(&mut code, &mut in_buf, &mut out_buf);

    out_buf
}

fn main() {
    let output = input_one(intcode::from_str("3,0,4,0,99"), 123);
    assert_eq!(output, [123]);

    let mut mul_test_program = intcode::from_str("1002,4,3,4,33");
    intcode::exec(&mut mul_test_program, &mut Vec::new(), &mut Vec::new());
    assert_eq!(mul_test_program[4], 99);

    let input = include_str!("day5.txt");
    let test_program = intcode::from_str(input);

    const AIR_CON: Word = 1;
    let air_con_diagnostics = input_one(test_program.clone(), AIR_CON);

    println!("air conditioner diagnostic: {:?}", air_con_diagnostics);

    // part 2

    const RADIATOR: Word = 5;
    let radiator_diagnostics = input_one(test_program, RADIATOR);

    println!("radiator diagnostic: {:?}", radiator_diagnostics);
}