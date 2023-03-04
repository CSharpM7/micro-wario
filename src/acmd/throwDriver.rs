use super::super::*;
pub const FRAME_FALL: f32 = 50.0;
pub const FRAME_LAND: f32 = 55.0;
pub const FRAME_THROW: f32 = 58.0;
pub static mut THROWHI_HEAVY:[bool;8] = [false; 8];



#[acmd_script( agent = "wario", script = "game_throwhi", category = ACMD_GAME )]
unsafe fn game_throwhi(fighter: &mut L2CAgentBase) {
    let entry = get_entry(fighter) as usize;
    if is_excute(fighter) {
        macros::FT_LEAVE_NEAR_OTTOTTO(fighter, -2, 3);

        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 10.0, 80, 28, 0, 80, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    //Affect hitbox size based on scale
    let opponent = get_grabbed_opponent_boma(fighter.module_accessor);
    let opponentScale = PostureModule::scale(opponent);
    let weight =  WorkModule::get_param_float(opponent, hash40("weight"), 0);
    let opponent_kind = utility::get_kind(&mut *opponent);
    //Factor in mini and megashroom
    let extraLarge = opponentScale > 0.5
    && (opponentScale > 1.5 || [
        *FIGHTER_KIND_KOOPA,
        *FIGHTER_KIND_KROOL,
        *FIGHTER_KIND_DONKEY,
        *FIGHTER_KIND_DEDEDE,
        *FIGHTER_KIND_GANON,
        *FIGHTER_KIND_LIZARDON,
        *FIGHTER_KIND_RIDLEY,
        *FIGHTER_KIND_ROBOT,
        *FIGHTER_KIND_ROSETTA //sorry about this one
    ].contains(&opponent_kind)); 
    let extraHeavy = weight>110.0;

    let factorSize = (if extraLarge {1.5} else {1.0})*opponentScale.max(1.0);
    let factorPower = if extraLarge || extraHeavy {1.2} else {1.0};
    //Add rate based on item status
    let mut addRate = 0.0;
    if opponentScale != PostureModule::scale(fighter.module_accessor)
    {
        addRate = if {opponentScale > PostureModule::scale(fighter.module_accessor)} {0.25} else {-0.25};
    }
    let leadHead = WorkModule::is_flag(opponent, *FIGHTER_INSTANCE_WORK_ID_FLAG_METAL)
    || WorkModule::is_flag(opponent, *FIGHTER_INSTANCE_WORK_ID_FLAG_GOLD);
    if leadHead {addRate += 0.375};
    
    let factorRate = (if extraLarge || extraHeavy {1.375} else {1.0})+addRate;

    println!("ExtraLarge: {} ExtraHeavy: {} Added Rate: {}",extraLarge,extraHeavy,addRate);
    THROWHI_HEAVY[entry] = extraLarge || extraHeavy;

    frame(fighter.lua_state_agent, 8.0);
    FT_MOTION_RATE(fighter, factorRate);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_THROW_FLAG_START_AIR);
    }
    //Going up//
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        macros::ATTACK_IGNORE_THROW(fighter, 0, 0, Hash40::new("throw"), 8.0, 50, 70, 0, 100, 5.5*factorSize, 0.0, -5.0, 0.0, Some(0.0), Some(2.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_THROW);
    }

    frame(fighter.lua_state_agent, 38.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    wait(fighter.lua_state_agent, 2.0);
    FT_MOTION_RATE(fighter, 1.0);

    frame(fighter.lua_state_agent, FRAME_FALL);
    if macros::is_excute(fighter) {
        macros::ATTACK_IGNORE_THROW(fighter, 0, 0, Hash40::new("rot"), 10.0*factorPower, 270, 90, 0, 15, 4.75*factorSize, 0.0, 0.0, 0.0, Some(0.0), Some(0.5), Some(0.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_THROW);
        macros::ATTACK_IGNORE_THROW(fighter, 1, 0, Hash40::new("rot"), 8.0*factorPower, 50, 70, 0, 100, 6.0*factorSize, 0.0, 0.0, 0.0, Some(0.0), Some(2.5), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_THROW);
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear(fighter.module_accessor,0,false);
    } 
    
    frame(fighter.lua_state_agent, FRAME_LAND+1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        
        macros::ATTACK_IGNORE_THROW(fighter, 0, 0, Hash40::new("top"), 10.0*factorPower, 270, 90, 0, 15, 2.5*factorSize, 0.0, 0.0, -3.5, Some(0.0), Some(0.0), Some(3.5), 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_THROW);

        macros::ATTACK_IGNORE_THROW(fighter, 1, 0, Hash40::new("rot"), 7.0, 65, 95, 0, 85, 8.25*factorSize, 0.0, 0.0, 0.0, None,None,None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_THROW);
        AttackModule::set_catch_only_all(fighter.module_accessor, true, false);

        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_THROW_FLAG_STOP);
        macros::CHECK_FINISH_CAMERA(fighter, 18, 4);
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        WorkModule::set_float(fighter.module_accessor, 5.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_FINISH_CAMERA_THROW_RAY_LENGTH);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_RAY_CHECK_FINISH_CAMERA_THROW);
        lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.5);
    } 
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        let target = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
        AttackModule::clear_all(fighter.module_accessor);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), false);
    }
}
#[acmd_script( agent = "wario", script = "effect_throwhi", category = ACMD_EFFECT )]
unsafe fn effect_throwhi(fighter: &mut L2CAgentBase) {
    let entry = get_entry(fighter) as usize;
    let opponent = get_grabbed_opponent_boma(fighter.module_accessor);
    let opponentScale = PostureModule::scale(opponent);

    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT_FLIP(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_YZ);
        macros::LAST_EFFECT_SET_RATE(fighter, 0.8);
    }
    frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("wario_attack_air_n"), Hash40::new("wario_attack_air_n"), Hash40::new("rot"), 0, 3, 0, 0, -90, 0, 1, true, *EF_FLIP_YZ, 1);
        macros::EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("wario_attack_air_n"), Hash40::new("wario_attack_air_n"), Hash40::new("rot"), 0, 2, 0, 0, -90, 0, 1.8, true, *EF_FLIP_YZ, 0.6);
    }
    frame(fighter.lua_state_agent, 28.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("wario_attack_air_n"), Hash40::new("wario_attack_air_n"), Hash40::new("rot"), 0, 4, 0, 0, -90, 0, 1, true, *EF_FLIP_YZ, 1);
        LAST_EFFECT_SET_RATE(fighter,1.25);
        macros::EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("wario_attack_air_n"), Hash40::new("wario_attack_air_n"), Hash40::new("rot"), 0, 5, 0, 0, -90, 0, 1.8, true, *EF_FLIP_YZ, 0.6);
        LAST_EFFECT_SET_RATE(fighter,1.25);
    }
    /*
    frame(fighter.lua_state_agent, 35.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("wario_attack_air_n"), Hash40::new("wario_attack_air_n"), Hash40::new("rot"), 0, 3, 0, 0, -90, 0, 1, true, *EF_FLIP_YZ, 1);
        macros::EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("wario_attack_air_n"), Hash40::new("wario_attack_air_n"), Hash40::new("rot"), 0, 4, 0, 0, -90, 0, 1.8, true, *EF_FLIP_YZ, 0.6);
    } */
    frame(fighter.lua_state_agent, FRAME_FALL-3.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("top"), 0, -1.0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("rot"), 0, 20, 0, 90, 0, 0, 1.2, true);
        macros::LAST_EFFECT_SET_RATE(fighter, 0.5);
    }


    frame(fighter.lua_state_agent, FRAME_LAND-1.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, -3, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_RATE(fighter, 0.8);
        macros::EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, -3, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    /* 
    wait(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_merikomi"),false,true);
    }*/
}
#[acmd_script( agent = "wario", script = "sound_throwhi", category = ACMD_SOUND )]
unsafe fn sound_throwhi(fighter: &mut L2CAgentBase) {
    let entry = get_entry(fighter) as usize;
    let opponent = get_grabbed_opponent_boma(fighter.module_accessor);
    let opponentScale = PostureModule::scale(opponent);
    
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_common_throw_01"));
    }
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_wario_jump02"));
    }
    frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_common_throw_01"));
    }
    frame(fighter.lua_state_agent, 32.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_common_throw_01"));
    }
    frame(fighter.lua_state_agent, FRAME_FALL-1.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("vc_wario_006")); //006,final01
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_common_throw_02"));
        //macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_wario_rnd_attack"));
    }
    frame(fighter.lua_state_agent, FRAME_LAND);
    if macros::is_excute(fighter) {
        macros::PLAY_LANDING_SE(fighter, Hash40::new("se_wario_landing02"));
    }
}

#[acmd_script( agent = "wario", script = "expression_throwhi", category = ACMD_EXPRESSION )]
unsafe fn expression_throwhi(fighter: &mut L2CAgentBase) {
    let entry = get_entry(fighter) as usize;
    let opponent = get_grabbed_opponent_boma(fighter.module_accessor);
    let opponentScale = PostureModule::scale(opponent);

    if macros::is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_L, 13);
    }
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 29.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 43.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 48.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_attackm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, FRAME_LAND);
    if macros::is_excute(fighter) {
        let mut quakeKind = if THROWHI_HEAVY[entry] {*CAMERA_QUAKE_KIND_XL} else {*CAMERA_QUAKE_KIND_L};   
        if opponentScale < 1.0 {quakeKind = *CAMERA_QUAKE_KIND_M};
        let rumbleKind = if THROWHI_HEAVY[entry] {"rbkind_explosion"} else {"rbkind_attackl"};   
        macros::QUAKE(fighter, quakeKind);
        macros::RUMBLE_HIT(fighter, Hash40::new(rumbleKind), 0);
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 4);
    }
}

pub fn install() {
    install_acmd_scripts!(
        game_throwhi,
        effect_throwhi,
        sound_throwhi,
        expression_throwhi
    );
}