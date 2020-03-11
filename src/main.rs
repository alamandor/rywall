use clap::{App, Arg};
use float_cmp::*;
use image::ImageFormat;
use std::collections::HashMap;
use std::fs::*;
use std::io::{BufRead, BufReader, Error, Write};
use std::process::Command;
mod mcq_image;

fn main() -> Result<(), Error> {
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
                .help("save created pallet in a theme file")
                .takes_value(true),
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

    // Load Pallet and apply colorscheme from a JPEG file
    if matches.is_present("image") {
        let image_file_name = matches.value_of("image").unwrap();
        if matches.is_present("save") {
            let save_file = matches.value_of("save").unwrap();
            colors_from_image(image_file_name, save_file)?;
        } else {
            colors_from_image(image_file_name, "")?;
        }
    }

    // Reload Xresource file
    if matches.is_present("reload") {
        let p_output = Command::new("xrdb")
            .arg("/home/aag/.Xresources")
            .output()
            .expect("failed to execute xrdb");
        println!("{:?}", p_output);
    }

    // Print the Current colorscheme in the XDatabase
    if matches.is_present("list") {
        let current_colors = xrdb::Colors::new("*");

        let fg = current_colors.clone().unwrap().fg.unwrap();
        let bg = current_colors.clone().unwrap().bg.unwrap();
        println!("Current colorscheme loaded by Xresources database");
        println!("fg = {:?}", fg);
        println!("bg = {:?}", bg);

        for color in current_colors.unwrap().colors.iter() {
            println!("color = {}", color.clone().unwrap());
        }
    }

    // Done
    Ok(())
}

fn colors_from_image(file: &str, o_path: &str) -> Result<(), Error> {
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
    let path = if !o_path.is_empty() {
        o_path
    } else {
        "./colorscheme"
    };
    let mut output = File::create(path)?;

    let mut all_colors = HashMap::new();

    for x in 0..pallet_size {
        let mut q = common_colors[x as usize];

        // If number are too low add just enough to get them over 16.
        // Messy fix for not getting format! to pad numbers below 16 with a zero in Hexadecimal.

        if q.red <= 16 {
            q.red += (16 - q.red) + 1;
        }
        if q.grn <= 16 {
            q.grn += (16 - q.grn) + 1;
        }
        if q.blu <= 16 {
            q.blu += (16 - q.blu) + 1;
        }

        // Find the luminance (brightness) of color. brighter = higher
        let lum = calc_luminance(q.red, q.grn, q.blu);
        let x_color_str = format!("*color{}: #{:X}{:X}{:X}", x, q.red, q.grn, q.blu);
        // println!("{:?}", x_color_str);
        writeln!(output, "{}", x_color_str)?;
        all_colors.insert(x_color_str, lum);
    }

    let mut lum_max = std::f64::MIN;
    let mut lum_min = std::f64::MAX;

    for val in all_colors.values() {
        if *val < lum_min {
            lum_min = *val;
        }
        if *val > lum_max {
            lum_max = *val;
        }
    }

    let fg = all_colors.iter().find_map(|(key, &val)| {
        if approx_eq!(f64, val, lum_max, ulps = 5) {
            Some(key)
        } else {
            None
        }
    });
    let bg = all_colors.iter().find_map(|(key, &val)| {
        if approx_eq!(f64, val, lum_min, ulps = 5) {
            Some(key)
        } else {
            None
        }
    });

    let bg_color = bg.unwrap().as_str();
    let fg_color = fg.unwrap().as_str();
    let bg_str: Vec<&str> = bg_color.split(' ').collect();
    let fg_str: Vec<&str> = fg_color.split(' ').collect();

    // Construct string for file that has the brightest and darkest values
    let bg_file_string = format!("*background: {}", bg_str[1]);
    let fg_file_string = format!("*foreground: {}", fg_str[1]);

    writeln!(output, "{}", bg_file_string)?;
    writeln!(output, "{}", fg_file_string)?;

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    println!("This is your new Xresources Colorcheme:");
    for line in buffered.lines() {
        println!("{}", line?);
    }

    Ok(())
}

// TODO implement an iterator method to ColorChannel that takes this as a closure?
fn calc_luminance(r: u8, g: u8, b: u8) -> f64 {
    (r as f64 * 0.299 + g as f64 * 0.587 + b as f64 * 0.114) / 256.0
}
