use crate::data::gamemodes::is_ULTS;

mod bair;
mod dair;
mod catch;
mod throwF;
pub mod throwDriver;
mod throwLw;

use crate::data::gamemodes::*;

pub fn install() {
    bair::install();
    dair::install();
    catch::install();
    throwF::install();
    throwDriver::install();

    if !is_HDR(){
        throwLw::install();
    }
}