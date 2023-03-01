use super::*;

#[status_script(agent = "wario", status = FIGHTER_STATUS_KIND_CATCH_ATTACK, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe fn wario_catch_attack_exec(fighter: &mut L2CFighterCommon) -> L2CValue {

    let defender= get_grabbed_opponent_boma(fighter.module_accessor);
    let currentFrame = MotionModule::frame(fighter.module_accessor);
    let cutOff = 10.0;
    let defenderAnim = if (currentFrame<=cutOff) 
    {Hash40::new("damage_elec")} //damage_air_3, damage_elec ottotto_wait
    else {Hash40::new("capture_pulled_lw")};
    
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
    /* 
    if VarModule::is_flag(fighter.battle_object, wario::status::flag::THROW_B_MOVE) {
        // KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        // KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        if !VarModule::is_flag(fighter.battle_object, wario::status::flag::THROW_B_CONTROL_RESET) {
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
            VarModule::on_flag(fighter.battle_object, wario::status::flag::THROW_B_CONTROL_RESET);
        }
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
    }
    */
    return false.into();
}

pub fn install() {
    install_status_scripts!(
        wario_throw_exec,
        wario_catch_attack_exec
    );
}