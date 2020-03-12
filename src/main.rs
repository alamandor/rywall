use clap::{App, Arg};
use rand::seq::SliceRandom;
use rand::{thread_rng};
use dirs::home_dir;
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
        .help("myapp v1.0\n\
           Generate colorschemes from .jpg/.jpeg\n\
           (C) aag3@pdx.edu\n\n\

           USAGE: rusty-theme -i <image_file> -s <output_name> [Options]\n\n\

           Options:\n\
           -h, --help           Display this message\n\
           -i, --image <file>   Use supplied file for colorscheme\n\
           -s  --save <name>    Use supplied name for colorscheme file generated\n\
           -r                   Reload the default .Xresources file cannot use with -n\n\
           -n --now             Reload Xresources with generated colorscheme\n\
           -c --colorscheme     Load the provided colorscheme file made with the tool in xrdb")
        .usage("rusty-theme [-i <image file path>]\n\t-- [-c Immediately load generated colorscheme]\n\t-- [-r Load the user's default .Xresources file in their home directory] (Cannot be used with the -c Option)]\n\t-- [-s <Desired colorscheme name>]")
        .about("Use existing images to calculate a pallet for Xresources")
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
            Arg::with_name("now")
                .short("n")
                .long("now")
                .help("Immediately load the created colorscheme in xrdb"),
        )
        .arg(
            Arg::with_name("colorscheme")
                .short("c")
                .long("colorscheme")
                .help("Load the provided colorscheme")
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
    let save_file: &str;

    if matches.is_present("colorscheme") {
        let file = matches.value_of("colorscheme").unwrap();
        let p_output = Command::new("xrdb")
            .arg(file)
            .status()
            .expect("failed to execute xrdb");
            match p_output.code() {
                Some(code) => {
                    if code != 0 {
                        println!("Error in running xrdb");
                        return Ok(());
                    }
                },
                None => println!("Process terminated by signal")
            }
        return Ok(());
    }

    // Load Pallet and apply colorscheme from a JPEG file
    if matches.is_present("image") {
        let image_file_name = matches.value_of("image").unwrap();
        if matches.is_present("save") {
            save_file = matches.value_of("save").unwrap();
            println!("{}", save_file);
            colors_from_image(image_file_name, save_file)?;
        } else {
            colors_from_image(image_file_name, "")?;
            save_file = "/colorscheme";
        }
        // Reload colorscheme  file
        if matches.is_present("now") {
            let p_output = Command::new("xrdb")
                .arg(save_file)
                .status()
                .expect("failed to execute xrdb");
            match p_output.code() {
                Some(code) => {
                    if code != 0 {
                        println!("Error in running xrdb");
                        return Ok(());
                    }
                },
                None => println!("Process terminated by signal")
            }
        }
    }

    // Reload Default Xresource file
    if matches.is_present("reload") {
        let home = home_dir();
        match home {
            Some(x) => {
                let mut home = x;
                home.push(".Xresources");
                if matches.occurrences_of("now") == 0 {
                    let p_output = Command::new("xrdb")
                        .arg(home)
                        .status()
                        .expect("failed to execute xrdb");
                    match p_output.code() {
                        Some(code) => {
                            if code != 0 {
                                println!("Error in running xrdb");
                                return Ok(());
                            }
                        },
                        None => println!("Process terminated by signal")
                    }
                } else {
                    println!("Can't use reload (-r) default .Xresources file with the -n option");
                }
            }
            None => {
                println!("Cannot find Home Directory, make sure ENV variable is set");
            }
        }
    }

    // Print the Current colorscheme in the XDatabase
    if matches.is_present("list") {
        list_loaded_colors();
    }

    // Done
    Ok(())
}

fn shuffle_colors(colormap: &HashMap<String, f64>) -> Vec<String>{
    let mut names = Vec::new();
    let mut hex = Vec::new();
    let mut rng = thread_rng();

    for key in colormap.keys(){
        let name_and_hex: Vec<&str> = key.split(' ').collect();
        names.push(name_and_hex[0]);
        hex.push(name_and_hex[1]);
        names.shuffle(&mut rng);
        hex.shuffle(&mut rng);
    }

    let mut new_strings = Vec::new();
    for name_hex in names.iter().zip(hex.iter_mut()) {
        let (n, h) = name_hex;
        let full_string = format!("{}{}", n, h);
        new_strings.push(full_string.clone());
    }

    new_strings
}

fn list_loaded_colors() {
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

    // Find and get the appropriate matching max and min values and their key
    // approx_eq! is from the float-cmp crate, makes it so it compiles with cargo clippy
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

    let rand_colors: Vec<String> = shuffle_colors(&all_colors);
    println!("{:?}", rand_colors);

    let bg_color = bg.unwrap().as_str();
    let fg_color = fg.unwrap().as_str();

    // Confirm what was found is one of our colors
    assert!(all_colors.contains_key(bg_color));
    assert!(all_colors.contains_key(fg_color));

    let bg_str: Vec<&str> = bg_color.split(' ').collect();
    let fg_str: Vec<&str> = fg_color.split(' ').collect();


    // Construct string for file that has the brightest and darkest values
    let bg_file_string = format!("*background: {}", bg_str[1]);
    let fg_file_string = format!("*foreground: {}", fg_str[1]);

    // Write string to bottom of colorscheme file
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
