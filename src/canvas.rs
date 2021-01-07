use crate::color::Color;
use ndarray::Array2;

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Canvas {
    pixels: Array2::<Color>,
}

impl Canvas {
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
                assert_eq!(Color { red: 0.0, green: 0.0, blue: 0.0 }, c.pixels[[w,h]]);
            }
        }
    }
}