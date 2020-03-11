use std::convert::TryInto;

// Data structs used based from Java implementation provided in README

#[derive(Debug, Clone, Copy, PartialEq, Default)]
struct ColorBucket {
    lower: usize,
    upper: usize,
    level: isize,
    count: usize,
    rmin: i32,
    rmax: i32,
    gmin: i32,
    gmax: i32,
    bmin: i32,
    bmax: i32,
}

impl ColorBucket {
    fn new(lower: usize, upper: usize, level: isize, colors: &[ColorChannel]) -> ColorBucket {
        let mut bucket = ColorBucket {
            lower,
            upper,
            level,

            ..Default::default()
        };

        bucket.update_bounds(colors);

        bucket
    }

    // Reset values for use in the update_bounds() function
    fn reset_dimensions(&mut self) {
        self.rmin = 255;
        self.rmax = 0;
        self.gmin = 255;
        self.gmax = 0;
        self.bmin = 255;
        self.bmax = 0;
        self.count = 0;
    }

    fn color_count(&self) -> usize {
        self.upper - self.lower
    }

    // Sets internal color value to whatever is the largest of that and the provided argument val. Also matches the colortype to decide what channel to compare.
    fn larger(&mut self, in_color: i32, c_type: Color) {
        match c_type {
            Color::Red => {
                if in_color > self.rmax {
                    self.rmax = in_color;
                }
                if in_color < self.rmin {
                    self.rmin = in_color;
                }
            }
            Color::Green => {
                if in_color > self.gmax {
                    self.gmax = in_color;
                }
                if in_color < self.gmin {
                    self.gmin = in_color;
                }
            }
            Color::Blue => {
                if in_color > self.bmax {
                    self.bmax = in_color;
                }
                if in_color < self.bmin {
                    self.bmin = in_color;
                }
            }
        }
    }

    // Updates bounds of colors in bucket
    fn update_bounds(&mut self, colors: &[ColorChannel]) {
        self.reset_dimensions();

        for c in colors.iter().take(self.upper).skip(self.lower) {
            // let color = colors[i];
            self.count += c.cnt;

            self.larger(c.red as i32, Color::Red);
            self.larger(c.grn as i32, Color::Green);
            self.larger(c.blu as i32, Color::Blue);
        }
    }

    // Grabs Median of the longest color dimension and use it to find where the next split should be.
    fn split_box(&mut self, colors: &mut Vec<ColorChannel>) -> Option<ColorBucket> {
        if self.color_count() < 2 {
            None
        } else {
            let longest_dimension = self.get_longest_color();

            let med = self.find_median(longest_dimension, colors);

            // Level used to help make sure we split boxes that havent been split yet first
            let next_level = self.level + 1;
            let new_box = ColorBucket::new(med + 1, self.upper, next_level, colors);
            self.upper = med;
            self.level = next_level;
            self.update_bounds(colors);
            Some(new_box)
        }
    }

    fn get_longest_color(&self) -> Color {
        let r_length = self.rmax - self.rmin;
        let g_length = self.gmax - self.gmin;
        let b_length = self.bmax - self.bmin;

        if b_length >= r_length && b_length >= g_length {
            Color::Blue
        } else if g_length >= r_length && g_length >= b_length {
            Color::Green
        } else {
            Color::Red
        }
    }

    fn find_median(&self, longest_dimension: Color, colors: &mut Vec<ColorChannel>) -> usize {
        // sort color in this box along longest_dimension
        // By continuing to do this until the pallet is created, we try and seperate off distinctive colors by moving them to the top and splitting them off
        match longest_dimension {
            Color::Red => colors[self.lower..=self.upper].sort_by(|x, y| x.red.cmp(&y.red)),
            Color::Green => colors[self.lower..=self.upper].sort_by(|x, y| x.grn.cmp(&y.grn)),
            Color::Blue => colors[self.lower..=self.upper].sort_by(|x, y| x.blu.cmp(&y.blu)),
        }

        // iterate through and find the appropriate median to return by using the color count of each channel to increment the pixel number
        let half = self.count / 2;
        let mut pixel_num = 0;
        for median in self.lower..self.upper {
            pixel_num += colors[median].cnt;

            if pixel_num >= half {
                return median;
            }
        }
        self.lower
    }

    // Returns a new ColorChannel containing the acerage values of all the colors in the provided channel
    fn avg_color(&self, colors: &mut Vec<ColorChannel>) -> ColorChannel {
        let mut r_sum = 0;
        let mut g_sum = 0;
        let mut b_sum = 0;
        let mut n: usize = 0;
        for channel in colors.iter().take(self.upper).skip(self.lower) {
            let c = channel.cnt;
            r_sum += c * channel.red as usize;
            g_sum += c * channel.grn as usize;
            b_sum += c * channel.blu as usize;
            n += c;
        }
        let avg_r = (r_sum as f64 / n as f64) as u8;
        let avg_g = (g_sum as f64 / n as f64) as u8;
        let avg_b = (b_sum as f64 / n as f64) as u8;
        ColorChannel::new_colors(avg_r, avg_g, avg_b, n)
    }
}


#[derive(Debug, Clone, Copy)]
enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Debug, Clone,PartialEq, Copy, Default)]
pub struct ColorChannel {
    pub rgb: u32,
    pub red: u8,
    pub grn: u8,
    pub blu: u8,
    pub cnt: usize,
}

impl ColorChannel {
    fn new_rgb(rgb: u32, cnt: usize) -> ColorChannel {
        ColorChannel {
            rgb: (rgb & 0x00FF_FFFF),
            blu: ((rgb & 0x00FF_0000) >> 16) as u8,
            grn: ((rgb & 0xFF00) >> 8) as u8,
            red: (rgb & 0xFF) as u8,
            cnt,
        }
    }

