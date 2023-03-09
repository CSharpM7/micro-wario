use super::super::*;

#[acmd_script( agent = "wario", script = "game_catchattack", category = ACMD_GAME)]
unsafe fn game_catchattack(fighter: &mut L2CAgentBase) {
    let boma = fighter.module_accessor;
    let entry = get_entry(fighter) as usize;
    let defender= get_grabbed_opponent_boma(fighter.module_accessor);
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 1.3, 361, 100, 30, 0, 5.0, 0.0, 10.0, 7.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_COIN, *ATTACK_REGION_PUNCH);
        AttackModule::set_damage_shake_scale(fighter.module_accessor, 0.0);
        AttackModule::set_attack_camera_quake_forced(fighter.module_accessor, 0, *CAMERA_QUAKE_KIND_NONE, false);
        AttackModule::set_catch_only_all(fighter.module_accessor, true, false);

        //defenderMotion_Damage = WorkModule::get_int64(defender, *FIGHTER_STATUS_DAMAGE_WORK_INT_MOTION_KIND);
        MotionModule::set_rate(defender, 0.0);
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        MotionModule::set_frame(defender, 0.0, false);
        //MotionModule::change_motion(defender, Hash40::new("damage_elec"), 1.0, 0.0, false, 0.0, false, false);
    }
    frame(fighter.lua_state_agent,12.0);
    if is_excute(fighter) {
        MotionModule::set_frame(defender, 14.0, false);
        MotionModule::set_rate(defender, 1.0);
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