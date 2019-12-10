fn read_img(data: &str) -> Vec<u32> {
    data.chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect()
}

fn image_checksum(image: &[u32], w: usize, h: usize) -> usize {
    let layer_len = w * h;
    let layer_index = image.chunks(layer_len)
        .enumerate()
        .min_by_key(|(_index, layer)| {
            layer.iter().filter(|col| **col == 0).count()
        })
        .map(|(index, _)| index)
        .unwrap();

    println!("using layer {} for checksum", layer_index + 1);

    let layer_start = layer_len * layer_index;

    let (ones, twos) = image[layer_start..layer_start + layer_len].iter()
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

fn main() {
    let input = include_str!("day8.txt");
    let input_colors = read_img(input);

    let checksum = image_checksum(&input_colors, 25, 6);
    println!("image has checksum value {}", checksum);
}