use super::super::*;
pub static mut CATCH_ANIM:[u64;8] = [0; 8];

#[acmd_script( agent = "wario", script = "game_catchattack", category = ACMD_GAME)]
unsafe fn game_catchattack(fighter: &mut L2CAgentBase) {
    let boma = fighter.module_accessor;
    let entry = get_entry(fighter) as usize;
    let defender= get_grabbed_opponent_boma(fighter.module_accessor);
    
    frame(fighter.lua_state_agent, 0.5);
    if is_excute(fighter) {
        if MotionModule::motion_kind(defender) != Hash40::new("damage_elec").hash {
            CATCH_ANIM[entry] = MotionModule::motion_kind(defender);
        }
        //MotionModule::change_motion_kind(defender, Hash40::new("damage_elec"));
        MotionModule::change_motion(defender, Hash40::new("damage_elec"), 1.0, 1.0, false, 0.0, false, false);
    }
    let defenderAnim_Wait =Hash40::new_raw(CATCH_ANIM[entry]);
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 0.9, 361, 100, 30, 0, 5.0, 0.0, 10.0, 7.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_COIN, *ATTACK_REGION_PUNCH);
        AttackModule::set_damage_shake_scale(fighter.module_accessor, 0.0);
        AttackModule::set_attack_camera_quake_forced(fighter.module_accessor, 0, *CAMERA_QUAKE_KIND_NONE, false);
        AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent,11.0);
    if is_excute(fighter) {
        MotionModule::change_motion(defender, defenderAnim_Wait, 1.0, 1.0, false, 0.0, false, false);
        //MotionModule::change_motion_kind(defender, defenderAnim_Wait);
    }
}
#[acmd_script( agent = "wario", script = "effect_catchattack", category = ACMD_EFFECT )]
unsafe fn effect_catchattack(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0.0, 7.25, 11.0, 260, 0, 0, 1.1, true);
        //macros::EFFECT_FLIP(fighter, Hash40::new("sys_attack_line"), Hash40::new("sys_attack_line"), Hash40::new("top"), -4, 6, -1, -20, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
    }
}


pub fn install() {
    install_acmd_scripts!(
        game_catchattack,
        effect_catchattack
    );
}