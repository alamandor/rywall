use std::convert::TryInto;

// for reducing the number of image colors. Instead, all colors contained in the original
// image are considered in the quantization process. After the set of representative
// colors has been found, each image color is mapped to the closest representative
// in RGB color space using the Euclidean distance.
// The quantization process has two steps: first a ColorQuantizer object is created from
// a given image using one of the constructor methods provided. Then this ColorQuantizer
// can be used to quantize the original image or any other image using the same set of
// representative colors (color table).
//

#[derive(Debug, Clone, Copy, PartialEq)]
enum ColorDimension {
    Red,
    Green,
    Blue,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct ColorNode {
    pub rgb: u32,
    pub red: u8,
    pub grn: u8,
    pub blu: u8,
    pub cnt: usize,
}

impl ColorNode {
    fn new_rgb(rgb: u32, cnt: usize) -> ColorNode {
        ColorNode {
            rgb: (rgb & 0xFFFFFF),
            blu: ((rgb & 0xFF0000) >> 16) as u8,
            grn: ((rgb & 0xFF00) >> 8) as u8,
            red: (rgb & 0xFF) as u8,
            cnt: cnt,
        }
    }

    fn new_colors(red: u8, grn: u8, blu: u8, cnt: usize) -> ColorNode {
        ColorNode {
            rgb: ((red as u32 & 0xff) << 16) | ((grn as u32 & 0xff) << 8) | blu as u32 & 0xff,
            red: red,
            grn: grn,
            blu: blu,
            cnt: cnt,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
struct ColorBucket {
    lower: usize, // lower index into 'imageColors'
    upper: usize, // upper index into 'imageColors'
    level: isize, // where to split
    count: usize, // number of pixels in bucket
    rmin: i32,
    rmax: i32,
    gmin: i32,
    gmax: i32,
    bmin: i32,
    bmax: i32,
}

impl ColorBucket {
    fn new(lower: usize, upper: usize, level: isize, colors: &Vec<ColorNode>) -> ColorBucket {
        let mut bucket = ColorBucket {
            lower: lower,
            upper: upper,
            level: level,

            ..Default::default()
        };

        bucket.update_bounds(colors);

        bucket
    }

    fn color_count(&self) -> usize {
        self.upper - self.lower
    }

    // Updates bounds of colors in bucket
    fn update_bounds(&mut self, colors: &Vec<ColorNode>) {
        self.rmin = 255;
        self.rmax = 0;
        self.gmin = 255;
        self.gmax = 0;
        self.bmin = 255;
        self.bmax = 0;
        self.count = 0;
        for i in self.lower..self.upper {
            let color = colors[i];
            self.count += color.cnt;
            let r = color.red as i32;
            let g = color.grn as i32;
            let b = color.blu as i32;
            if r > self.rmax {
                self.rmax = r;
            }
            if r < self.rmin {
                self.rmin = r;
            }
            if g > self.gmax {
                self.gmax = g;
            }
            if g < self.gmin {
                self.gmin = g;
            }
            if b > self.bmax {
                self.bmax = b;
            }
            if b < self.bmin {
                self.bmin = b;
            }
        }
    }

    fn split_box(&mut self, colors: &mut Vec<ColorNode>) -> Option<ColorBucket> {
        if self.color_count() < 2 {
            None
        } else {
            // find longest dimension
            let longest_dimension = self.get_longest_color_longest_dimensionension();

            // find median along longest_dimension
            let med = self.find_median(longest_dimension, colors);

            // now split this box at the median return the resulting new box.
            let next_level = self.level + 1;
            let new_box = ColorBucket::new(med + 1, self.upper, next_level, colors);
            self.upper = med;
            self.level = next_level;
            self.update_bounds(colors);
            Some(new_box)
        }
    }

    fn get_longest_color_longest_dimensionension(&self) -> ColorDimension {
        let r_length = self.rmax - self.rmin;
        let g_length = self.gmax - self.gmin;
        let b_length = self.bmax - self.bmin;

        if b_length >= r_length && b_length >= g_length {
            ColorDimension::Blue
        } else if g_length >= r_length && g_length >= b_length {
            return ColorDimension::Green;
        } else {
            ColorDimension::Red
        }
    }

    fn find_median(&self, longest_dimension: ColorDimension, colors: &mut Vec<ColorNode>) -> usize {
        // sort color in this box along longest_dimension
        match longest_dimension {
            ColorDimension::Red => {
                colors[self.lower..(self.upper + 1)].sort_by(|a, b| a.red.cmp(&b.red))
            }
            ColorDimension::Green => {
                colors[self.lower..(self.upper + 1)].sort_by(|a, b| a.grn.cmp(&b.grn))
            }
            ColorDimension::Blue => {
                colors[self.lower..(self.upper + 1)].sort_by(|a, b| a.blu.cmp(&b.blu))
            }
        }

        // find the median point:
        let half = self.count / 2;
        let mut n_pixels = 0;
        // for (median = lower, n_pixels = 0; median < upper; median++) {
        for median in self.lower..self.upper {
            n_pixels = n_pixels + colors[median].cnt;
            if n_pixels >= half {
                return median;
            }
        }
        self.lower
    }

    fn get_average_color(&self, colors: &mut Vec<ColorNode>) -> ColorNode {
        let mut r_sum = 0;
        let mut g_sum = 0;
        let mut b_sum = 0;
        let mut n = 0usize;
        for i in self.lower..self.upper {
            let ci = colors[i];
            let cnt = ci.cnt;
            r_sum = r_sum + cnt * ci.red as usize;
            g_sum = g_sum + cnt * ci.grn as usize;
            b_sum = b_sum + cnt * ci.blu as usize;
            n = n + cnt;
        }
        // let nd = n as f64;
        let avg_red = (r_sum as f64 / n as f64) as u8;
        let avg_grn = (g_sum as f64 / n as f64) as u8;
        let avg_blu = (b_sum as f64 / n as f64) as u8;
        ColorNode::new_colors(avg_red, avg_grn, avg_blu, n)
    }
}

struct ColorHistogram {
    color_array: Vec<u32>,
    count_array: Vec<usize>,
}

impl ColorHistogram {
    pub fn new(colors: Vec<u32>, counts: Vec<usize>) -> ColorHistogram {
        ColorHistogram {
            color_array: colors,
            count_array: counts,
        }
    }

    pub fn new_pixels(pixels_orig: &[u32]) -> ColorHistogram {
        let n = pixels_orig.len();
        let mut pixels_copy = Vec::with_capacity(n);
        for i in 0..n {
            // remove possible alpha components
            pixels_copy.push(0xFFFFFF & pixels_orig[i]);
        }
        pixels_copy.sort();

        // count unique colors:
        let mut k = 0; // current color index
        let mut inited = false;
        let mut cur_color = 0;
        for i in 0..pixels_copy.len() {
            if pixels_copy[i] != cur_color || !inited {
                cur_color = pixels_copy[i];
                k += 1;
                inited = true;
            }
        }

        // tabulate and count unique colors:
        let mut color_array = Vec::with_capacity(k);
        let mut count_array = Vec::with_capacity(k);
        k = 0; // current color index
        cur_color = 0;
        let mut inited = false;
        for i in 0..pixels_copy.len() {
            if pixels_copy[i] != cur_color || !inited {
                // new color
                cur_color = pixels_copy[i];
                color_array.push(cur_color);
                count_array.push(1);
                inited = true;
                k += 1;
            } else {
                count_array[k - 1] += 1;
            }
        }
        ColorHistogram::new(color_array, count_array)
    }
}

pub struct MMCQ {
    image_colors: Vec<ColorNode>,
    quant_colors: Vec<ColorNode>,
}

impl MMCQ {
    pub fn from_pixel_vec(pixels: &[u8], k_max: u32) -> MMCQ {
        let pixel_len = pixels.len();
        let mut vec_32_bit = Vec::<u32>::new();

        // Safe'er' method to get a slice of [u32] out of [u8]
        for x in (0..pixel_len / 4).step_by(4) {
            // println!("x = {}", x);
            let slice_32 = &pixels[x..(x + 4)];
            let byte_slice =
                u32::from_le_bytes(slice_32.try_into().expect("failure converting u8 to u32"));
            vec_32_bit.push(byte_slice);
        }
        // Grab groups of 4 8bit numbers and interpet them as single u32 numbers , slice will be a quarter of the length as a result.

        MMCQ::from_u32_vec(vec_32_bit.as_slice(), k_max)
    }

    pub fn from_u32_vec(pixels: &[u32], k_max: u32) -> MMCQ {
        let mut m = MMCQ {
            image_colors: Vec::new(),
            quant_colors: Vec::new(),
        };

        m.quant_colors = m.find_representative_colors(&pixels, k_max);
        m.quant_colors.sort_by(|a, b| b.cnt.cmp(&a.cnt));

        m
    }

    pub fn get_quantized_colors(&self) -> &Vec<ColorNode> {
        &self.quant_colors
    }

    fn find_representative_colors(&mut self, pixels: &[u32], k_max: u32) -> Vec<ColorNode> {
        let color_hist = ColorHistogram::new_pixels(pixels);
        let hist_color_total = color_hist.color_array.len();

        self.image_colors = Vec::with_capacity(hist_color_total);
        for i in 0..hist_color_total {
            let rgb = color_hist.color_array[i];
            let cnt = color_hist.count_array[i];
            self.image_colors.push(ColorNode::new_rgb(rgb, cnt));
        }

        // println!("{:?}", self.image_colors);

        if hist_color_total <= k_max as usize {
            let result = self.image_colors.clone();
            result
        }
        else{
            let initial_box = ColorBucket::new(0, hist_color_total - 1, 0, &mut self.image_colors);
            let mut color_set = Vec::new();
            color_set.push(initial_box);
            let mut k = 1;
            let mut done = false;

            while k < k_max && !done {
                let mut new_box: Option<ColorBucket>;
                match self.get_next_split(&mut color_set) {
                    Some(x) => {
                        new_box = x.split_box(&mut self.image_colors);
                        color_set.push(new_box.unwrap());
                        k += 1;
                    },
                    None => {
                        done = true;
                    }
                }
            }

            let result = self.average_colors(&color_set);
            result
        }


//         let r_cols = if hist_color_total <= k_max as usize {
//             // image has fewer colors than k_max
//             self.image_colors.clone()
//         } else {
//             let initial_box = ColorBucket::new(0, hist_color_total - 1, 0, &mut self.image_colors);
//             let mut color_set = Vec::new();
//             color_set.push(initial_box);
//             let mut k = 1;
//             let mut done = false;
//             while k < k_max && !done {
//                 let new_box = if let Some(next_box) = self.get_next_split(&mut color_set) {
//                     next_box.split_box(&mut self.image_colors)
//                 } else {
//                     done = true;
//                     None
//                 };

//                 if let Some(new_box) = new_box {
//                     color_set.push(new_box);
//                     k = k + 1;
//                 }
//             }

//             self.average_colors(&color_set)
//         };
//         r_cols
    }

    fn average_colors(&mut self, color_buckets: &Vec<ColorBucket>) -> Vec<ColorNode> {
        let n = color_buckets.len();
        let mut avg_colors = Vec::with_capacity(n);
        for b in color_buckets {
            // println!("color box {:?}", b);
            avg_colors.push(b.get_average_color(&mut self.image_colors));
            // println!("avg {:?}", avg_colors[avg_colors.len()-1]);
        }
        return avg_colors;
    }

    fn get_next_split<'a>(
        &self,
        color_buckets: &'a mut Vec<ColorBucket>,
    ) -> Option<&'a mut ColorBucket> {
        let mut box_to_split = None;
        // from the set of splitable color boxes
        // select the one with the minimum level
        let mut min_level = std::isize::MAX;
        for b in color_buckets {
            if b.color_count() >= 2 {
                // box can be split
                if b.level < min_level {
                    min_level = b.level;
                    box_to_split = Some(b);
                }
            }
        }
        box_to_split
    }
}
