use super::*;

mod bair;
mod catch;
mod throwF;
mod throwLw;

pub fn install() {
    bair::install();
    catch::install();
    throwF::install();
    throwLw::install();
}