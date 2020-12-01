pub struct Tuple {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

impl Tuple {
    pub fn is_vector(&self) -> bool{
        return self.w == 0.0;
    }
    pub fn is_point(&self) -> bool {
        return self.w == 1.0;
    }
}


#[cfg(test)]
mod tests {
    use crate::tuple::Tuple;

    #[test]
    fn tuple_with_w1_is_a_point() {
        let target = Tuple {
            x :4.3, y:-4.2, z:3.1, w:1.0};

        assert_eq!(true, target.is_point());
    }

    #[test]
    fn tuple_with_w1_is_not_a_vector() {
        let target = Tuple {
            x :4.3, y:-4.2, z:3.1, w:1.0};

        assert_eq!(false, target.is_vector());
    }

    #[test]
    fn tuple_with_w0_is_not_a_point() {
        let target = Tuple {
            x :4.3, y:-4.2, z:3.1, w:0.0};

        assert_eq!(false, target.is_point());
    }

    #[test]
    fn tuple_with_w0_is_a_vector() {
        let target = Tuple {
            x :4.3, y:-4.2, z:3.1, w:0.0};

        assert_eq!(true, target.is_vector());
    }
}