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

#[macro_use]
extern crate lazy_static;


#[skyline::from_offset(0x3ac540)]
pub fn get_battle_object_from_id(id: u32) -> *mut BattleObject;

unsafe fn get_grabbed_opponent_boma(attacker: *mut BattleObjectModuleAccessor) -> &'static mut BattleObjectModuleAccessor {
    let opponent_id = LinkModule::get_node_object_id(attacker, *LINK_NO_CAPTURE) as u32;
    let opponent_object = get_battle_object_from_id(opponent_id);
    return &mut *(*opponent_object).module_accessor
}
pub unsafe fn get_entry_from_boma(boma: *mut BattleObjectModuleAccessor) -> u32 {
    return WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32
}
pub unsafe fn get_entry(fighter: &mut L2CAgentBase) -> u32 {
    return get_entry_from_boma(fighter.module_accessor);
}




mod acmd;
mod status;
mod data;

pub const THROW_F_STATUS_KIND: i32 = 0x45;
pub const THROW_B_STATUS_KIND: i32 = 0x46;
pub const THROW_HI_STATUS_KIND: i32 = 0x47;
pub const THROW_LW_STATUS_KIND: i32 = 0x48;

unsafe fn agent_start(fighter: &mut L2CFighterCommon)
{
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
    if fighter_kind != *FIGHTER_KIND_WARIO {
        return;
    }
    let driver=THROW_HI_STATUS_KIND;
    fighter.global_table[driver].assign(&FIGHTER_STATUS_KIND_THROW_KIRBY.into());
}

#[smashline::fighter_init]
fn agent_init(fighter: &mut L2CFighterCommon) {
    unsafe {
        agent_start(fighter);
    }
}
#[fighter_reset]
fn agent_reset(fighter: &mut L2CFighterCommon) {
    unsafe {
        agent_start(fighter);
    }
}


std::arch::global_asm!(
    r#"
    .section .nro_header
    .global __nro_header_start
    .word 0
    .word _mod_header
    .word 0
    .word 0
    
    .section .rodata.module_name
        .word 0
        .word 5
        .ascii "warioland"
    .section .rodata.mod0
    .global _mod_header
    _mod_header:
        .ascii "MOD0"
        .word __dynamic_start - _mod_header
        .word __bss_start - _mod_header
        .word __bss_end - _mod_header
        .word __eh_frame_hdr_start - _mod_header
        .word __eh_frame_hdr_end - _mod_header
        .word __nx_module_runtime - _mod_header // runtime-generated module object offset
    .global IS_NRO
    IS_NRO:
        .word 1
    
    .section .bss.module_runtime
    __nx_module_runtime:
    .space 0xD0
    "#
);
#[no_mangle]
pub extern "C" fn main() {
    println!("[smashline_wario::main] Loading...");
    data::install();
    acmd::install();
    status::install();
    smashline::install_agent_init_callbacks!(
        agent_init
    );
    install_agent_resets!(
        agent_reset
    );
    println!("[smashline_wario::main] HERE I GO!");
}