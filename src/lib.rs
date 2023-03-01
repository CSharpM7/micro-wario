#![feature(
    concat_idents,
    proc_macro_hygiene
)]
#![allow(
    non_snake_case,
    unused
)]
#![deny(
    deprecated
)]


use smash::{
    lib::{
        L2CValue,
        LuaConst,
        lua_const::*
    },
    //lib::,
    app::{
        *,
        self,
        sv_animcmd::{
            frame,
            wait
        },
        lua_bind::*
    },
    hash40,
    lua2cpp::*,
    phx::*
};
use smash_script::{
    *,
    macros::*
};
use smashline::*;

#[skyline::from_offset(0x3ac540)]
pub fn get_battle_object_from_id(id: u32) -> *mut BattleObject;

unsafe fn get_grabbed_opponent_boma(attacker: *mut BattleObjectModuleAccessor) -> &'static mut BattleObjectModuleAccessor {
    let opponent_id = LinkModule::get_node_object_id(attacker, *LINK_NO_CAPTURE) as u32;
    let opponent_object = get_battle_object_from_id(opponent_id);
    return &mut *(*opponent_object).module_accessor
}



mod acmd;
mod status;

#[skyline::main(name = "smashline_warioland")]
pub fn main() {
    acmd::install();
    status::install();
}