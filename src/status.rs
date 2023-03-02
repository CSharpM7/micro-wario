use super::*;


#[status_script(agent = "wario", status = FIGHTER_STATUS_KIND_CATCH, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe fn wario_catch_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_THROW_KIRBY_GROUND);
    return false.into();
}
#[status_script(agent = "wario", status = FIGHTER_STATUS_KIND_CATCH_ATTACK, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe fn wario_catch_attack_exec(fighter: &mut L2CFighterCommon) -> L2CValue {

    let defender= get_grabbed_opponent_boma(fighter.module_accessor);
    let currentFrame = MotionModule::frame(fighter.module_accessor);
    let cutOff = 10.0;
    let defenderAnim = if (currentFrame<=cutOff) 
    {Hash40::new("damage_elec")} //damage_air_3, damage_elec ottotto_wait
    else {Hash40::new("capture_wait_lw")};
    
    if MotionModule::motion_kind(defender) != defenderAnim.hash{
        MotionModule::change_motion(defender, defenderAnim, 1.0, 1.0, false, 0.0, false, false);
        MotionModule::set_rate(defender, 0.5);
        //MotionModule::change_motion_kind(defender, defenderAnim);
    }
    if (currentFrame<=cutOff+2.0){
        let lrRot = if (PostureModule::lr(fighter.module_accessor) <0.0) {0.0} else {180.0};
        let rot = Vector3f{x: 5.0, y: 0.0, z: 0.0};
        PostureModule::set_rot(
            defender,
            &rot,
            0
        );
        PostureModule::update_rot_y_lr(defender);
    } 
    else{
        let rot = Vector3f{x: 0.0, y: 0.0, z: 0.0};
        PostureModule::set_rot(
            defender,
            &rot,
            0
        );
    }
    return false.into();
}

#[status_script(agent = "wario", status = FIGHTER_STATUS_KIND_THROW, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe fn wario_throw_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let throwB = Hash40::new("throw_b");
    let throwDriver = Hash40::new("throw_hi");
    let currentFrame = MotionModule::frame(fighter.module_accessor);
    
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_THROW_KIRBY_GROUND);
    if MotionModule::motion_kind(fighter.module_accessor) == throwB.hash{
        let maxFrame = 46.0;
         // KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        // KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        if currentFrame < 2.0 || currentFrame>=maxFrame {
            sv_kinetic_energy!(
                reset_energy,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_CONTROL,
                ENERGY_CONTROLLER_RESET_TYPE_FREE,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0
            );
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        }
        if currentFrame < maxFrame
        {
            /* 
            sv_kinetic_energy!(
                set_stable_speed,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_CONTROL,
                vl::param_private::throw_b_speed_max,
                0.0
            );
            sv_kinetic_energy!(
                set_limit_speed,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_CONTROL,
                vl::param_private::throw_b_speed_max,
                0.0
            );
            sv_kinetic_energy!(
                controller_set_accel_x_add,
                fighter,
                vl::param_private::throw_b_accel
            );
            */
        }
    }
    return false.into();
}

#[status_script(agent = "wario", status = FIGHTER_STATUS_KIND_THROW_KIRBY, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn wario_throwk_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    return L2CFighterCommon::status_pre_ThrowKirby(fighter);
}
#[status_script(agent = "wario", status = FIGHTER_STATUS_KIND_THROW_KIRBY, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
unsafe fn wario_throwk_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    L2CFighterCommon::sub_status_uniq_process_ThrowKirby_initStatus(fighter);
    let hitStop = 8;
    WorkModule::set_int(fighter.module_accessor, hitStop, *FIGHTER_STATUS_THROW_WORK_INT_STOP_FRAME);

    return false.into();
}
#[status_script(agent = "wario", status = FIGHTER_STATUS_KIND_THROW_KIRBY, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn wario_throwk_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    return L2CFighterCommon::status_ThrowKirby(fighter);
}
#[status_script(agent = "wario", status = FIGHTER_STATUS_KIND_THROW_KIRBY, condition = LUA_SCRIPT_STATUS_FUNC_EXIT_STATUS)]
unsafe fn wario_throwk_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    return L2CFighterCommon::sub_status_uniq_process_ThrowKirby_exitStatus(fighter);
}
#[status_script(agent = "wario", status = FIGHTER_STATUS_KIND_THROW_KIRBY, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn wario_throwk_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    return L2CFighterCommon::status_end_ThrowKirby(fighter);
}

#[status_script(agent = "wario", status = FIGHTER_STATUS_KIND_THROW_KIRBY, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe fn wario_throwk_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let frameRise=8.0;
    let frameRiseStop=35.0;
    let frameFallStart = acmd::throwDriver::FRAME_FALL;
    let frameFallStartLoop = frameFallStart+1.0;
    let frameLand = acmd::throwDriver::FRAME_LAND+1.0;
    let currentFrame = MotionModule::frame(fighter.module_accessor);

    if (currentFrame > frameFallStart && currentFrame < frameFallStart+3.0)
    {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    }
    else if (currentFrame >= frameFallStartLoop && currentFrame < frameLand)
    {
        MotionModule::set_rate(fighter.module_accessor, 0.0);
        let speed = smash::phx::Vector3f { x: 0.0, y: -0.375, z: 0.0 };
        KineticModule::add_speed(fighter.module_accessor, &speed);
    }

    if GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32)
    && currentFrame > frameFallStart
    && currentFrame < frameLand
    {
        MotionModule::set_frame_sync_anim_cmd(fighter.module_accessor, frameLand, true,true,false);
        //MotionModule::set_frame(fighter.module_accessor, frameLand,false);
        println!("Landed!");
        MotionModule::set_rate(fighter.module_accessor, 1.0);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        LinkModule::send_event_nodes_throw(fighter.module_accessor, Hash40::new("throw_sync_motion"), Hash40::new("invalid"), true, 0, 0, 0);
    }

    return false.into();
}


#[status_script(agent = "wario", status = FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR, condition = LUA_SCRIPT_STATUS_FUNC_EXIT_STATUS)]
unsafe fn wario_landing_attack_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_merikomi"),false,true);
    return false.into();
}
#[status_script(agent = "wario", status = FIGHTER_STATUS_KIND_ATTACK_AIR, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe fn wario_attack_air_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let dairAnim = Hash40::new("attack_air_lw");
    let dairRiseAnim = Hash40::new("attack_air_lw2");
    
    if MotionModule::motion_kind(fighter.module_accessor) != dairAnim.hash{
        return false.into();
    }
    if (AttackModule::is_infliction_status(fighter.module_accessor,  *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD)){
        MotionModule::change_motion(fighter.module_accessor, dairRiseAnim, 18.0, 1.0, false, 0.0, false, false);
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_machstamp"),false,true);
        AttackModule::clear_all(fighter.module_accessor);
        println!("Request bounce");
    }
    
    return false.into();
}

pub fn install() {
    install_status_scripts!(
        wario_catch_attack_exec,
        wario_throw_exec,

        wario_throwk_pre,
        wario_throwk_init,
        wario_throwk_main,
        wario_throwk_exit,
        wario_throwk_end,
        wario_throwk_exec,

        wario_landing_attack_exit,
        wario_attack_air_exec
    );
}