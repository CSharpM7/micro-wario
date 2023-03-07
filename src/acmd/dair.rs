use super::super::*;
pub const FRAME_FALL: f32 = 15.0;

#[acmd_script( agent = "wario", script = "game_attackairlw", category = ACMD_GAME )]
unsafe fn game_attackairlw(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_LANDING_CLEAR_SPEED);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        macros::SET_SPEED_EX(fighter, 0, 0.8, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        JostleModule::set_status(fighter.module_accessor, false);
        KineticModule::suspend_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE);
    }
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(fighter.lua_state_agent, FRAME_FALL);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        macros::SET_SPEED_EX(fighter, 0, -3.8, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        JostleModule::set_status(fighter.module_accessor, true);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 270, 75, 0, 50, 5.2, 0.0, 0.8, 0.0, None,None,None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HEAD);
    }
    wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 60, 95, 0, 30, 5.0, 0.0, -0.8, 0.0, None,None,None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HEAD);
    }
    frame(fighter.lua_state_agent, 44.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 48.0);
    if macros::is_excute(fighter) {
        KineticModule::resume_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(fighter.lua_state_agent, 58.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE);
    }
}

#[acmd_script( agent = "wario", script = "effect_attackairlw", category = ACMD_EFFECT )]
unsafe fn effect_attackairlw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, FRAME_FALL+1.0);
    if macros::is_excute(fighter) {
        /* 
        macros::EFFECT(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 15, 0, -90, 0, 0, 1.1, 0, 1, 0, 0, 0, 0, true);*/
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_machstamp"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, true);
        LAST_EFFECT_SET_COLOR(fighter,1,1,0.625);
    }
    frame(fighter.lua_state_agent, 48.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_machstamp"),false,true);
    }
}

#[acmd_script( agent = "wario", script = "sound_attackairlw", category = ACMD_SOUND )]
unsafe fn sound_attackairlw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_wario_jump01"));
    }
    frame(fighter.lua_state_agent, FRAME_FALL);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_wario_attackair_l01"));
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("vc_wario_007"));
    }
}

#[acmd_script( agent = "wario", script = "expression_attackairlw", category = ACMD_EXPRESSION )]
unsafe fn expression_attackairlw(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
    }
    frame(fighter.lua_state_agent, FRAME_FALL);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
    }
}


#[acmd_script( agent = "wario", script = "game_landingairlw", category = ACMD_GAME )]
unsafe fn game_landingairlw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 60, 30, 0, 70, 4.0, 0.0, 2.0, -9.0, Some(0.0), Some(4.0), Some(9.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HEAD);
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "wario", script = "sound_landingairlw", category = ACMD_SOUND )]
unsafe fn sound_landingairlw(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_wario_landing03"));
    }
}

#[acmd_script( agent = "wario", script = "effect_landingairlw", category = ACMD_EFFECT )]
unsafe fn effect_landingairlw(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, -3, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_RATE(fighter, 0.8);


        macros::EFFECT(fighter, Hash40::new("sys_merikomi"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_merikomi"),false,true);
    }
}

#[acmd_script( agent = "wario", script = "expression_landingairlw", category = ACMD_EXPRESSION )]
unsafe fn expression_landingairlw(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_impact"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
    }
    frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        ItemModule::set_have_item_visibility(fighter.module_accessor, true, 0);
    }
}

#[acmd_script( agent = "wario", script = "game_attackairlw2", category = ACMD_GAME )]
unsafe fn game_attackairlw2(fighter: &mut L2CAgentBase) {
    let speed_max=0.75;

    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        KineticModule::resume_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE);
        SET_SPEED_EX(fighter, 0, 1.875, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        println!("WARIO BOUNCE!");
        sv_kinetic_energy!(
            set_stable_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            speed_max,
            0.0
        );
        sv_kinetic_energy!(
            set_limit_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            speed_max,
            0.0
        );
    }
    frame(fighter.lua_state_agent, 27.0);
    if macros::is_excute(fighter) {
    }
    frame(fighter.lua_state_agent, 37.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE);
    }
    frame(fighter.lua_state_agent, 40.0);
    if macros::is_excute(fighter) {
        KineticModule::resume_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}
#[acmd_script( agent = "wario", script = "effect_attackairlw2", category = ACMD_EFFECT )]
unsafe fn effect_attackairlw2(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_bound_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.5, 0, 1, 0, 0, 0, 0, true);
    }
}
#[acmd_script( agent = "wario", script = "sound_attackairlw2", category = ACMD_SOUND )]
unsafe fn sound_attackairlw2(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_wario_jump02"));
    }
}
#[acmd_script( agent = "wario", script = "expression_attackairlw2", category = ACMD_EXPRESSION )]
unsafe fn expression_attackairlw2(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {

    }
}

pub fn install() {
    install_acmd_scripts!(
        game_attackairlw,
        effect_attackairlw,
        sound_attackairlw,
        expression_attackairlw,
        
        game_landingairlw,
        effect_landingairlw,
        sound_landingairlw,
        expression_landingairlw,

        game_attackairlw2,
        effect_attackairlw2,
        sound_attackairlw2,
        expression_attackairlw
    );
}