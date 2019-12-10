use digits_iterator::*;

mod intcode;
use intcode::{Word, Computer};

fn exec_one(code: Vec<Word>) -> Vec<Word> {
    let mut computer = Computer::new(code);
    computer.run().expect("should run until halt");

    computer.out_buf
}

fn main() {
    assert_eq!(exec_one(intcode::from_str("104,1125899906842624,99")), [1125899906842624]);

    assert_eq!(exec_one(intcode::from_str("1102,34915192,34915192,7,4,7,99,0"))[0].digits().count(), 16);

    let quine = intcode::from_str("109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99");
    assert_eq!(exec_one(quine.clone()), quine);

    let input = include_str!("day9.txt");
    let code = intcode::from_str(input);

    let mut boost_computer = Computer::new(code);
    boost_computer.in_buf.push(1);
    boost_computer.run().expect("should halt");

    println!("BOOST output: {:?}", boost_computer.out_buf);
}