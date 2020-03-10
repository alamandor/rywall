use xrdb::*;
use clap::{App, Arg};
use image::ImageFormat;
use std::fs::*;
use std::io::BufReader;
mod mcq_image;

fn main() {
    let cli = App::new("rusty-theme")
        .version("1.0")
        .usage("rusty-theme [-i <image file path>]")
        .about("Use existing images to calculate a pallet for Xresources")
        .arg(Arg::with_name("theme").help("The theme to use").index(1))
        .arg(
            Arg::with_name("image")
                .short("i")
                .long("image")
                .value_name("file")
                .help("Direct to image file you want to use to make a pallet")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("save")
                .short("s")
                .long("save")
                .help("save created pallet in a theme file"),
        )
        .arg(
            Arg::with_name("reload")
                .short("r")
                .long("reload")
                .help("Reload Xresource files to update system colorscheme"),
        )
        .arg(
            Arg::with_name("list")
                .short("l")
                .long("list")
                .help("Print currently loaded theme in Xresources Database."),
        );

    let matches = cli.get_matches();

    if matches.is_present("image") {
        let image_file_name = matches.value_of("image").unwrap();
        colors_from_image(image_file_name);
    }
}

fn colors_from_image(file: &str) {
    let pallet_size = 16;
    println!("Reading image {}", file);

    let q_col = {
        let img = image::load(BufReader::new(File::open(file).unwrap()), ImageFormat::Jpeg)
            .unwrap()
            .to_rgba();
        let data = img.into_vec();

        mcq_image::MedianCut::from_pixel_vec(data.as_slice(), pallet_size)
    };

    let common_colors = q_col.get_quantized_colors();

    for x1 in 0..pallet_size {
        println!("Color {}:", (x1 + 1));
        let q = common_colors[x1 as usize];
        println!("Decimal: red = {}, grn = {}, blu = {}", q.red, q.grn, q.blu);
        println!("{}", q.rgb);
        println!(
            "Hexadecimal: red = {:X}, grn = {:X}, blu = {:X}",
            q.red, q.grn, q.blu
        );
    }
}
