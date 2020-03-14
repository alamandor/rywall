#[cfg(test)]
mod tests {
use std::path::Path;
use crate::colors_from_image;
use std::process::Command;
use crate::list_loaded_colors;
use std::env;
    #[test]
    fn loaded_to_xsystem() {

        println!("Testing that rusty-theme loads colorscheme and saves it to Xsystem database, verify that colors for generated output and xrdb colors match up\n");

        let image_file_name = "res/snow_sunset.jpeg";
        let output_file = "test_colorscheme_loaded_to_xsystem";
        colors_from_image(image_file_name, output_file, false);
        let p_output = Command::new("xrdb")
            .arg(output_file)
            .status()
            .expect("failed to execute xrdb");
        match p_output.code() {
            Some(code) => {
                if code != 0 {
                    println!("Error in running xrdb");
                }
            }
            None => println!("Process terminated by signal"),
        }

        println!("LOADING COLORS IN X DATABASE\n");
        list_loaded_colors();





    }
    #[test]
    fn random_to_xsystem() {

        println!("Testing that rusty-theme loads colorscheme, randomizes the hex values associated with each number, and properly loads it to Xsystem\n");

        let image_file_name = "res/snow_sunset.jpeg";
        let output_file = "test_colorscheme_random_to_xsystem";
        colors_from_image(image_file_name, output_file, true);
        let p_output = Command::new("xrdb")
            .arg(output_file)
            .status()
            .expect("failed to execute xrdb");
        match p_output.code() {
            Some(code) => {
                if code != 0 {
                    println!("Error in running xrdb");
                }
            }
            None => println!("Process terminated by signal"),
        }

        println!("LOADING COLORS IN X DATABASE\n");
        list_loaded_colors();
    }
    #[test]
    fn save_with_correct_name() {
        println!("Testing that rusty-theme loads colorscheme, and saves it with the desired name");

        let image_file_name = "res/snow_sunset.jpeg";
        let output_file = "test_name";
        colors_from_image(image_file_name, output_file, false);
        let mut path = env::current_dir().unwrap();
        path.push("test_name");
        assert!(path.exists());

    }
}

