use crate::color::Color;
use ndarray::Array2;

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Canvas {
    pixels: Array2::<Color>,
}

impl Canvas {
    pub(crate) fn to_ppm(&self) -> String {
        let header = format!("P3\n{} {}\n255\n", self.width().to_string(), self.height().to_string());
        let mut res = String::from(header);

        for h in 0..self.height() {
            let mut line_counter: usize = 0;
            let line_start_counter  = res.len();
            for w in 0..self.width() {
                let red = (self.pixels[[w, h]].red * 256.0) as u8;
                let green = (self.pixels[[w, h]].green * 256.0) as u8;
                let blue = (self.pixels[[w, h]].blue * 256.0) as u8;

                line_counter = Canvas::add_color_to_file(&mut res, line_counter, red, line_start_counter);
                line_counter = Canvas::add_color_to_file(&mut res, line_counter, green, line_start_counter);
                line_counter = Canvas::add_color_to_file(&mut res, line_counter, blue, line_start_counter);
            }
            res.push_str("\n");
        }

        return res;
    }

    fn add_color_to_file(res: &mut String, line_counter: usize, color: u8, line_start_counter: usize) -> usize {
        let mut local_counter = line_counter;
        let color_string = color.to_string();

        if color_string.len() + local_counter >= 70 {
            res.push_str(&format!("\n{}", color_string));
            local_counter = color_string.len();
        }
        else if line_start_counter == res.len() {// Missing test
            res.push_str( &color_string);
            local_counter = color_string.len();
        }
        else {
            res.push_str(&format!(" {}", color_string));
            local_counter = local_counter + 1 + color_string.len();
        }

        return local_counter;
    }

    pub(crate) fn write_pixel(&mut self, x: usize, y: usize, color: Color) {
        self.pixels[[x, y]] = color;
    }

    pub fn new(width: usize, height: usize) -> Self {
        Canvas { pixels: Array2::<Color>::default((width, height)) }
    }

    pub fn width(&self) -> usize {
        return self.pixels.shape()[0];
    }

    pub fn height(&self) -> usize {
        return self.pixels.shape()[1];
    }
}


#[cfg(test)]
mod tests {
    use crate::color::Color;
    use crate::canvas::Canvas;

    #[test]
    fn creating_a_canvas() {
        let c = Canvas::new(10, 20);

        assert_eq!(10, c.width());
        assert_eq!(20, c.height());

        for w in 0..c.width() {
            for h in 0..c.height() {
                assert_eq!(Color { red: 0.0, green: 0.0, blue: 0.0 }, c.pixels[[w, h]]);
            }
        }
    }

    #[test]
    fn writing_pixels_to_a_canvas() {
        let mut c = Canvas::new(10, 20);
        let red = Color { red: 1.0, blue: 0.0, green: 0.0 };

        c.write_pixel(2, 3, red);

        assert_eq!(red, c.pixels[[2, 3]]);
    }

    #[test]
    fn constructing_the_ppm_header() {
        let c = Canvas::new(5, 3);

        let ppm = c.to_ppm();

        assert!(ppm.starts_with("P3\n5 3\n255\n"));
    }

    #[test]
    fn constructing_the_ppm_pixel_data() {
        let mut c = Canvas::new(5, 3);

        let c1 = Color { red: 1.5, green: 0.0, blue: 0.0 };
        let c2 = Color { red: 0.0, green: 0.5, blue: 0.0 };
        let c3 = Color { red: -0.5, green: 0.0, blue: 1.0 };

        c.write_pixel(0, 0, c1);
        c.write_pixel(2, 1, c2);
        c.write_pixel(4, 2, c3);

        let ppm = c.to_ppm();

        let lines: Vec<&str> = ppm.lines().collect();

        assert_eq!("255 0 0 0 0 0 0 0 0 0 0 0 0 0 0", lines[3]);
        assert_eq!("0 0 0 0 0 0 0 128 0 0 0 0 0 0 0", lines[4]);
        assert_eq!("0 0 0 0 0 0 0 0 0 0 0 0 0 0 255", lines[5]);
    }

    #[test]
    fn splitting_long_lines_in_ppm_files() {
        let mut c = Canvas::new(10, 2);
        for w in 0..c.width() {
            for h in 0..c.height() {
                c.write_pixel(w, h, Color { red: 1.0, green: 0.8, blue: 0.6 })
            }
        }

        let ppm = c.to_ppm();

        let lines: Vec<&str> = ppm.lines().collect();

        assert_eq!("255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204", lines[3]);
        assert_eq!("153 255 204 153 255 204 153 255 204 153 255 204 153", lines[4]);
        assert_eq!("255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204", lines[5]);
        assert_eq!("153 255 204 153 255 204 153 255 204 153 255 204 153", lines[6]);
    }

    #[test]
    fn ppm_files_are_terminated_by_a_newline_character() {
        let c = Canvas::new(10, 2);

        let ppm = c.to_ppm();

        let last_char: char = ppm.chars().last().unwrap();

        assert_eq!('\n', last_char);
    }
}
