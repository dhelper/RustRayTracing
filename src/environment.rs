use crate::tuple::Tuple;

#[derive(Debug, Copy, Clone)]
pub struct Environment {
    pub(crate) gravity: Tuple,
    pub(crate) wind: Tuple,
}