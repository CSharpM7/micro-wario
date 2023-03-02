use super::*;

mod bair;
mod catch;
mod throwF;
pub mod throwDriver;
mod throwLw;

pub fn install() {
    bair::install();
    catch::install();
    throwF::install();
    throwDriver::install();
    throwLw::install();
}