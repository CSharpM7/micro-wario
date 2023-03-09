use crate::data::gamemodes::is_ULTS;

mod bair;
mod dair;
mod catch;
mod throwF;
pub mod throwHi;
mod throwLw;

use crate::data::gamemodes::*;

pub fn install() {
    bair::install();
    dair::install();
    throwF::install();
    throwHi::install();
    catch::install();

    if !is_HDR(){
        throwLw::install();
    }
}