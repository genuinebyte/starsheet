use rand::distributions::{Distribution, Uniform, WeightedIndex};

/// The main structure exposed by starsheet.
pub struct Space {
    width: usize,
    height: usize,
    data: Vec<u8>,
}

impl Space {
    /// Create a new `Space` struct.
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width: width as usize,
            height: height as usize,
            data: vec![0; (width * height) as usize],
        }
    }

    /// Get the width of the image.
    pub fn width(&self) -> u32 {
        self.width as u32
    }

    // Get the height of the image.
    pub fn height(&self) -> u32 {
        self.height as u32
    }

    /// Fill space with a random assortmnt of stars. They will have varrying
    /// brightness and positins.
    pub fn fill_randomly(&mut self, density: u32) {
        let mut rng = rand::thread_rng();
        let rand_x = Uniform::from(0..self.width);
        let rand_y = Uniform::from(0..self.height);

        let weights = [755, 125, 90, 18, 6, 4, 2, 1];
        let rand_z =
            WeightedIndex::new(&weights).expect("Failed to created intensity WeightedIndex");

        let starcount = ((self.width as f32 / 100f32)
            * (self.height as f32 / 100f32)
            * density as f32) as usize;

        for _ in 0..starcount {
            self.draw_star(
                rand_x.sample(&mut rng),
                rand_y.sample(&mut rng),
                rand_z.sample(&mut rng),
            );
        }
    }

    /// Consumes itself and returns the raw image data. This data is greyscale,
    /// so each u8 is a pixel.
    pub fn to_data(self) -> Vec<u8> {
        self.data
    }

    fn draw_star(&mut self, x: usize, y: usize, intensity: usize) {
        if intensity == 0 {
            let index = y * self.width + x;

            *self.data.get_mut(index).unwrap() = 255;
        } else {
            let clamp = |val: isize, low: usize, high: usize| -> usize {
                if val < low as isize {
                    low
                } else if val > high as isize {
                    high
                } else {
                    val as usize
                }
            };

            let x1 = x as isize - intensity as isize;
            let y1 = y as isize - intensity as isize;
            let x2 = x + intensity;
            let y2 = y + intensity;

            // Draw the mainlines
            self.linex(
                y,
                clamp(x1, 0, self.width - 1),
                clamp(x2 as isize, 0, self.width - 1),
                255,
            );
            self.liney(
                x,
                clamp(y1, 0, self.height - 1),
                clamp(y2 as isize, 0, self.height - 1),
                255,
            );

            // Draw the diagonals
            if intensity >= 2 {
                let x1 = x as isize - (intensity / 2) as isize;
                let y1 = y as isize - (intensity / 2) as isize;
                let x2 = x + (intensity / 2);
                let y2 = y + (intensity / 2);

                self.line(
                    clamp(x1, 0, self.width - 1),
                    clamp(y1, 0, self.height - 1),
                    clamp(x2 as isize, 0, self.width - 1),
                    clamp(y2 as isize, 0, self.height - 1),
                    255,
                );

                self.line(
                    clamp(x2 as isize, 0, self.width - 1),
                    clamp(y1, 0, self.height - 1),
                    clamp(x1, 0, self.width - 1),
                    clamp(y2 as isize, 0, self.height - 1),
                    255,
                );
            }

            // Remove the center
            *self.data.get_mut(y * self.width + x).unwrap() = 0;
        }
    }

    fn linex(&mut self, y: usize, x1: usize, x2: usize, gray: u8) {
        for x in x1..=x2 {
            *self.data.get_mut(y * self.width + x).unwrap() = gray;
        }
    }

    fn liney(&mut self, x: usize, y1: usize, y2: usize, gray: u8) {
        for y in y1..=y2 {
            *self.data.get_mut(y * self.width + x).unwrap() = gray;
        }
    }

    fn line(&mut self, x1: usize, y1: usize, x2: usize, y2: usize, gray: u8) {
        let dx = x2 as isize - x1 as isize;
        let dy = y2 as isize - y1 as isize;
        let step = if dx.abs() > dy.abs() {
            dx.abs()
        } else {
            dy.abs()
        };

        let dx = dx / step;
        let dy = dy / step;
        let mut x = x1 as isize;
        let mut y = y1 as isize;
        let mut i = 1;
        loop {
            if x > 0 && (x as usize) < self.width && y > 0 && (y as usize) < self.height {
                *self
                    .data
                    .get_mut(y as usize * self.width + x as usize)
                    .unwrap() = gray;
            }

            if i > step {
                break;
            }

            x = x + dx;
            y = y + dy;
            i += 1;
        }
    }
}
