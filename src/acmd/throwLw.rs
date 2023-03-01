use super::*;

#[acmd_script( agent = "wario", script = "game_throwlw", category = ACMD_GAME )]
unsafe fn game_throwlw(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 7.0, 130, 18, 0, 80, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 361, 100, 0, 30, 6.0, 0.0, 3.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HIP);
        AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
        CHECK_FINISH_CAMERA(fighter, -2, 0);
        lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.5);
    }
    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        let target = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
        AttackModule::clear_all(fighter.module_accessor);
    }
}
#[acmd_script( agent = "wario", script = "effect_throwlw", category = ACMD_EFFECT, low_priority )]
unsafe fn effect_throwlw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 15, 0, -90, 0, 0, 1.1, 0, 1, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 15, 0, -90, 0, 0, 1.1, 0, 1, 0, 0, 0, 0, true);
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, -3, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_RATE(fighter, 0.8);
        macros::EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, -3, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}
#[acmd_script( agent = "wario", script = "sound_throwlw", category = ACMD_SOUND, low_priority )]
unsafe fn sound_throwlw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_common_throw_01"));
    }
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_common_kick_hit_s"));
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_wario_jump02"));
    }
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_wario_rnd_attack"));
        macros::PLAY_SE(fighter, Hash40::new("se_common_throw_02"));
    }
    frame(fighter.lua_state_agent, 57.0);
    if macros::is_excute(fighter) {
        macros::PLAY_LANDING_SE(fighter, Hash40::new("se_wario_landing02"));
    }
}
#[acmd_script( agent = "wario", script = "expression_throwlw", category = ACMD_EXPRESSION, low_priority )]
unsafe fn expression_throwlw(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        lua_args!(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, *CAMERA_QUAKE_KIND_NONE);
        smash::app::sv_animcmd::FT_ATTACK_ABS_CAMERA_QUAKE(fighter.lua_state_agent);
        fighter.clear_lua_stack();
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_NONE, 3);
    }
    frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_L);
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
    }
    frame(fighter.lua_state_agent, 47.0);
    if macros::is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 4);
    }
}

pub fn install() {
    install_acmd_scripts!(
        game_throwlw,
        effect_throwlw,
        sound_throwlw,
        expression_throwlw
    );
}