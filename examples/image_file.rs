use colorsys::Hsl;
use image::DynamicImage::ImageRgba8;
use image::ImageFormat;
use std::fs::*;
use std::io::BufReader;
use std::path::*;
mod mcq_image;

const COLOR_HEIGHT: u32 = 64;
const QUANT_SIZE: u32 = 8;

fn main() {
    let paths = read_dir("./examples/res");

    for path in paths.unwrap() {
        let path = path.unwrap().path();
        let path = path.to_str().unwrap();
        process_image(path);
    }

    println!("\nPlease visit 'target' folder for the results");
}

fn process_image(file: &str) {
    println!("Reading image {}", file);

    let mcq = {
        let img = image::load(BufReader::new(File::open(file).unwrap()), ImageFormat::Jpeg)
            .unwrap()
            .to_rgba();
        let data = img.into_vec();

        // Here we extract the quantized colors from the image.
        // We need no more than 16 colors (QUANT_SIZE).
        mcq_image::MMCQ::from_pixels_u8_rgba(data.as_slice(), QUANT_SIZE)
    };

    // A `Vec` of colors, descendantely sorted by usage frequency
    let qc = mcq.get_quantized_colors();
    // println!("Quantized {:?}", qc);

    // =============================================================================================
    // Here we will demonstrate the extracted colors by generating the image
    // that consists of both original image and a resulted palette.
    // =============================================================================================

    // let img = image::load(BufReader::new(File::open(file).unwrap()), ImageFormat::Jpeg)
    //     .unwrap()
    //     .to_rgba();
    // let (ix, iy) = img.dimensions();

    // let mut imgbuf = image::ImageBuffer::new(ix, iy + COLOR_HEIGHT);

    // for x in 0..ix {
    //     for y in 0..iy {
    //         imgbuf.put_pixel(x, y, img.get_pixel(x, y).clone());
    //     }
    // }

    for x1 in 0..QUANT_SIZE {
        let q = qc[x1 as usize];
        println!("red = {:X}, grn = {:X}, blu = {:X}", q.red, q.grn, q.blu);
    }

    // let color_width = ix / QUANT_SIZE;

    // for y in (iy + 1)..(iy + COLOR_HEIGHT) {
    //     for x0 in 0..QUANT_SIZE {
    //         let x1 = x0 * color_width;
    //         let q = qc[x0 as usize];

    //         for x2 in 0..color_width {
    //             imgbuf.put_pixel(x1 + x2, y, image::Rgba([q.red, q.grn, q.blu, 0xff]));
    //         }
    //     }
    // }

    // let path_string = format!(
    //     "./target/{}",
    //     Path::new(file).file_name().unwrap().to_str().unwrap()
    // );

    // println!("Saving Output file to: {}", path_string);

    // let _ = image::DynamicImage::ImageRgba8(imgbuf).save(Path::new(&path_string));
}
