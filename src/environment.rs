use crate::tuple::Tuple;

#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
pub struct Environment {
    pub(crate) gravity: Tuple,
    pub(crate) wind: Tuple,
}