use super::*;

mod attack_lw3;
mod special_n;
mod special_s;
mod cshot;
mod supermissile;

pub fn install() {
    attack_lw3::install();
    special_n::install();
    special_s::install();
    cshot::install();
    supermissile::install();
}