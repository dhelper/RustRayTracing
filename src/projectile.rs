use crate::tuple::Tuple;

#[derive(Debug, Clone, Copy)]
pub struct Projectile {
    pub(crate) position : Tuple,
    pub(crate) velocity : Tuple
}