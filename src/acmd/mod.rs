use crate::data::gamemodes::*;

mod bair;
mod dair;
mod catch;
mod throwF;
pub mod throwHi;
mod throwLw;


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