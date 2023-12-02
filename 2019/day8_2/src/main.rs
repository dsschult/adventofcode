use std::fs;

type Pixel = u8;
type Row = Vec<Pixel>;
type Layer = Vec<Row>;

struct Image {
    width: usize,
    height: usize,
    layers: Vec<Layer>,
}

impl Image {
    fn from_str(input: String, width: usize, height: usize)
        -> Image {
        if input.len() % (width * height) != 0 {
            panic!("input len not divisible by layer size");
        }
        let layers = input.len() / (width * height);
        let mut next_digit = input.chars();

        let mut image = Image{width: width, height: height, layers: Vec::new()};
        for l in 0..layers {
            let mut layer = Vec::new();
            for r in 0..height {
                let mut row = Vec::new();
                for c in 0..width {
                    let d = match next_digit.next() {
                        Some(x) => x.to_digit(10).unwrap() as Pixel,
                        None => panic!("no more pixels in image @ layer {}, row {}, col {}", l, r, c),
                    };
                    row.push(d);
                }
                layer.push(row);
            }
            image.layers.push(layer);
        }
        image
    }
}

fn get_checksum(img: &Image) -> usize {
    // first get layer with fewest 0s
    let mut min_layer_size = 999999;
    let mut min_layer_index = None;
    for (i, layer) in img.layers.iter().enumerate() {
        let mut num_zeros = 0;
        for row in layer {
            for pixel in row {
                if *pixel == 0 {
                    num_zeros += 1;
                }
            }
        }
        if num_zeros < min_layer_size {
            min_layer_size = num_zeros;
            min_layer_index = Some(i);
        }
    }
    if min_layer_index == None {
        panic!("couldn't find a layer with fewest 0s");
    }
    // now, get the number of 1 digits multiplied by 2 digits on layer
    let mut num_ones = 0;
    let mut num_twos = 0;
    for row in img.layers[min_layer_index.unwrap()].iter() {
        for pixel in row {
            match *pixel {
                1 => num_ones += 1,
                2 => num_twos += 1,
                _ => ()
            }
        }
    }
    num_ones * num_twos
}

fn flatten_image(img: &Image) -> Layer {
    let mut flat = Vec::new();
    for row in 0..img.height {
        let mut row_pixels = Vec::new();
        for col in 0..img.width {
            let mut color = 2; // start with transparent
            for layer in img.layers.iter() {
                color = layer[row][col];
                if color != 2 {
                    break;
                }
            }
            row_pixels.push(color);
        }
        flat.push(row_pixels);
    }
    flat
}

fn ascii_art(layer: Layer) -> () {
    for row in layer.iter() {
        for pixel in row.iter() {
            match pixel {
                0 => print!("   "),
                1 => print!(" * "),
                _ => panic!("transparent pixel"),
            }
        }
        print!("\n");
    }
}

fn main() {
    let input = fs::read_to_string("input")
        .expect("Something went wrong reading the input file");
    let img = Image::from_str(input, 25, 6);
    println!("checksum: {}", get_checksum(&img));
    println!("flattened:");
    ascii_art(flatten_image(&img));
}

mod tests {
    use super::*;

    #[test]
    fn test_image_from_str() {
        let input = String::from("123456789012");
        let img = Image::from_str(input, 3, 2);
        assert_eq!(img.width, 3);
        assert_eq!(img.height, 2);
        assert_eq!(img.layers.len(), 2);
        assert_eq!(img.layers[0][0], vec![1,2,3]);
        assert_eq!(img.layers[0][1], vec![4,5,6]);
        assert_eq!(img.layers[1][0], vec![7,8,9]);
        assert_eq!(img.layers[1][1], vec![0,1,2]);
    }

    #[test]
    fn test_flatten() {
        let input = String::from("0222112222120000");
        let img = Image::from_str(input, 2, 2);
        let img_flat = flatten_image(&img);
        assert_eq!(img_flat[0], vec![0, 1]);
        assert_eq!(img_flat[1], vec![1, 0]);
    }
}