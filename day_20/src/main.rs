use std::io::BufRead;

type Pixel = u8;
type Image = matrix::Matrix<Pixel>;

fn parse_input() -> (Vec<Pixel>, Image) {
    fn light_level_from_symbol(c: char) -> Pixel {
        match c {
            '.' => 0,
            '#' => 1,
            _ => panic!("boom"),
        }
    }
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines().map(Result::unwrap);
    let codec = lines
        .next()
        .unwrap()
        .trim()
        .chars()
        .map(light_level_from_symbol)
        .collect();
    assert!(lines.next().unwrap().trim().is_empty());
    let mut im = Image::new();
    lines.for_each(|l| {
        im.next_row()
            .from_iter(l.trim().chars().map(light_level_from_symbol))
            .finish()
    });
    (codec, im)
}

fn expand_image(im: &Image) -> Image {
    let mut new = Image::default_with_size((im.dims().0 + 4, im.dims().1 + 4));
    for (i, j) in im.iter_coords() {
        *new.get_mut(i + 2, j + 2).unwrap() = *im.get(i, j).unwrap();
    }
    new
}

fn coord_to_new_val(im: &Image, codec: &[Pixel], i: usize, j: usize) -> Pixel {
    assert!(i > 0);
    assert!(j > 0);
    let mut coord = 0_usize;
    for k in [j - 1, j, j + 1] {
        for p in [i - 1, i, i + 1] {
            coord <<= 1;
            coord |= *im.get(p, k).unwrap() as usize;
        }
    }
    codec[coord]
}

fn background_change(bg: Pixel, codec: &[Pixel]) -> Pixel {
    let coord = if bg == 0{
        0
    } else {
        (1 << 9) - 1
    };
    codec[coord]
}

fn apply_decompression(im: &Image, codec: &[Pixel], background: Pixel) -> Image {
    let mut new = Image::new_with_elem((im.dims().0 + 4, im.dims().1 + 4), background);
    for i in 1..(im.dims().0 - 1) {
        for j in 1..(im.dims().1 - 1) {
            *new.get_mut(i + 2, j + 2).unwrap() = coord_to_new_val(im, codec, i, j);
        }
    }
    new
}

fn step_1(im: &Image, codec: &[Pixel]) {
    let mut im = expand_image(im);
    let mut background = 0;
    for _ in 0..2 {
        background = background_change(background, codec);
        im = apply_decompression(&im, codec, background);
    }

    let result = im.iter().filter(|&l| *l > 0).count();
    println!("Step 1: {}", result);
}

fn step_2(im: &Image, codec: &[Pixel]) {
    let mut im = expand_image(im);
    let mut background = 0;
    for _ in 0..50 {
        background = background_change(background, codec);
        im = apply_decompression(&im, codec, background);
    }

    let result = im.iter().filter(|&l| *l > 0).count();
    println!("Step 2: {}", result);
}

fn main() {
    let (codec, image) = parse_input();
    step_1(&image, &codec);
    step_2(&image, &codec)
}
