use crate::tuple::Tuple;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct Projectile {
    pub(crate) position : Tuple,
    pub(crate) velocity : Tuple
}