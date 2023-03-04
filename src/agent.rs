use super::*;

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

pub fn install() {
    smashline::install_agent_init_callbacks!(
        agent_init
    );
    install_agent_resets!(
        agent_reset
    );
}