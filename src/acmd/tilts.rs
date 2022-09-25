use super::*;

pub fn install() {
    install_acmd_scripts!(
        attacklw3_game,
        attacklw3_effect,
        attacklw3_expression,
        attacklw3_sound
    );
}

#[acmd_script( agent = "samus", script =  "game_attacklw3", category = ACMD_GAME)]
unsafe fn attacklw3_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=7)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("kneer"), Damage=6.0, Angle=70, KBG=50, FKB=0, BKB=60, Size=3.6, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.2, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=1, Part=0, Bone=hash40("footr"), Damage=6.0, Angle=70, KBG=50, FKB=0, BKB=60, Size=3.6, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.2, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			AttackModule::set_attack_height_all(smash::app::AttackHeight(*ATTACK_HEIGHT_LOW), false)
			ATK_SET_SHIELD_SETOFF_MUL_arg3(ID1=0, ID2=1, ShieldstunMul=0.2)
			AttackModule::set_add_reaction_frame(ID=0, Frames=-7.0, Unk=false)
			AttackModule::set_add_reaction_frame(ID=1, Frames=-7.0, Unk=false)
		}
		frame(Frame=18)
		if(is_excute){
			AttackModule::clear_all()
		}
    });
}

#[acmd_script( agent = "samus", script =  "effect_attacklw3", category = ACMD_EFFECT)]
unsafe fn attacklw3_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=5)
		if(is_excute){
			FOOT_EFFECT(hash40("sys_dash_smoke"), hash40("top"), -5, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false)
		}
    });
}

#[acmd_script( agent = "samus", script =  "expression_attacklw3", category = ACMD_EXPRESSION)]
unsafe fn attacklw3_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
}

#[acmd_script( agent = "samus", script =  "sound_attacklw3", category = ACMD_SOUND)]
unsafe fn attacklw3_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=6)
		if(is_excute){
			PLAY_SE(hash40("se_samus_swing_m"))
		}
    });
}