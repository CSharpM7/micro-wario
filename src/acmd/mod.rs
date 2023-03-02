use super::*;

mod bair;
mod dair;
mod catch;
mod throwF;
pub mod throwDriver;
mod throwLw;

pub fn install() {
    bair::install();
    dair::install();
    catch::install();
    throwF::install();
    throwDriver::install();
    throwLw::install();
}