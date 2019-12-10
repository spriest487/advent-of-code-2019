const BLACK: u32 = 0;
const WHITE: u32 = 1;
const CLEAR: u32 = 2;

fn read_img(data: &str) -> Vec<u32> {
    data.chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect()
}

fn image_checksum(data: &[u32], w: usize, h: usize) -> usize {
    let layer_len = w * h;
    let layer_index = data.chunks(layer_len)
        .enumerate()
        .min_by_key(|(_index, layer)| {
            layer.iter().filter(|col| **col == 0).count()
        })
        .map(|(index, _)| index)
        .unwrap();

    println!("using layer {} for checksum", layer_index + 1);

    let layer_start = layer_len * layer_index;

    let (ones, twos) = data[layer_start..layer_start + layer_len].iter()
        .fold((0, 0), |(ones, twos), color| {
            match *color {
                1 => (ones + 1, twos),
                2 => (ones, twos + 1),
                _ => (ones, twos),
            }
        });

    println!("ones: {}, twos: {}", ones, twos);

    ones * twos
}

fn decode_image(data: &[u32], w: usize, h: usize) -> Vec<u32> {
    let layer_len = w * h;

    let mut output = vec![CLEAR; layer_len];

    for layer in data.chunks(layer_len).rev() {
        for y in 0..h {
            for x in 0..w {
                let off = y * w + x;
                let dest = output[off];
                let src = layer[off];

                output[off] = match (src, dest) {
                    (CLEAR, _) => dest,
                    (_, CLEAR) => src,
                    _ => src,
                };
            }
        }
    }

    output
}

fn main() {
    let input = include_str!("day8.txt");
    const W: usize = 25;
    const H: usize = 6;

    let data = read_img(input);

    let checksum = image_checksum(&data, W, H);
    println!("image has checksum value {}", checksum);

    let image = decode_image(&data, W, H);
    for y in 0..H {
        for x in 0..W {
            let off = y * W + x;
            print!("{}", match image[off] {
                WHITE => '#',
                BLACK => '.',
                _ => ' ',
            })
        }
        println!();
    }
}