use super::*;
//Deprecated 

unsafe fn dair_bounce(fighter: &mut L2CFighterCommon){
    let dairRiseAnim = Hash40::new("attack_air_lw2");
    if (AttackModule::is_infliction_status(fighter.module_accessor,  *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD)){
        MotionModule::change_motion(fighter.module_accessor, dairRiseAnim, 18.0, 1.0, false, 0.0, false, false);
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_machstamp"),false,true);
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[fighter_frame( agent = FIGHTER_KIND_WARIO )]
fn wario_update(fighter: &mut L2CFighterCommon) {
    unsafe {
        let status = StatusModule::status_kind(fighter.module_accessor);
        if status == *FIGHTER_STATUS_KIND_ATTACK_AIR
        {
            let dairAnim = Hash40::new("attack_air_lw");
            if MotionModule::motion_kind(fighter.module_accessor) == dairAnim.hash{
                dair_bounce(fighter);
            }
        }
    }
}


pub fn install() {
    smashline::install_agent_frames!(
        wario_update
    );
}