    fn new_colors(red: u8, grn: u8, blu: u8, cnt: usize) -> ColorChannel {
        ColorChannel {
            rgb: ((red as u32 & 0xff) << 16) | ((grn as u32 & 0xff) << 8) | blu as u32 & 0xff,
            red,
            grn,
            blu,
            cnt,
        }
    }
}

struct Histogram {
    color_vec: Vec<u32>,
    count_vec: Vec<usize>,
}

impl Histogram {
    pub fn new(colors: Vec<u32>, counts: Vec<usize>) -> Histogram {
        Histogram {
            color_vec: colors,
            count_vec: counts,
        }
    }

    // Build a histogram from a provided u32 array and build a histogram
    // object from it. Bit-wise operation is used to remove any potential alpha values from the array before adding.
    pub fn new_pixels(pixels: &[u32]) -> Histogram {
        let mut color_vec = Vec::new();
        let mut count_vec = Vec::new();
        let mut c_index = 0;
        let mut first_loop = false;
        let mut cur_color = 0;
        let n = pixels.len();
        let mut pixels_copy = Vec::with_capacity(n);

        for p in pixels.iter().take(n) {
            pixels_copy.push(0x00FF_FFFF & p);
        }
        pixels_copy.sort();

        for p in &pixels_copy {
            if *p != cur_color || !first_loop {
                cur_color = *p;
                c_index += 1;
                first_loop = true;
            }
        }
        c_index = 0;
        cur_color = 0;
        first_loop = false;
        for p in &pixels_copy {
            if *p != cur_color || !first_loop {
                cur_color = *p;
                color_vec.push(cur_color);
                count_vec.push(1);
                first_loop = true;
                c_index += 1;
            } else {
                count_vec[c_index - 1] += 1;
            }
        }
        Histogram::new(color_vec, count_vec)
    }
}

pub struct MedianCut {
    image: Vec<ColorChannel>,
    quantized: Vec<ColorChannel>,
}

// Takes a vector of u8 rgb values and converts them to an array of u32's for the purpose of calulation and avoiding overflow
impl MedianCut {
    pub fn from_pixel_vec(pixels: &[u8], pallet_size: u32) -> MedianCut {
        let pixel_len = pixels.len();
        let mut vec_32_bit = Vec::<u32>::new();

        let mut dominant_colors = MedianCut {
            image: Vec::new(),
            quantized: Vec::new(),
        };

        // Safe'er' method to get a slice of [u32] out of [u8]
        for x in (0..pixel_len / 4).step_by(4) {
            let slice_32 = &pixels[x..(x + 4)];
            let byte_slice =
                u32::from_le_bytes(slice_32.try_into().expect("failure converting u8 to u32"));
            vec_32_bit.push(byte_slice);
        }
        // Grab groups of 4 8bit numbers and interpet them as single u32 numbers , slice will be a quarter of the length as a result.

        dominant_colors.quantized = dominant_colors.median_cut(&vec_32_bit, pallet_size);
        dominant_colors.quantized.sort_by(|a, b| b.cnt.cmp(&a.cnt));

        dominant_colors
    }

    pub fn get_quantized_colors(&self) -> &Vec<ColorChannel> {
        &self.quantized
    }

    fn median_cut(&mut self, pixels: &[u32], pallet_size: u32) -> Vec<ColorChannel> {
        let color_hist = Histogram::new_pixels(pixels);
        let mut count = 1;
        let mut done = false;
        let hist_color_total = color_hist.color_vec.len();

        // Move the rgb values with the count to the underlying image vector along with their volume
        self.image = Vec::with_capacity(hist_color_total);
        for i in 0..hist_color_total {
            let rgb = color_hist.color_vec[i];
            let cnt = color_hist.count_vec[i];
            self.image.push(ColorChannel::new_rgb(rgb, cnt));
        }

        // If their arent enough colors then we just return it early with whatever we have
        if hist_color_total <= pallet_size as usize {
            self.image.clone()
        } else {
            // Create first box to split from, with all the colors
            let initial_box = ColorBucket::new(0, hist_color_total - 1, 0, &self.image);
            let mut color_set = Vec::new();
            color_set.push(initial_box);

            // Continue splitting until we've reached our pallette size (16)
            while count < pallet_size && !done {
                let new_box: Option<ColorBucket>;
                match self.get_next_split(&mut color_set) {
                    Some(x) => {
                        new_box = x.split_box(&mut self.image);
                        color_set.push(new_box.unwrap());
                        count += 1;
                    }
                    None => {
                        done = true;
                    }
                }
            }

            self.avg_colors(&color_set)
        }
    }

    // Average all the colors in th
    fn avg_colors(&mut self, color_buckets: &[ColorBucket]) -> Vec<ColorChannel> {
        let n = color_buckets.len();
        let mut avg_colors = Vec::with_capacity(n);
        for bucket in color_buckets {
            avg_colors.push(bucket.avg_color(&mut self.image));
        }
        avg_colors
    }

    // Find the next bucket with the smallest level that has more than 2 colors left in it
    fn get_next_split<'a>(
        &self,
        color_buckets: &'a mut Vec<ColorBucket>,
    ) -> Option<&'a mut ColorBucket> {
        let mut next_split = None;
        let mut min = std::isize::MAX;
        for bucket in color_buckets {
            if bucket.color_count() >= 2 && bucket.level < min {
                min = bucket.level;
                next_split = Some(bucket);
            }
        }
        next_split
    }
}
