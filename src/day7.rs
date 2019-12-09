use itertools::*;
mod intcode;

fn main() {


    let setting_range = || (0 as intcode::Word..=4);

    let combinations = setting_range().permutations(5);

    let mut in_buf = Vec::new();
    let mut out_buf = Vec::new();

    let mut max_output = 0;
    let mut max_combination = Vec::new();

    let input = include_str!("day7.txt");
    let code = intcode::from_str(input);

    for combination in combinations {
        in_buf.clear();
        in_buf.push(0);

        out_buf.clear();

        let mut output = 0;
        for &setting in &combination {
            in_buf.insert(0, setting);

            intcode::exec(&mut code.clone(), &mut in_buf, &mut out_buf);
            assert_eq!(out_buf.len(), 1, "amplifier software should output one value");
            assert_eq!(in_buf.len(), 0, "amplifier software should consume all input");

            output = out_buf.remove(0);
            in_buf.push(output);
        }

        println!("{:?} -> {}", combination, output);

        if output > max_output {
            max_output = output;
            max_combination = combination;
        }
    }

    println!("combination with max output (value {}): {:?}", max_output, max_combination);
}