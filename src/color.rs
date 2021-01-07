use core::ops;

#[derive(Copy, Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

impl Default for Color{
    fn default() -> Self {
        Color{red:0.0, green: 0.0, blue: 0.0}
    }
}

impl ops::Add<Color> for Color {
    type Output = Color;

    fn add(self, rhs: Color) -> Self::Output {
        return Color {
            red: self.red + rhs.red,
            green: self.green + rhs.green,
            blue: self.blue + rhs.blue
        };
    }
}

impl ops::Sub<Color> for Color {
    type Output = Color;

    fn sub(self, rhs: Color) -> Self::Output {
        return Color {
            red: self.red - rhs.red,
            green: self.green - rhs.green,
            blue: self.blue - rhs.blue
        };
    }
}

impl ops::Mul<f64> for Color {
    type Output = Color;

    fn mul(self, rhs: f64) -> Self::Output {
        return Color {
            red: self.red * rhs,
            green: self.green * rhs,
            blue: self.blue * rhs
        };
    }
}

impl ops::Mul<Color> for Color {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        return Color {
            red: self.red * rhs.red,
            green: self.green * rhs.green,
            blue: self.blue * rhs.blue
        };
    }
}

#[cfg(test)]
mod tests {
    use crate::color::Color;

    #[test]
    fn colors_are_red_green_blue_tuples() {
        let c = Color { red: -0.1, green: 0.2, blue: 0.3 };

        assert_eq!(-0.1, c.red);
        assert_eq!(0.2, c.green);
        assert_eq!(0.3, c.blue);
    }

    #[test]
    fn adding_colors() {
        let c1 = Color { red: 0.9, green: 0.6, blue: 0.75 };
        let c2 = Color { red: 0.7, green: 0.1, blue: 0.25 };

        let result = c1 + c2;

        let expected = Color { red: 1.6, green: 0.7, blue: 1.0 };
        assert_eq!(expected, result);
    }

    #[test]
    fn subtracting_colors() {
        let c1 = Color { red: 0.6, green: 0.6, blue: 0.75 };
        let c2 = Color { red: 0.3, green: 0.1, blue: 0.25 };

        let result = c1 - c2;

        let expected = Color { red: 0.3, green: 0.5, blue: 0.5 };
        assert_eq!(expected, result);
    }

    #[test]
    fn multiplying_a_color_by_a_scalar(){
        let c = Color { red: 0.2, green: 0.3, blue: 0.4 };

        let result = c * 2.0;

        let expected = Color { red: 0.4, green: 0.6, blue: 0.8 };
        assert_eq!(expected, result);
    }

    #[test]
    fn multiplying_colors(){
        let c1 = Color { red: 1.0, green: 0.2, blue: 0.5 };
        let c2 = Color { red: 0.9, green: 1.0, blue: 0.1 };

        let result = c1 * c2;

        let expected = Color { red: 0.9, green: 0.2, blue: 0.05 };
        assert_eq!(expected, result);
    }
}