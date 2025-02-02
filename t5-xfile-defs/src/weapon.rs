use alloc::{boxed::Box, vec::Vec};

use crate::{
    Error, ErrorKind, FatPointer, Ptr32, Ptr32ArrayConst, Result, ScriptString, T5XFileDeserialize,
    XFileDeserializeInto, XString, XStringRaw, assert_size,
    common::{Vec2, Vec3},
    file_line_col, fx, techset, xmodel,
};

use num::FromPrimitive;
use num_derive::FromPrimitive;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Clone, Default, Debug, Deserialize)]
pub(crate) struct WeaponVariantDefRaw<'a> {
    pub internal_name: XStringRaw<'a>,
    pub variant_count: i32,
    pub weap_def: Ptr32<'a, WeaponDefRaw<'a>>,
    pub display_name: XStringRaw<'a>,
    pub xanims: Ptr32ArrayConst<'a, XStringRaw<'a>, 66>,
    pub alt_weapon_name: XStringRaw<'a>,
    pub hide_tags: Ptr32ArrayConst<'a, ScriptString, 32>,
    pub alt_weapon_index: u32,
    pub clip_size: i32,
    pub reload_time: i32,
    pub reload_empty_time: i32,
    pub reload_quick_time: i32,
    pub reload_quick_empty_time: i32,
    pub ads_trans_in_time: i32,
    pub ads_trans_out_time: i32,
    pub alt_raise_time: i32,
    pub ammo_name: XStringRaw<'a>,
    pub ammo_index: i32,
    pub clip_name: XStringRaw<'a>,
    pub clip_index: i32,
    pub aim_assist_range_ads: f32,
    pub ads_sway_horiz_scale: f32,
    pub ads_sway_vert_scale: f32,
    pub ads_view_kick_center_speed: f32,
    pub hip_view_kick_center_speed: f32,
    pub ads_zoom_fov_1: f32,
    pub ads_zoom_fov_2: f32,
    pub ads_zoom_fov_3: f32,
    pub ads_zoom_in_frac: f32,
    pub ads_zoom_out_frac: f32,
    pub overlay_alpha_scale: f32,
    pub oo_pos_anim_length: [f32; 2],
    pub silenced: bool,
    pub dual_mag: bool,
    pub full_metal_jacket: bool,
    pub hollow_point: bool,
    pub rapid_fire: bool,
    pad: [u8; 3],
    pub overlay_material: Ptr32<'a, techset::MaterialRaw<'a>>,
    pub overlay_material_low_res: Ptr32<'a, techset::MaterialRaw<'a>>,
    pub dpad_icon: Ptr32<'a, techset::MaterialRaw<'a>>,
    pub dpad_icon_ratio: u32,
    pub left_hand_offset: [f32; 3],
    pub left_hand_rotation: [f32; 3],
    pub left_hand_prone_offset: [f32; 3],
    pub left_hand_prone_rotation: [f32; 3],
    pub left_hand_ui_viewer_offset: [f32; 3],
    pub left_hand_ui_viewer_rotation: [f32; 3],
}
assert_size!(WeaponVariantDefRaw, 228);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, FromPrimitive)]
pub enum WeaponIconRatioType {
    #[default]
    ONE_TO_ONE = 0,
    TWO_TO_ONE = 1,
    FOUR_TO_ONE = 2,
    COUNT = 3,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Default, Debug)]
pub struct WeaponVariantDef {
    pub internal_name: XString,
    pub variant_count: usize,
    pub weap_def: Option<Box<WeaponDef>>,
    pub display_name: XString,
    pub xanims: Vec<XString>,
    pub alt_weapon_name: XString,
    pub hide_tags: Vec<XString>,
    pub alt_weapon_index: u32,
    pub clip_size: i32,
    pub reload_time: i32,
    pub reload_empty_time: i32,
    pub reload_quick_time: i32,
    pub reload_quick_empty_time: i32,
    pub ads_trans_in_time: i32,
    pub ads_trans_out_time: i32,
    pub alt_raise_time: i32,
    pub ammo_name: XString,
    pub ammo_index: usize,
    pub clip_name: XString,
    pub clip_index: usize,
    pub aim_assist_range_ads: f32,
    pub ads_sway_horiz_scale: f32,
    pub ads_sway_vert_scale: f32,
    pub ads_view_kick_center_speed: f32,
    pub hip_view_kick_center_speed: f32,
    pub ads_zoom_fov_1: f32,
    pub ads_zoom_fov_2: f32,
    pub ads_zoom_fov_3: f32,
    pub ads_zoom_in_frac: f32,
    pub ads_zoom_out_frac: f32,
    pub overlay_alpha_scale: f32,
    pub oo_pos_anim_length: Vec2,
    pub silenced: bool,
    pub dual_mag: bool,
    pub full_metal_jacket: bool,
    pub hollow_point: bool,
    pub rapid_fire: bool,
    pub overlay_material: Option<Box<techset::Material>>,
    pub overlay_material_low_res: Option<Box<techset::Material>>,
    pub dpad_icon: Option<Box<techset::Material>>,
    pub dpad_icon_ratio: WeaponIconRatioType,
    pub left_hand_offset: Vec3,
    pub left_hand_rotation: Vec3,
    pub left_hand_prone_offset: Vec3,
    pub left_hand_prone_rotation: Vec3,
    pub left_hand_ui_viewer_offset: Vec3,
    pub left_hand_ui_viewer_rotation: Vec3,
}

impl<'a> XFileDeserializeInto<WeaponVariantDef, ()> for WeaponVariantDefRaw<'a> {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<WeaponVariantDef> {
        let internal_name = self.internal_name.xfile_deserialize_into(de, ())?;
        let weap_def = self.weap_def.xfile_deserialize_into(de, ())?;
        let display_name = self.display_name.xfile_deserialize_into(de, ())?;
        let xanims = self.xanims.xfile_deserialize_into(de, ())?;
        let alt_weapon_name = self.alt_weapon_name.xfile_deserialize_into(de, ())?;
        let hide_tags = self
            .hide_tags
            .to_vec(de)?
            .into_iter()
            .map(|s| XString(s.to_string(de).unwrap_or_default()))
            .collect();
        let ammo_name = self.ammo_name.xfile_deserialize_into(de, ())?;
        let clip_name = self.clip_name.xfile_deserialize_into(de, ())?;
        let oo_pos_anim_length = self.oo_pos_anim_length.into();
        let overlay_material = self.overlay_material.xfile_deserialize_into(de, ())?;
        let overlay_material_low_res = self
            .overlay_material_low_res
            .xfile_deserialize_into(de, ())?;
        let dpad_icon = self.dpad_icon.xfile_deserialize_into(de, ())?;
        let dpad_icon_ratio =
            FromPrimitive::from_u32(self.dpad_icon_ratio).ok_or(Error::new_with_offset(
                file_line_col!(),
                de.stream_pos()? as _,
                ErrorKind::BadFromPrimitive(self.dpad_icon_ratio as _),
            ))?;
        let left_hand_offset = self.left_hand_offset.into();
        let left_hand_rotation = self.left_hand_rotation.into();
        let left_hand_prone_offset = self.left_hand_prone_offset.into();
        let left_hand_prone_rotation = self.left_hand_prone_rotation.into();
        let left_hand_ui_viewer_offset = self.left_hand_ui_viewer_offset.into();
        let left_hand_ui_viewer_rotation = self.left_hand_ui_viewer_rotation.into();

        Ok(WeaponVariantDef {
            internal_name,
            variant_count: self.variant_count as _,
            weap_def,
            display_name,
            xanims,
            alt_weapon_name,
            hide_tags,
            alt_weapon_index: self.alt_weapon_index,
            clip_size: self.clip_size,
            reload_time: self.reload_time,
            reload_empty_time: self.reload_empty_time,
            reload_quick_time: self.reload_quick_time,
            reload_quick_empty_time: self.reload_quick_empty_time,
            ads_trans_in_time: self.ads_trans_in_time,
            ads_trans_out_time: self.ads_trans_out_time,
            alt_raise_time: self.alt_raise_time,
            ammo_name,
            ammo_index: self.ammo_index as _,
            clip_name,
            clip_index: self.clip_index as _,
            aim_assist_range_ads: self.aim_assist_range_ads,
            ads_sway_horiz_scale: self.ads_sway_horiz_scale,
            ads_sway_vert_scale: self.ads_sway_vert_scale,
            ads_view_kick_center_speed: self.ads_view_kick_center_speed,
            hip_view_kick_center_speed: self.hip_view_kick_center_speed,
            ads_zoom_fov_1: self.ads_zoom_fov_1,
            ads_zoom_fov_2: self.ads_zoom_fov_2,
            ads_zoom_fov_3: self.ads_zoom_fov_3,
            ads_zoom_in_frac: self.ads_zoom_in_frac,
            ads_zoom_out_frac: self.ads_zoom_out_frac,
            overlay_alpha_scale: self.overlay_alpha_scale,
            oo_pos_anim_length,
            silenced: self.silenced,
            dual_mag: self.dual_mag,
            full_metal_jacket: self.full_metal_jacket,
            hollow_point: self.hollow_point,
            rapid_fire: self.rapid_fire,
            overlay_material,
            overlay_material_low_res,
            dpad_icon,
            dpad_icon_ratio,
            left_hand_offset,
            left_hand_rotation,
            left_hand_prone_offset,
            left_hand_prone_rotation,
            left_hand_ui_viewer_offset,
            left_hand_ui_viewer_rotation,
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Clone, Default, Debug, Deserialize)]
pub(crate) struct WeaponDefRaw<'a> {
    pub overlay_name: XStringRaw<'a>,
    pub gun_xmodel: Ptr32ArrayConst<'a, Ptr32<'a, xmodel::XModelRaw<'a>>, 16>,
    pub hand_xmodel: Ptr32<'a, xmodel::XModelRaw<'a>>,
    pub mode_name: XStringRaw<'a>,
    pub notetrack_sound_map_keys: Ptr32ArrayConst<'a, ScriptString, 20>,
    pub notetrack_sound_map_values: Ptr32ArrayConst<'a, ScriptString, 20>,
    pub player_anim_type: i32,
    pub weap_type: u32,
    pub weap_class: u32,
    pub penetrate_type: u32,
    pub impact_type: u32,
    pub inventory_type: u32,
    pub fire_type: u32,
    pub clip_type: u32,
    pub item_index: i32,
    pub parent_weapon_name: XStringRaw<'a>,
    pub jam_fire_time: i32,
    pub tracer_frequency: i32,
    pub tracer_width: f32,
    pub tracer_length: f32,
    pub overheat_weapon: i32,
    pub overheat_rate: f32,
    pub cooldown_rate: f32,
    pub overheat_end_val: f32,
    pub cool_while_firing: bool,
    pub fuel_tank_weapon: bool,
    #[allow(dead_code)]
    pad: [u8; 2],
    pub tank_life_time: i32,
    pub offhand_class: u32,
    pub offhand_slot: u32,
    pub stance: u32,
    pub view_flash_effect: Ptr32<'a, fx::FxEffectDefRaw<'a>>,
    pub world_flash_effect: Ptr32<'a, fx::FxEffectDefRaw<'a>>,
    pub pickup_sound: XStringRaw<'a>,
    pub pickup_sound_player: XStringRaw<'a>,
    pub ammo_pickup_sound: XStringRaw<'a>,
    pub ammo_pickup_sound_player: XStringRaw<'a>,
    pub projectile_sound: XStringRaw<'a>,
    pub pullback_sound: XStringRaw<'a>,
    pub pullback_sound_player: XStringRaw<'a>,
    pub fire_sound: XStringRaw<'a>,
    pub fire_sound_player: XStringRaw<'a>,
    pub fire_loop_sound: XStringRaw<'a>,
    pub fire_loop_sound_player: XStringRaw<'a>,
    pub fire_loop_end_sound: XStringRaw<'a>,
    pub fire_loop_end_sound_player: XStringRaw<'a>,
    pub fire_stop_sound: XStringRaw<'a>,
    pub fire_stop_sound_player: XStringRaw<'a>,
    pub fire_last_sound: XStringRaw<'a>,
    pub fire_last_sound_player: XStringRaw<'a>,
    pub empty_fire_sound: XStringRaw<'a>,
    pub empty_fire_sound_player: XStringRaw<'a>,
    pub crack_sound: XStringRaw<'a>,
    pub whiz_by_sound: XStringRaw<'a>,
    pub melee_swipe_sound: XStringRaw<'a>,
    pub melee_swipe_sound_player: XStringRaw<'a>,
    pub melee_hit_sound: XStringRaw<'a>,
    pub melee_miss_sound: XStringRaw<'a>,
    pub rechamber_sound: XStringRaw<'a>,
    pub rechamber_sound_player: XStringRaw<'a>,
    pub reload_sound: XStringRaw<'a>,
    pub reload_sound_player: XStringRaw<'a>,
    pub reload_empty_sound: XStringRaw<'a>,
    pub reload_empty_sound_player: XStringRaw<'a>,
    pub reload_start_sound: XStringRaw<'a>,
    pub reload_start_sound_player: XStringRaw<'a>,
    pub reload_end_sound: XStringRaw<'a>,
    pub reload_end_sound_player: XStringRaw<'a>,
    pub rotate_loop_sound: XStringRaw<'a>,
    pub rotate_loop_sound_player: XStringRaw<'a>,
    pub deploy_sound: XStringRaw<'a>,
    pub deploy_sound_player: XStringRaw<'a>,
    pub finish_deploy_sound: XStringRaw<'a>,
    pub finish_deploy_sound_player: XStringRaw<'a>,
    pub breakdown_sound: XStringRaw<'a>,
    pub breakdown_sound_player: XStringRaw<'a>,
    pub finish_breakdown_sound: XStringRaw<'a>,
    pub finish_breakdown_sound_player: XStringRaw<'a>,
    pub detonate_sound: XStringRaw<'a>,
    pub detonate_sound_player: XStringRaw<'a>,
    pub night_vision_wear_sound: XStringRaw<'a>,
    pub night_vision_wear_sound_player: XStringRaw<'a>,
    pub night_vision_remove_sound: XStringRaw<'a>,
    pub night_vision_remove_sound_player: XStringRaw<'a>,
    pub alt_switch_sound: XStringRaw<'a>,
    pub alt_switch_sound_player: XStringRaw<'a>,
    pub raise_sound: XStringRaw<'a>,
    pub raise_sound_player: XStringRaw<'a>,
    pub first_raise_sound: XStringRaw<'a>,
    pub first_raise_sound_player: XStringRaw<'a>,
    pub put_away_sound: XStringRaw<'a>,
    pub put_away_sound_player: XStringRaw<'a>,
    pub overheat_sound: XStringRaw<'a>,
    pub overheat_sound_player: XStringRaw<'a>,
    pub ads_zoom_sound: XStringRaw<'a>,
    pub bounce_sound: Ptr32ArrayConst<'a, XStringRaw<'a>, 31>,
    pub stand_mounted_weapdef: XStringRaw<'a>,
    pub crouch_mounted_weapdef: XStringRaw<'a>,
    pub prone_mounted_weapdef: XStringRaw<'a>,
    pub stand_mounted_index: i32,
    pub crouch_mounted_index: i32,
    pub prone_mounted_index: i32,
    pub view_shell_eject_effect: Ptr32<'a, fx::FxEffectDefRaw<'a>>,
    pub world_shell_eject_effect: Ptr32<'a, fx::FxEffectDefRaw<'a>>,
    pub view_last_shot_eject_effect: Ptr32<'a, fx::FxEffectDefRaw<'a>>,
    pub world_last_shot_eject_effect: Ptr32<'a, fx::FxEffectDefRaw<'a>>,
    pub reticle_center: Ptr32<'a, techset::MaterialRaw<'a>>,
    pub reticle_side: Ptr32<'a, techset::MaterialRaw<'a>>,
    pub reticle_center_size: i32,
    pub reticle_side_size: i32,
    pub reticle_min_ofs: i32,
    pub active_reticle_type: u32,
    pub stand_move: [f32; 3],
    pub stand_rot: [f32; 3],
    pub ducked_ofs: [f32; 3],
    pub ducked_move: [f32; 3],
    pub ducked_sprint_ofs: [f32; 3],
    pub ducked_sprint_rot: [f32; 3],
    pub ducked_sprint_bob: [f32; 2],
    pub ducked_sprint_cycle_scale: f32,
    pub sprint_ofs: [f32; 3],
    pub sprint_rot: [f32; 3],
    pub sprint_bob: [f32; 2],
    pub sprint_cycle_scale: f32,
    pub low_ready_ofs: [f32; 3],
    pub low_ready_rot: [f32; 3],
    pub dtp_ofs: [f32; 3],
    pub dtp_rot: [f32; 3],
    pub dtp_bob: [f32; 2],
    pub dtp_cycle_scale: f32,
    pub mantle_ofs: [f32; 3],
    pub mantle_rot: [f32; 3],
    pub slide_ofs: [f32; 3],
    pub slide_rot: [f32; 3],
    pub ducked_rot: [f32; 3],
    pub prone_ofs: [f32; 3],
    pub prone_move: [f32; 3],
    pub prone_rot: [f32; 3],
    pub strafe_move: [f32; 3],
    pub strafe_rot: [f32; 3],
    pub pos_move_rate: f32,
    pub pos_prone_move_rate: f32,
    pub stand_move_min_speed: f32,
    pub ducked_move_min_speed: f32,
    pub prone_move_min_speed: f32,
    pub pos_rot_rate: f32,
    pub pos_prone_rot_rate: f32,
    pub stand_rot_min_speed: f32,
    pub ducked_rot_min_speed: f32,
    pub prone_rot_min_speed: f32,
    pub world_model: Ptr32ArrayConst<'a, Ptr32<'a, xmodel::XModelRaw<'a>>, 16>,
    pub world_clip_model: Ptr32<'a, xmodel::XModelRaw<'a>>,
    pub rocket_model: Ptr32<'a, xmodel::XModelRaw<'a>>,
    pub mounted_model: Ptr32<'a, xmodel::XModelRaw<'a>>,
    pub additional_melee_model: Ptr32<'a, xmodel::XModelRaw<'a>>,
    pub hud_icon: Ptr32<'a, techset::MaterialRaw<'a>>,
    pub hud_icon_ratio: u32,
    pub indicator_icon: Ptr32<'a, techset::MaterialRaw<'a>>,
    pub indicator_icon_ratio: u32,
    pub ammo_counter_icon: Ptr32<'a, techset::MaterialRaw<'a>>,
    pub ammo_counter_icon_ratio: u32,
    pub ammo_counter_clip: u32,
    pub start_ammo: i32,
    pub head_index: i32,
    pub max_ammo: i32,
    pub shot_count: i32,
    pub shared_ammo_cap_name: XStringRaw<'a>,
    pub shared_ammo_cap_index: i32,
    pub shared_ammo_cap: i32,
    pub unlimited_ammo: bool,
    pub ammo_count_clip_relative: bool,
    #[allow(dead_code)]
    pad2: [u8; 2],
    pub damage: i32,
    pub damage_duration: f32,
    pub damage_interval: f32,
    pub player_damage: i32,
    pub melee_damage: i32,
    pub damage_type: i32,
    pub explosion_tag: ScriptString,
    #[allow(dead_code)]
    pad3: [u8; 2],
    pub fire_delay: i32,
    pub melee_delay: i32,
    pub melee_charge_delay: i32,
    pub detonate_delay: i32,
    pub spin_up_time: i32,
    pub spin_down_time: i32,
    pub spin_rate: f32,
    pub spin_loop_sound: XStringRaw<'a>,
    pub spin_loop_sound_player: XStringRaw<'a>,
    pub start_spin_sound: XStringRaw<'a>,
    pub start_spin_sound_player: XStringRaw<'a>,
    pub stop_spin_sound: XStringRaw<'a>,
    pub stop_spin_sound_player: XStringRaw<'a>,
    pub fire_time: i32,
    pub last_fire_time: i32,
    pub rechamber_time: i32,
    pub rechamber_bolt_time: i32,
    pub hold_fire_time: i32,
    pub detonate_fire_time: i32,
    pub melee_time: i32,
    pub melee_charge_time: i32,
    pub reload_time_right: i32,
    pub reload_time_left: i32,
    pub reload_show_rocket_time: i32,
    pub reload_empty_time_left: i32,
    pub reload_add_time: i32,
    pub reload_empty_add_time: i32,
    pub reload_quick_add_time: i32,
    pub reload_quick_empty_add_time: i32,
    pub reload_start_time: i32,
    pub reload_start_add_time: i32,
    pub reload_end_time: i32,
    pub drop_time: i32,
    pub raise_time: i32,
    pub alt_drop_time: i32,
    pub quick_drop_time: i32,
    pub quick_raise_time: i32,
    pub first_raise_time: i32,
    pub empty_raise_time: i32,
    pub empty_drop_time: i32,
    pub sprint_in_time: i32,
    pub sprint_loop_time: i32,
    pub sprint_out_time: i32,
    pub low_ready_in_time: i32,
    pub low_ready_loop_time: i32,
    pub low_ready_out_time: i32,
    pub cont_fire_in_time: i32,
    pub cont_fire_loop_time: i32,
    pub cont_fire_out_time: i32,
    pub dtp_in_time: i32,
    pub dtp_loop_time: i32,
    pub dtp_out_time: i32,
    pub slide_in_time: i32,
    pub deploy_time: i32,
    pub breakdown_time: i32,
    pub night_vision_wear_time: i32,
    pub night_vision_wear_time_fade_out_end: i32,
    pub night_vision_wear_time_power_up: i32,
    pub night_vision_remove_time: i32,
    pub night_vision_remove_time_power_down: i32,
    pub night_vision_remove_time_fade_in_start: i32,
    pub fuse_time: i32,
    pub ai_fuse_time: i32,
    pub lock_on_radius: i32,
    pub lock_on_speed: i32,
    pub require_lockon_to_fire: bool,
    pub no_ads_when_mag_empty: bool,
    pub avoid_drop_cleanup: bool,
    #[allow(dead_code)]
    pad4: [u8; 1],
    pub stack_fire: u32,
    pub stack_fire_spread: f32,
    pub stack_fire_accuracy_decay: f32,
    pub stack_sound: XStringRaw<'a>,
    pub auto_aim_range: f32,
    pub aim_assist_range: f32,
    pub mountable_weapon: bool,
    #[allow(dead_code)]
    pad5: [u8; 3],
    pub aim_padding: f32,
    pub enemy_crosshair_range: f32,
    pub crosshair_color_change: bool,
    pad6: [u8; 3],
    pub move_speed_scale: f32,
    pub ads_move_speed_scale: f32,
    pub sprint_duration_scale: f32,
    pub overlay_reticle: u32,
    pub overlay_interface: u32,
    pub overlay_width: f32,
    pub overlay_height: f32,
    pub ads_bob_factor: f32,
    pub ads_view_bob_mult: f32,
    pub hip_spread_stand_min: f32,
    pub hip_spread_ducked_min: f32,
    pub hip_spread_prone_min: f32,
    pub hip_spread_stand_max: f32,
    pub hip_spread_ducked_max: f32,
    pub hip_spread_prone_max: f32,
    pub hip_spread_decay_rate: f32,
    pub hip_spread_fire_add: f32,
    pub hip_spread_turn_add: f32,
    pub hip_spread_move_add: f32,
    pub hip_spread_ducked_decay: f32,
    pub hip_spread_prone_decay: f32,
    pub hip_reticle_side_pos: f32,
    pub ads_idle_amount: f32,
    pub hip_idle_amount: f32,
    pub ads_idle_speed: f32,
    pub hip_idle_speed: f32,
    pub idle_crouch_factor: f32,
    pub idle_prone_factor: f32,
    pub gun_max_pitch: f32,
    pub gun_max_yaw: f32,
    pub sway_max_angle: f32,
    pub sway_lerp_speed: f32,
    pub sway_pitch_scale: f32,
    pub sway_yaw_scale: f32,
    pub sway_horiz_scale: f32,
    pub sway_vert_scale: f32,
    pub sway_shell_shock_scale: f32,
    pub ads_sway_max_angle: f32,
    pub ads_sway_lerp_speed: f32,
    pub ads_sway_pitch_scale: f32,
    pub ads_sway_yaw_scale: f32,
    pub shared_ammo: bool,
    pub rifle_bullet: bool,
    pub armor_piercing: bool,
    pub bolt_action: bool,
    pub use_alt_tag_flesh: bool,
    pub use_anti_lag_rewind: bool,
    pub is_carried_killstreak_weapon: bool,
    pub aim_down_sight: bool,
    pub rechamber_while_ads: bool,
    pub reload_while_ads: bool,
    #[allow(dead_code)]
    pad7: [u8; 2],
    pub ads_view_error_min: f32,
    pub ads_view_error_max: f32,
    pub cook_off_hold: bool,
    pub clip_only: bool,
    pub can_use_in_vehicle: bool,
    pub no_drops_or_raises: bool,
    pub ads_fire_only: bool,
    pub cancel_auto_holster_when_empty: bool,
    pub suppress_ammo_reserve_display: bool,
    pub laser_sight_during_nightvision: bool,
    pub hide_third_person: bool,
    pub has_bayonet: bool,
    pub dual_wield: bool,
    pub explode_on_ground: bool,
    pub throw_back: bool,
    pub retrievable: bool,
    pub die_on_respawn: bool,
    pub no_third_person_drops_or_raises: bool,
    pub continuous_fire: bool,
    pub no_ping: bool,
    pub force_bounce: bool,
    pub use_dropped_model_as_stowed: bool,
    pub no_quick_drop_when_empty: bool,
    pub keep_crosshair_when_ads: bool,
    pub use_only_alt_weaopon_hide_tags_in_alt_mode: bool,
    #[allow(dead_code)]
    pad8: [u8; 1],
    pub kill_icon: Ptr32<'a, techset::MaterialRaw<'a>>,
    pub kill_icon_ratio: u32,
    pub flip_kill_icon: bool,
    pub no_partial_reload: bool,
    pub segmented_reload: bool,
    pub no_ads_auto_reload: bool,
    pub reload_ammo_add: i32,
    pub reload_start_add: i32,
    pub spawned_grenade_weapon_name: XStringRaw<'a>,
    pub dual_wield_weapon_name: XStringRaw<'a>,
    pub dual_wield_weapon_index: u32,
    pub drop_ammo_min: i32,
    pub drop_ammo_max: i32,
    pub drop_clip_ammo_min: i32,
    pub drop_clip_ammo_max: i32,
    pub blocks_prone: bool,
    pub show_indicator: bool,
    #[allow(dead_code)]
    pad9: [u8; 2],
    pub is_rolling_grenade: i32,
    pub explosion_radius: i32,
    pub explosion_radius_min: i32,
    pub indicator_radius: i32,
    pub explosion_inner_damage: i32,
    pub explosion_outer_damage: i32,
    pub damage_cone_angle: f32,
    pub projectile_speed: i32,
    pub projectile_speed_up: i32,
    pub projectile_speed_relative_up: i32,
    pub projectile_speed_forward: i32,
    pub projectile_active_dist: i32,
    pub proj_lifetime: f32,
    pub time_to_accelerate: f32,
    pub projectile_curvature: f32,
    pub projectile_model: Ptr32<'a, xmodel::XModelRaw<'a>>,
    pub proj_explosion: u32,
    pub proj_explosion_effect: Ptr32<'a, fx::FxEffectDefRaw<'a>>,
    pub proj_explosion_effect_force_normal_up: bool,
    #[allow(dead_code)]
    pad10: [u8; 3],
    pub proj_explosion_effect_2: Ptr32<'a, fx::FxEffectDefRaw<'a>>,
    pub proj_explosion_effect_2_force_normal_up: bool,
    #[allow(dead_code)]
    pad11: [u8; 3],
    pub proj_explosion_effect_3: Ptr32<'a, fx::FxEffectDefRaw<'a>>,
    pub proj_explosion_effect_3_force_normal_up: bool,
    #[allow(dead_code)]
    pad12: [u8; 3],
    pub proj_explosion_effect_4: Ptr32<'a, fx::FxEffectDefRaw<'a>>,
    pub proj_explosion_effect_4_force_normal_up: bool,
    #[allow(dead_code)]
    pad13: [u8; 3],
    pub proj_explosion_effect_5: Ptr32<'a, fx::FxEffectDefRaw<'a>>,
    pub proj_explosion_effect_5_force_normal_up: bool,
    #[allow(dead_code)]
    pad14: [u8; 3],
    pub proj_dud_effect: Ptr32<'a, fx::FxEffectDefRaw<'a>>,
    pub proj_explosion_sound: XStringRaw<'a>,
    pub proj_dud_sound: XStringRaw<'a>,
    pub mortar_shell_sound: XStringRaw<'a>,
    pub tank_shell_sound: XStringRaw<'a>,
    pub proj_impact_explode: bool,
    pub bullet_impact_explode: bool,
    #[allow(dead_code)]
    pad15: [u8; 2],
    pub stickiness: u32,
    pub rotate_type: u32,
    pub plantable: bool,
    pub has_detonator: bool,
    pub time_detonation: bool,
    pub no_crumple_missile: bool,
    pub rotate: bool,
    pub keep_rolling: bool,
    pub hold_button_to_throw: bool,
    pub offhand_hold_is_cancelable: bool,
    pub freeze_movement_when_firing: bool,
    #[allow(dead_code)]
    pad16: [u8; 3],
    pub low_ammo_warning_threshold: f32,
    pub melee_charge_range: f32,
    pub use_as_melee: bool,
    pub is_camera_sensor: bool,
    pub is_acoustic_sensor: bool,
    #[allow(dead_code)]
    pad17: [u8; 1],
    pub parallel_bounce: Ptr32ArrayConst<'a, f32, 31>,
    pub perpendicular_bounce: Ptr32ArrayConst<'a, f32, 31>,
    pub proj_tail_effect: Ptr32<'a, fx::FxEffectDefRaw<'a>>,
    pub projectile_color: [f32; 3],
    pub guided_missile_type: u32,
    pub max_steering_accel: f32,
    pub proj_ignition_delay: i32,
    pub proj_ignition_effect: Ptr32<'a, fx::FxEffectDefRaw<'a>>,
    pub proj_ignition_sound: XStringRaw<'a>,
    pub ads_aim_pitch: f32,
    pub ads_crosshair_in_frac: f32,
    pub ads_crosshair_out_frac: f32,
    pub ads_gun_kick_reduced_kick_bullets: i32,
    pub ads_gun_kick_reduced_kick_percent: f32,
    pub ads_gun_kick_pitch_min: f32,
    pub ads_gun_kick_pitch_max: f32,
    pub ads_gun_kick_yaw_min: f32,
    pub ads_gun_kick_yaw_max: f32,
    pub ads_gun_kick_accel: f32,
    pub ads_gun_kick_speed_max: f32,
    pub ads_gun_kick_speed_decay: f32,
    pub ads_gun_kick_static_decay: f32,
    pub ads_view_kick_pitch_min: f32,
    pub ads_view_kick_pitch_max: f32,
    pub ads_view_kick_yaw_min: f32,
    pub ads_view_kick_yaw_max: f32,
    pub ads_view_scatter_min: f32,
    pub ads_view_scatter_max: f32,
    pub ads_spread: f32,
    pub hip_gun_kick_reduced_kick_bullets: i32,
    pub hip_gun_kick_reduced_kick_percent: f32,
    pub hip_gun_kick_pitch_min: f32,
    pub hip_gun_kick_pitch_max: f32,
    pub hip_gun_kick_yaw_min: f32,
    pub hip_gun_kick_yaw_max: f32,
    pub hip_gun_kick_accel: f32,
    pub hip_gun_kick_speed_max: f32,
    pub hip_gun_kick_speed_decay: f32,
    pub hip_gun_kick_static_decay: f32,
    pub hip_view_kick_pitch_min: f32,
    pub hip_view_kick_pitch_max: f32,
    pub hip_view_kick_yaw_min: f32,
    pub hip_view_kick_yaw_max: f32,
    pub hip_view_scatter_min: f32,
    pub hip_view_scatter_max: f32,
    pub fight_dist: f32,
    pub max_dist: f32,
    pub accuracy_graph_name: [XStringRaw<'a>; 2],
    pub accuracy_graph_knots: [Ptr32<'a, [f32; 2]>; 2],
    pub original_accuracy_graph_knots: [Ptr32<'a, [f32; 2]>; 2],
    pub accuracy_graph_knot_count: [i32; 2],
    pub original_accuracy_graph_knot_count: [i32; 2],
    pub position_reload_trans_time: i32,
    pub left_arc: f32,
    pub right_arc: f32,
    pub top_arc: f32,
    pub bottom_arc: f32,
    pub accuracy: f32,
    pub ai_spread: f32,
    pub player_spread: f32,
    pub min_turn_speed: [f32; 2],
    pub max_turn_speed: [f32; 2],
    pub pitch_convergence_time: f32,
    pub yaw_convergence_time: f32,
    pub suppress_time: f32,
    pub max_range: f32,
    pub anim_hor_rotate_inc: f32,
    pub player_position_dist: f32,
    pub use_hint_string: XStringRaw<'a>,
    pub drop_hint_string: XStringRaw<'a>,
    pub use_hint_string_index: i32,
    pub drop_hint_string_index: i32,
    pub horiz_view_jitter: f32,
    pub vert_view_jitter: f32,
    pub script: XStringRaw<'a>,
    pub min_damage: i32,
    pub min_player_damage: i32,
    pub max_damage_range: f32,
    pub min_damage_range: f32,
    pub destabilization_rate_time: f32,
    pub destabilization_curvature_max: f32,
    pub destabilize_distance: i32,
    pub location_damage_multipliers: Ptr32ArrayConst<'a, f32, 19>,
    pub fire_rumble: XStringRaw<'a>,
    pub melee_impact_rumble: XStringRaw<'a>,
    pub reload_rumble: XStringRaw<'a>,
    pub ads_dof_start: f32,
    pub ads_dof_end: f32,
    pub hip_dof_start: f32,
    pub hip_dof_end: f32,
    pub scan_speed: f32,
    pub scan_accel: f32,
    pub scan_pause_time: i32,
    pub flame_table_first_person: XStringRaw<'a>,
    pub flame_table_third_person: XStringRaw<'a>,
    pub flame_table_first_person_ptr: Ptr32<'a, FlameTableRaw<'a>>,
    pub flame_table_third_person_ptr: Ptr32<'a, FlameTableRaw<'a>>,
    pub tag_fx_preparation_effect: Ptr32<'a, fx::FxEffectDefRaw<'a>>,
    pub tag_flash_preparation_effect: Ptr32<'a, fx::FxEffectDefRaw<'a>>,
    pub do_gibbing: bool,
    #[allow(dead_code)]
    pad18: [u8; 3],
    pub max_gib_distance: f32,
}
assert_size!(WeaponDefRaw, 2056);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, FromPrimitive)]
pub enum WeapType {
    #[default]
    BULLET = 0,
    GRENADE = 1,
    PROJECTILE = 2,
    BINOCULARS = 3,
    GAS = 4,
    BOMB = 5,
    MINE = 6,
    MELEE = 7,
    NUM = 8,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, FromPrimitive)]
pub enum WeapClass {
    #[default]
    RIFLE = 0,
    MG = 1,
    SMG = 2,
    SPREAD = 3,
    PISTOL = 4,
    GRENADE = 5,
    ROCKETLAUNCHER = 6,
    TURRET = 7,
    NON_PLAYER = 8,
    GAS = 9,
    ITEM = 10,
    MELEE = 11,
    KILLSTREAK_ALT_STORED_WEAPON = 12,
    NUM = 13,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, FromPrimitive)]
pub enum PenetrateType {
    #[default]
    NONE = 0,
    SMALL = 1,
    MEDIUM = 2,
    LARGE = 3,
    COUNT = 4,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, FromPrimitive)]
pub enum ImpactType {
    #[default]
    NONE = 0,
    BULLET_SMALL = 1,
    BULLET_LARGE = 2,
    BULLET_AP = 3,
    BULLET_XTREME = 4,
    SHOTGUN = 5,
    GRENADE_BOUNCE = 6,
    GRENADE_EXPLODE = 7,
    RIFLE_GRENADE = 8,
    ROCKET_EXPLODE = 9,
    ROCKET_EXPLODE_XTREME = 10,
    PROJECTILE_DUD = 11,
    MORTAR_SHELL = 12,
    TANK_SHELL = 13,
    BOLT = 14,
    BLADE = 15,
    COUNT = 16,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, FromPrimitive)]
pub enum WeapInventoryType {
    #[default]
    PRIMARY = 0,
    OFFHAND = 1,
    ITEM = 2,
    ALTMODE = 3,
    MELEE = 4,
    COUNT = 5,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, FromPrimitive)]
pub enum WeapFireType {
    #[default]
    FULLAUTO = 0,
    SINGLESHOT = 1,
    BURSTFIRE2 = 2,
    BURSTFIRE3 = 3,
    BURSTFIRE4 = 4,
    STACKED = 5,
    MINIGUN = 6,
    COUNT = 7,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, FromPrimitive)]
pub enum WeapClipType {
    #[default]
    BOTTOM = 0,
    TOP = 1,
    LEFT = 2,
    DP28 = 3,
    PTRS = 4,
    LMG = 5,
    COUNT = 6,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, FromPrimitive)]
pub enum OffhandClass {
    #[default]
    NONE = 0,
    FRAG_GRENADE = 1,
    SMOKE_GRENADE = 2,
    FLASH_GRENADE = 3,
    GEAR = 4,
    COUNT = 5,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, FromPrimitive)]
pub enum OffhandSlot {
    #[default]
    NONE = 0,
    LETHAL_GRENADE = 1,
    TACTICAL_GRENADE = 2,
    EQUIPMENT = 3,
    SPECIFIC_USE = 4,
    COUNT = 5,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, FromPrimitive)]
pub enum WeapStance {
    #[default]
    STAND = 0,
    DUCK = 1,
    PRONE = 2,
    NUM = 3,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, FromPrimitive)]
pub enum ActiveReticleType {
    #[default]
    NONE = 0,
    PIP_ON_A_STICK = 1,
    BOUNCING_DIAMOND = 2,
    COUNT = 3,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, FromPrimitive)]
pub enum AmmoCounterClipType {
    #[default]
    NONE = 0,
    MAGAZINE = 1,
    SHORTMAGAZINE = 2,
    SHOTGUN = 3,
    ROCKET = 4,
    BELTFED = 5,
    ALTWEAPON = 6,
    COUNT = 7,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, FromPrimitive)]
pub enum WeapOverlayReticle {
    #[default]
    NONE = 0,
    CROSSHAIR = 1,
    NUM = 2,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, FromPrimitive)]
pub enum WeapOverlayInterface {
    #[default]
    NONE = 0,
    JAVELIN = 1,
    TURRETSCOPE = 2,
    COUNT = 3,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, FromPrimitive)]
pub enum WeapProjExplosion {
    #[default]
    GRENADE = 0,
    ROCKET = 1,
    FLASHBANG = 2,
    NONE = 3,
    DUD = 4,
    SMOKE = 5,
    HEAVY = 6,
    FIRE = 7,
    NAPALMBLOB = 8,
    BOLT = 9,
    NUM = 10,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, FromPrimitive)]
pub enum WeapStickinessType {
    #[default]
    NONE = 0,
    ALL = 1,
    ALL_NO_SENTIENTS = 2,
    GROUND = 3,
    GROUND_WITH_YAW = 4,
    FLESH = 5,
    COUNT = 6,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, FromPrimitive)]
pub enum WeapRotateType {
    #[default]
    GRENADE_ROTATE = 0,
    BLADE_ROTATE = 1,
    CYLINDER_ROTATE = 2,
    COUNT = 3,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, FromPrimitive)]
pub enum GuidedMissileType {
    #[default]
    NONE = 0,
    SIDEWINDER = 1,
    HELLFIRE = 2,
    JAVELIN = 3,
    BALLISTIC = 4,
    WIREGUIDED = 5,
    TVGUIDED = 6,
    COUNT = 7,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Default, Debug)]
pub struct WeaponDef {
    pub overlay_name: XString,
    pub gun_xmodel: Option<[Option<Box<xmodel::XModel>>; 16]>,
    pub hand_xmodel: Option<Box<xmodel::XModel>>,
    pub mode_name: XString,
    pub notetrack_sound_map_keys: Option<Box<[String; 20]>>,
    pub notetrack_sound_map_values: Option<Box<[String; 20]>>,
    pub player_anim_type: i32,
    pub weap_type: WeapType,
    pub weap_class: WeapClass,
    pub penetrate_type: PenetrateType,
    pub impact_type: ImpactType,
    pub inventory_type: WeapInventoryType,
    pub fire_type: WeapFireType,
    pub clip_type: WeapClipType,
    pub item_index: usize,
    pub parent_weapon_name: XString,
    pub jam_fire_time: i32,
    pub tracer_frequency: i32,
    pub tracer_width: f32,
    pub tracer_length: f32,
    pub overheat_weapon: i32,
    pub overheat_rate: f32,
    pub cooldown_rate: f32,
    pub overheat_end_val: f32,
    pub cool_while_firing: bool,
    pub fuel_tank_weapon: bool,
    pub tank_life_time: i32,
    pub offhand_class: OffhandClass,
    pub offhand_slot: OffhandSlot,
    pub stance: WeapStance,
    pub view_flash_effect: Option<Box<fx::FxEffectDef>>,
    pub world_flash_effect: Option<Box<fx::FxEffectDef>>,
    pub pickup_sound: XString,
    pub pickup_sound_player: XString,
    pub ammo_pickup_sound: XString,
    pub ammo_pickup_sound_player: XString,
    pub projectile_sound: XString,
    pub pullback_sound: XString,
    pub pullback_sound_player: XString,
    pub fire_sound: XString,
    pub fire_sound_player: XString,
    pub fire_loop_sound: XString,
    pub fire_loop_sound_player: XString,
    pub fire_loop_end_sound: XString,
    pub fire_loop_end_sound_player: XString,
    pub fire_stop_sound: XString,
    pub fire_stop_sound_player: XString,
    pub fire_last_sound: XString,
    pub fire_last_sound_player: XString,
    pub empty_fire_sound: XString,
    pub empty_fire_sound_player: XString,
    pub crack_sound: XString,
    pub whiz_by_sound: XString,
    pub melee_swipe_sound: XString,
    pub melee_swipe_sound_player: XString,
    pub melee_hit_sound: XString,
    pub melee_miss_sound: XString,
    pub rechamber_sound: XString,
    pub rechamber_sound_player: XString,
    pub reload_sound: XString,
    pub reload_sound_player: XString,
    pub reload_empty_sound: XString,
    pub reload_empty_sound_player: XString,
    pub reload_start_sound: XString,
    pub reload_start_sound_player: XString,
    pub reload_end_sound: XString,
    pub reload_end_sound_player: XString,
    pub rotate_loop_sound: XString,
    pub rotate_loop_sound_player: XString,
    pub deploy_sound: XString,
    pub deploy_sound_player: XString,
    pub finish_deploy_sound: XString,
    pub finish_deploy_sound_player: XString,
    pub breakdown_sound: XString,
    pub breakdown_sound_player: XString,
    pub finish_breakdown_sound: XString,
    pub finish_breakdown_sound_player: XString,
    pub detonate_sound: XString,
    pub detonate_sound_player: XString,
    pub night_vision_wear_sound: XString,
    pub night_vision_wear_sound_player: XString,
    pub night_vision_remove_sound: XString,
    pub night_vision_remove_sound_player: XString,
    pub alt_switch_sound: XString,
    pub alt_switch_sound_player: XString,
    pub raise_sound: XString,
    pub raise_sound_player: XString,
    pub first_raise_sound: XString,
    pub first_raise_sound_player: XString,
    pub put_away_sound: XString,
    pub put_away_sound_player: XString,
    pub overheat_sound: XString,
    pub overheat_sound_player: XString,
    pub ads_zoom_sound: XString,
    pub bounce_sound: Option<Box<[XString; 31]>>,
    pub stand_mounted_weapdef: XString,
    pub crouch_mounted_weapdef: XString,
    pub prone_mounted_weapdef: XString,
    pub stand_mounted_index: usize,
    pub crouch_mounted_index: usize,
    pub prone_mounted_index: usize,
    pub view_shell_eject_effect: Option<Box<fx::FxEffectDef>>,
    pub world_shell_eject_effect: Option<Box<fx::FxEffectDef>>,
    pub view_last_shot_eject_effect: Option<Box<fx::FxEffectDef>>,
    pub world_last_shot_eject_effect: Option<Box<fx::FxEffectDef>>,
    pub reticle_center: Option<Box<techset::Material>>,
    pub reticle_side: Option<Box<techset::Material>>,
    pub reticle_center_size: i32,
    pub reticle_side_size: i32,
    pub reticle_min_ofs: i32,
    pub active_reticle_type: ActiveReticleType,
    pub stand_move: Vec3,
    pub stand_rot: Vec3,
    pub ducked_ofs: Vec3,
    pub ducked_move: Vec3,
    pub ducked_sprint_ofs: Vec3,
    pub ducked_sprint_rot: Vec3,
    pub ducked_sprint_bob: Vec2,
    pub ducked_sprint_cycle_scale: f32,
    pub sprint_ofs: Vec3,
    pub sprint_rot: Vec3,
    pub sprint_bob: Vec2,
    pub sprint_cycle_scale: f32,
    pub low_ready_ofs: Vec3,
    pub low_ready_rot: Vec3,
    pub dtp_ofs: Vec3,
    pub dtp_rot: Vec3,
    pub dtp_bob: Vec2,
    pub dtp_cycle_scale: f32,
    pub mantle_ofs: Vec3,
    pub mantle_rot: Vec3,
    pub slide_ofs: Vec3,
    pub slide_rot: Vec3,
    pub ducked_rot: Vec3,
    pub prone_ofs: Vec3,
    pub prone_move: Vec3,
    pub prone_rot: Vec3,
    pub strafe_move: Vec3,
    pub strafe_rot: Vec3,
    pub pos_move_rate: f32,
    pub pos_prone_move_rate: f32,
    pub stand_move_min_speed: f32,
    pub ducked_move_min_speed: f32,
    pub prone_move_min_speed: f32,
    pub pos_rot_rate: f32,
    pub pos_prone_rot_rate: f32,
    pub stand_rot_min_speed: f32,
    pub ducked_rot_min_speed: f32,
    pub prone_rot_min_speed: f32,
    pub world_model: Option<Box<[Option<Box<xmodel::XModel>>; 16]>>,
    pub world_clip_model: Option<Box<xmodel::XModel>>,
    pub rocket_model: Option<Box<xmodel::XModel>>,
    pub mounted_model: Option<Box<xmodel::XModel>>,
    pub additional_melee_model: Option<Box<xmodel::XModel>>,
    pub hud_icon: Option<Box<techset::Material>>,
    pub hud_icon_ratio: WeaponIconRatioType,
    pub indicator_icon: Option<Box<techset::Material>>,
    pub indicator_icon_ratio: WeaponIconRatioType,
    pub ammo_counter_icon: Option<Box<techset::Material>>,
    pub ammo_counter_icon_ratio: WeaponIconRatioType,
    pub ammo_counter_clip: AmmoCounterClipType,
    pub start_ammo: i32,
    pub head_index: usize,
    pub max_ammo: i32,
    pub shot_count: i32,
    pub shared_ammo_cap_name: XString,
    pub shared_ammo_cap_index: usize,
    pub shared_ammo_cap: i32,
    pub unlimited_ammo: bool,
    pub ammo_count_clip_relative: bool,
    pub damage: i32,
    pub damage_duration: f32,
    pub damage_interval: f32,
    pub player_damage: i32,
    pub melee_damage: i32,
    pub damage_type: i32,
    pub explosion_tag: XString,
    pub fire_delay: i32,
    pub melee_delay: i32,
    pub melee_charge_delay: i32,
    pub detonate_delay: i32,
    pub spin_up_time: i32,
    pub spin_down_time: i32,
    pub spin_rate: f32,
    pub spin_loop_sound: XString,
    pub spin_loop_sound_player: XString,
    pub start_spin_sound: XString,
    pub start_spin_sound_player: XString,
    pub stop_spin_sound: XString,
    pub stop_spin_sound_player: XString,
    pub fire_time: i32,
    pub last_fire_time: i32,
    pub rechamber_time: i32,
    pub rechamber_bolt_time: i32,
    pub hold_fire_time: i32,
    pub detonate_fire_time: i32,
    pub melee_time: i32,
    pub melee_charge_time: i32,
    pub reload_time_right: i32,
    pub reload_time_left: i32,
    pub reload_show_rocket_time: i32,
    pub reload_empty_time_left: i32,
    pub reload_add_time: i32,
    pub reload_empty_add_time: i32,
    pub reload_quick_add_time: i32,
    pub reload_quick_empty_add_time: i32,
    pub reload_start_time: i32,
    pub reload_start_add_time: i32,
    pub reload_end_time: i32,
    pub drop_time: i32,
    pub raise_time: i32,
    pub alt_drop_time: i32,
    pub quick_drop_time: i32,
    pub quick_raise_time: i32,
    pub first_raise_time: i32,
    pub empty_raise_time: i32,
    pub empty_drop_time: i32,
    pub sprint_in_time: i32,
    pub sprint_loop_time: i32,
    pub sprint_out_time: i32,
    pub low_ready_in_time: i32,
    pub low_ready_loop_time: i32,
    pub low_ready_out_time: i32,
    pub cont_fire_in_time: i32,
    pub cont_fire_loop_time: i32,
    pub cont_fire_out_time: i32,
    pub dtp_in_time: i32,
    pub dtp_loop_time: i32,
    pub dtp_out_time: i32,
    pub slide_in_time: i32,
    pub deploy_time: i32,
    pub breakdown_time: i32,
    pub night_vision_wear_time: i32,
    pub night_vision_wear_time_fade_out_end: i32,
    pub night_vision_wear_time_power_up: i32,
    pub night_vision_remove_time: i32,
    pub night_vision_remove_time_power_down: i32,
    pub night_vision_remove_time_fade_in_start: i32,
    pub fuse_time: i32,
    pub ai_fuse_time: i32,
    pub lock_on_radius: i32,
    pub lock_on_speed: i32,
    pub require_lockon_to_fire: bool,
    pub no_ads_when_mag_empty: bool,
    pub avoid_drop_cleanup: bool,
    pub stack_fire: u32,
    pub stack_fire_spread: f32,
    pub stack_fire_accuracy_decay: f32,
    pub stack_sound: XString,
    pub auto_aim_range: f32,
    pub aim_assist_range: f32,
    pub mountable_weapon: bool,
    pub aim_padding: f32,
    pub enemy_crosshair_range: f32,
    pub crosshair_color_change: bool,
    pub move_speed_scale: f32,
    pub ads_move_speed_scale: f32,
    pub sprint_duration_scale: f32,
    pub overlay_reticle: WeapOverlayReticle,
    pub overlay_interface: WeapOverlayInterface,
    pub overlay_width: f32,
    pub overlay_height: f32,
    pub ads_bob_factor: f32,
    pub ads_view_bob_mult: f32,
    pub hip_spread_stand_min: f32,
    pub hip_spread_ducked_min: f32,
    pub hip_spread_prone_min: f32,
    pub hip_spread_stand_max: f32,
    pub hip_spread_ducked_max: f32,
    pub hip_spread_prone_max: f32,
    pub hip_spread_decay_rate: f32,
    pub hip_spread_fire_add: f32,
    pub hip_spread_turn_add: f32,
    pub hip_spread_move_add: f32,
    pub hip_spread_ducked_decay: f32,
    pub hip_spread_prone_decay: f32,
    pub hip_reticle_side_pos: f32,
    pub ads_idle_amount: f32,
    pub hip_idle_amount: f32,
    pub ads_idle_speed: f32,
    pub hip_idle_speed: f32,
    pub idle_crouch_factor: f32,
    pub idle_prone_factor: f32,
    pub gun_max_pitch: f32,
    pub gun_max_yaw: f32,
    pub sway_max_angle: f32,
    pub sway_lerp_speed: f32,
    pub sway_pitch_scale: f32,
    pub sway_yaw_scale: f32,
    pub sway_horiz_scale: f32,
    pub sway_vert_scale: f32,
    pub sway_shell_shock_scale: f32,
    pub ads_sway_max_angle: f32,
    pub ads_sway_lerp_speed: f32,
    pub ads_sway_pitch_scale: f32,
    pub ads_sway_yaw_scale: f32,
    pub shared_ammo: bool,
    pub rifle_bullet: bool,
    pub armor_piercing: bool,
    pub bolt_action: bool,
    pub use_alt_tag_flesh: bool,
    pub use_anti_lag_rewind: bool,
    pub is_carried_killstreak_weapon: bool,
    pub aim_down_sight: bool,
    pub rechamber_while_ads: bool,
    pub reload_while_ads: bool,
    pub ads_view_error_min: f32,
    pub ads_view_error_max: f32,
    pub cook_off_hold: bool,
    pub clip_only: bool,
    pub can_use_in_vehicle: bool,
    pub no_drops_or_raises: bool,
    pub ads_fire_only: bool,
    pub cancel_auto_holster_when_empty: bool,
    pub suppress_ammo_reserve_display: bool,
    pub laser_sight_during_nightvision: bool,
    pub hide_third_person: bool,
    pub has_bayonet: bool,
    pub dual_wield: bool,
    pub explode_on_ground: bool,
    pub throw_back: bool,
    pub retrievable: bool,
    pub die_on_respawn: bool,
    pub no_third_person_drops_or_raises: bool,
    pub continuous_fire: bool,
    pub no_ping: bool,
    pub force_bounce: bool,
    pub use_dropped_model_as_stowed: bool,
    pub no_quick_drop_when_empty: bool,
    pub keep_crosshair_when_ads: bool,
    pub use_only_alt_weaopon_hide_tags_in_alt_mode: bool,
    pub kill_icon: Option<Box<techset::Material>>,
    pub kill_icon_ratio: WeaponIconRatioType,
    pub flip_kill_icon: bool,
    pub no_partial_reload: bool,
    pub segmented_reload: bool,
    pub no_ads_auto_reload: bool,
    pub reload_ammo_add: i32,
    pub reload_start_add: i32,
    pub spawned_grenade_weapon_name: XString,
    pub dual_wield_weapon_name: XString,
    pub dual_wield_weapon_index: usize,
    pub drop_ammo_min: i32,
    pub drop_ammo_max: i32,
    pub drop_clip_ammo_min: i32,
    pub drop_clip_ammo_max: i32,
    pub blocks_prone: bool,
    pub show_indicator: bool,
    pub is_rolling_grenade: i32,
    pub explosion_radius: i32,
    pub explosion_radius_min: i32,
    pub indicator_radius: i32,
    pub explosion_inner_damage: i32,
    pub explosion_outer_damage: i32,
    pub damage_cone_angle: f32,
    pub projectile_speed: i32,
    pub projectile_speed_up: i32,
    pub projectile_speed_relative_up: i32,
    pub projectile_speed_forward: i32,
    pub projectile_active_dist: i32,
    pub proj_lifetime: f32,
    pub time_to_accelerate: f32,
    pub projectile_curvature: f32,
    pub projectile_model: Option<Box<xmodel::XModel>>,
    pub proj_explosion: WeapProjExplosion,
    pub proj_explosion_effect: Option<Box<fx::FxEffectDef>>,
    pub proj_explosion_effect_force_normal_up: bool,
    pub proj_explosion_effect_2: Option<Box<fx::FxEffectDef>>,
    pub proj_explosion_effect_2_force_normal_up: bool,
    pub proj_explosion_effect_3: Option<Box<fx::FxEffectDef>>,
    pub proj_explosion_effect_3_force_normal_up: bool,
    pub proj_explosion_effect_4: Option<Box<fx::FxEffectDef>>,
    pub proj_explosion_effect_4_force_normal_up: bool,
    pub proj_explosion_effect_5: Option<Box<fx::FxEffectDef>>,
    pub proj_explosion_effect_5_force_normal_up: bool,
    pub proj_dud_effect: Option<Box<fx::FxEffectDef>>,
    pub proj_explosion_sound: XString,
    pub proj_dud_sound: XString,
    pub mortar_shell_sound: XString,
    pub tank_shell_sound: XString,
    pub proj_impact_explode: bool,
    pub bullet_impact_explode: bool,
    pub stickiness: WeapStickinessType,
    pub rotate_type: WeapRotateType,
    pub plantable: bool,
    pub has_detonator: bool,
    pub time_detonation: bool,
    pub no_crumple_missile: bool,
    pub rotate: bool,
    pub keep_rolling: bool,
    pub hold_button_to_throw: bool,
    pub offhand_hold_is_cancelable: bool,
    pub freeze_movement_when_firing: bool,
    pub low_ammo_warning_threshold: f32,
    pub melee_charge_range: f32,
    pub use_as_melee: bool,
    pub is_camera_sensor: bool,
    pub is_acoustic_sensor: bool,
    pub parallel_bounce: Option<Box<[f32; 31]>>,
    pub perpendicular_bounce: Option<Box<[f32; 31]>>,
    pub proj_tail_effect: Option<Box<fx::FxEffectDef>>,
    pub projectile_color: Vec3,
    pub guided_missile_type: GuidedMissileType,
    pub max_steering_accel: f32,
    pub proj_ignition_delay: i32,
    pub proj_ignition_effect: Option<Box<fx::FxEffectDef>>,
    pub proj_ignition_sound: XString,
    pub ads_aim_pitch: f32,
    pub ads_crosshair_in_frac: f32,
    pub ads_crosshair_out_frac: f32,
    pub ads_gun_kick_reduced_kick_bullets: i32,
    pub ads_gun_kick_reduced_kick_percent: f32,
    pub ads_gun_kick_pitch_min: f32,
    pub ads_gun_kick_pitch_max: f32,
    pub ads_gun_kick_yaw_min: f32,
    pub ads_gun_kick_yaw_max: f32,
    pub ads_gun_kick_accel: f32,
    pub ads_gun_kick_speed_max: f32,
    pub ads_gun_kick_speed_decay: f32,
    pub ads_gun_kick_static_decay: f32,
    pub ads_view_kick_pitch_min: f32,
    pub ads_view_kick_pitch_max: f32,
    pub ads_view_kick_yaw_min: f32,
    pub ads_view_kick_yaw_max: f32,
    pub ads_view_scatter_min: f32,
    pub ads_view_scatter_max: f32,
    pub ads_spread: f32,
    pub hip_gun_kick_reduced_kick_bullets: i32,
    pub hip_gun_kick_reduced_kick_percent: f32,
    pub hip_gun_kick_pitch_min: f32,
    pub hip_gun_kick_pitch_max: f32,
    pub hip_gun_kick_yaw_min: f32,
    pub hip_gun_kick_yaw_max: f32,
    pub hip_gun_kick_accel: f32,
    pub hip_gun_kick_speed_max: f32,
    pub hip_gun_kick_speed_decay: f32,
    pub hip_gun_kick_static_decay: f32,
    pub hip_view_kick_pitch_min: f32,
    pub hip_view_kick_pitch_max: f32,
    pub hip_view_kick_yaw_min: f32,
    pub hip_view_kick_yaw_max: f32,
    pub hip_view_scatter_min: f32,
    pub hip_view_scatter_max: f32,
    pub fight_dist: f32,
    pub max_dist: f32,
    pub accuracy_graph_name: [XString; 2],
    pub accuracy_graph_knots: [Vec<Vec2>; 2],
    pub original_accuracy_graph_knots: [Vec<Vec2>; 2],
    pub accuracy_graph_knot_count: [i32; 2],
    pub original_accuracy_graph_knot_count: [i32; 2],
    pub position_reload_trans_time: i32,
    pub left_arc: f32,
    pub right_arc: f32,
    pub top_arc: f32,
    pub bottom_arc: f32,
    pub accuracy: f32,
    pub ai_spread: f32,
    pub player_spread: f32,
    pub min_turn_speed: Vec2,
    pub max_turn_speed: Vec2,
    pub pitch_convergence_time: f32,
    pub yaw_convergence_time: f32,
    pub suppress_time: f32,
    pub max_range: f32,
    pub anim_hor_rotate_inc: f32,
    pub player_position_dist: f32,
    pub use_hint_string: XString,
    pub drop_hint_string: XString,
    pub use_hint_string_index: usize,
    pub drop_hint_string_index: usize,
    pub horiz_view_jitter: f32,
    pub vert_view_jitter: f32,
    pub script: XString,
    pub min_damage: i32,
    pub min_player_damage: i32,
    pub max_damage_range: f32,
    pub min_damage_range: f32,
    pub destabilization_rate_time: f32,
    pub destabilization_curvature_max: f32,
    pub destabilize_distance: i32,
    pub location_damage_multipliers: Option<Box<[f32; 19]>>,
    pub fire_rumble: XString,
    pub melee_impact_rumble: XString,
    pub reload_rumble: XString,
    pub ads_dof_start: f32,
    pub ads_dof_end: f32,
    pub hip_dof_start: f32,
    pub hip_dof_end: f32,
    pub scan_speed: f32,
    pub scan_accel: f32,
    pub scan_pause_time: i32,
    pub flame_table_first_person: XString,
    pub flame_table_third_person: XString,
    pub flame_table_first_person_ptr: Option<Box<FlameTable>>,
    pub flame_table_third_person_ptr: Option<Box<FlameTable>>,
    pub tag_fx_preparation_effect: Option<Box<fx::FxEffectDef>>,
    pub tag_flash_preparation_effect: Option<Box<fx::FxEffectDef>>,
    pub do_gibbing: bool,
    pub max_gib_distance: f32,
}

impl<'a> XFileDeserializeInto<WeaponDef, ()> for WeaponDefRaw<'a> {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<WeaponDef> {
        let overlay_name = self.overlay_name.xfile_deserialize_into(de, ())?;
        let gun_xmodel = if self.gun_xmodel.is_null() {
            None
        } else {
            Some(
                self.gun_xmodel
                    .xfile_deserialize_into(de, ())?
                    .try_into()
                    .unwrap_or_default(),
            )
        };
        let hand_xmodel = self.hand_xmodel.xfile_deserialize_into(de, ())?;
        let mode_name = self.mode_name.xfile_deserialize_into(de, ())?;
        let notetrack_sound_map_keys = if self.notetrack_sound_map_keys.is_null() {
            None
        } else {
            Some(Box::new(
                self.notetrack_sound_map_keys
                    .to_vec(de)?
                    .into_iter()
                    .map(|k| k.to_string(de).unwrap_or_default())
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap_or_default(),
            ))
        };
        let notetrack_sound_map_values = if self.notetrack_sound_map_values.is_null() {
            None
        } else {
            Some(Box::new(
                self.notetrack_sound_map_values
                    .to_vec(de)?
                    .into_iter()
                    .map(|v| v.to_string(de).unwrap_or_default())
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap_or_default(),
            ))
        };
        let weap_type = FromPrimitive::from_u32(self.weap_type).ok_or(Error::new_with_offset(
            file_line_col!(),
            de.stream_pos()? as _,
            ErrorKind::BadFromPrimitive(self.weap_type as _),
        ))?;
        let weap_class = FromPrimitive::from_u32(self.weap_class).ok_or(Error::new_with_offset(
            file_line_col!(),
            de.stream_pos()? as _,
            ErrorKind::BadFromPrimitive(self.weap_class as _),
        ))?;
        let penetrate_type =
            FromPrimitive::from_u32(self.penetrate_type).ok_or(Error::new_with_offset(
                file_line_col!(),
                de.stream_pos()? as _,
                ErrorKind::BadFromPrimitive(self.penetrate_type as _),
            ))?;
        let impact_type =
            FromPrimitive::from_u32(self.impact_type).ok_or(Error::new_with_offset(
                file_line_col!(),
                de.stream_pos()? as _,
                ErrorKind::BadFromPrimitive(self.impact_type as _),
            ))?;
        let inventory_type =
            FromPrimitive::from_u32(self.inventory_type).ok_or(Error::new_with_offset(
                file_line_col!(),
                de.stream_pos()? as _,
                ErrorKind::BadFromPrimitive(self.inventory_type as _),
            ))?;
        let fire_type = FromPrimitive::from_u32(self.fire_type).ok_or(Error::new_with_offset(
            file_line_col!(),
            de.stream_pos()? as _,
            ErrorKind::BadFromPrimitive(self.fire_type as _),
        ))?;
        let clip_type = FromPrimitive::from_u32(self.clip_type).ok_or(Error::new_with_offset(
            file_line_col!(),
            de.stream_pos()? as _,
            ErrorKind::BadFromPrimitive(self.clip_type as _),
        ))?;
        let parent_weapon_name = self.parent_weapon_name.xfile_deserialize_into(de, ())?;
        let offhand_class =
            FromPrimitive::from_u32(self.offhand_class).ok_or(Error::new_with_offset(
                file_line_col!(),
                de.stream_pos()? as _,
                ErrorKind::BadFromPrimitive(self.offhand_class as _),
            ))?;
        let offhand_slot =
            FromPrimitive::from_u32(self.offhand_slot).ok_or(Error::new_with_offset(
                file_line_col!(),
                de.stream_pos()? as _,
                ErrorKind::BadFromPrimitive(self.offhand_slot as _),
            ))?;
        let stance = FromPrimitive::from_u32(self.stance).ok_or(Error::new_with_offset(
            file_line_col!(),
            de.stream_pos()? as _,
            ErrorKind::BadFromPrimitive(self.stance as _),
        ))?;
        let view_flash_effect = self.view_flash_effect.xfile_deserialize_into(de, ())?;
        let world_flash_effect = self.world_flash_effect.xfile_deserialize_into(de, ())?;
        let pickup_sound = self.pickup_sound.xfile_deserialize_into(de, ())?;
        let pickup_sound_player = self.pickup_sound_player.xfile_deserialize_into(de, ())?;
        let ammo_pickup_sound = self.ammo_pickup_sound.xfile_deserialize_into(de, ())?;
        let ammo_pickup_sound_player = self
            .ammo_pickup_sound_player
            .xfile_deserialize_into(de, ())?;
        let projectile_sound = self.projectile_sound.xfile_deserialize_into(de, ())?;
        let pullback_sound = self.pullback_sound.xfile_deserialize_into(de, ())?;
        let pullback_sound_player = self.pullback_sound_player.xfile_deserialize_into(de, ())?;
        let fire_sound = self.fire_sound.xfile_deserialize_into(de, ())?;
        let fire_sound_player = self.fire_sound_player.xfile_deserialize_into(de, ())?;
        let fire_loop_sound = self.fire_loop_sound.xfile_deserialize_into(de, ())?;
        let fire_loop_sound_player = self.fire_loop_sound_player.xfile_deserialize_into(de, ())?;
        let fire_loop_end_sound = self.fire_loop_end_sound.xfile_deserialize_into(de, ())?;
        let fire_loop_end_sound_player = self
            .fire_loop_end_sound_player
            .xfile_deserialize_into(de, ())?;
        let fire_stop_sound = self.fire_stop_sound.xfile_deserialize_into(de, ())?;
        let fire_stop_sound_player = self.fire_stop_sound_player.xfile_deserialize_into(de, ())?;
        let fire_last_sound = self.fire_last_sound.xfile_deserialize_into(de, ())?;
        let fire_last_sound_player = self.fire_last_sound_player.xfile_deserialize_into(de, ())?;
        let empty_fire_sound = self.empty_fire_sound.xfile_deserialize_into(de, ())?;
        let empty_fire_sound_player = self
            .empty_fire_sound_player
            .xfile_deserialize_into(de, ())?;
        let crack_sound = self.crack_sound.xfile_deserialize_into(de, ())?;
        let whiz_by_sound = self.whiz_by_sound.xfile_deserialize_into(de, ())?;
        let melee_swipe_sound = self.melee_swipe_sound.xfile_deserialize_into(de, ())?;
        let melee_swipe_sound_player = self
            .melee_swipe_sound_player
            .xfile_deserialize_into(de, ())?;
        let melee_hit_sound = self.melee_hit_sound.xfile_deserialize_into(de, ())?;
        let melee_miss_sound = self.melee_miss_sound.xfile_deserialize_into(de, ())?;
        let rechamber_sound = self.rechamber_sound.xfile_deserialize_into(de, ())?;
        let rechamber_sound_player = self.rechamber_sound_player.xfile_deserialize_into(de, ())?;
        let reload_sound = self.reload_sound.xfile_deserialize_into(de, ())?;
        let reload_sound_player = self.reload_sound_player.xfile_deserialize_into(de, ())?;
        let reload_empty_sound = self.reload_empty_sound.xfile_deserialize_into(de, ())?;
        let reload_empty_sound_player = self
            .reload_empty_sound_player
            .xfile_deserialize_into(de, ())?;
        let reload_start_sound = self.reload_start_sound.xfile_deserialize_into(de, ())?;
        let reload_start_sound_player = self
            .reload_start_sound_player
            .xfile_deserialize_into(de, ())?;
        let reload_end_sound = self.reload_end_sound.xfile_deserialize_into(de, ())?;
        let reload_end_sound_player = self
            .reload_end_sound_player
            .xfile_deserialize_into(de, ())?;
        let rotate_loop_sound = self.rotate_loop_sound.xfile_deserialize_into(de, ())?;
        let rotate_loop_sound_player = self
            .rotate_loop_sound_player
            .xfile_deserialize_into(de, ())?;
        let deploy_sound = self.deploy_sound.xfile_deserialize_into(de, ())?;
        let deploy_sound_player = self.deploy_sound_player.xfile_deserialize_into(de, ())?;
        let finish_deploy_sound = self.finish_deploy_sound.xfile_deserialize_into(de, ())?;
        let finish_deploy_sound_player = self
            .finish_deploy_sound_player
            .xfile_deserialize_into(de, ())?;
        let breakdown_sound = self.breakdown_sound.xfile_deserialize_into(de, ())?;
        let breakdown_sound_player = self.breakdown_sound_player.xfile_deserialize_into(de, ())?;
        let finish_breakdown_sound = self.finish_breakdown_sound.xfile_deserialize_into(de, ())?;
        let finish_breakdown_sound_player = self
            .finish_breakdown_sound_player
            .xfile_deserialize_into(de, ())?;
        let detonate_sound = self.detonate_sound.xfile_deserialize_into(de, ())?;
        let detonate_sound_player = self.detonate_sound_player.xfile_deserialize_into(de, ())?;
        let night_vision_wear_sound = self
            .night_vision_wear_sound
            .xfile_deserialize_into(de, ())?;
        let night_vision_wear_sound_player = self
            .night_vision_wear_sound_player
            .xfile_deserialize_into(de, ())?;
        let night_vision_remove_sound = self
            .night_vision_remove_sound
            .xfile_deserialize_into(de, ())?;
        let night_vision_remove_sound_player = self
            .night_vision_remove_sound_player
            .xfile_deserialize_into(de, ())?;
        let alt_switch_sound = self.alt_switch_sound.xfile_deserialize_into(de, ())?;
        let alt_switch_sound_player = self
            .alt_switch_sound_player
            .xfile_deserialize_into(de, ())?;
        let raise_sound = self.raise_sound.xfile_deserialize_into(de, ())?;
        let raise_sound_player = self.raise_sound_player.xfile_deserialize_into(de, ())?;
        let first_raise_sound = self.first_raise_sound.xfile_deserialize_into(de, ())?;
        let first_raise_sound_player = self
            .first_raise_sound_player
            .xfile_deserialize_into(de, ())?;
        let put_away_sound = self.put_away_sound.xfile_deserialize_into(de, ())?;
        let put_away_sound_player = self.put_away_sound_player.xfile_deserialize_into(de, ())?;
        let overheat_sound = self.overheat_sound.xfile_deserialize_into(de, ())?;
        let overheat_sound_player = self.overheat_sound_player.xfile_deserialize_into(de, ())?;
        let ads_zoom_sound = self.ads_zoom_sound.xfile_deserialize_into(de, ())?;
        let bounce_sound = if self.bounce_sound.is_null() {
            None
        } else {
            Some(Box::new(
                self.bounce_sound
                    .xfile_deserialize_into(de, ())?
                    .try_into()
                    .unwrap(),
            ))
        };
        let stand_mounted_weapdef = self.stand_mounted_weapdef.xfile_deserialize_into(de, ())?;
        let crouch_mounted_weapdef = self.crouch_mounted_weapdef.xfile_deserialize_into(de, ())?;
        let prone_mounted_weapdef = self.prone_mounted_weapdef.xfile_deserialize_into(de, ())?;
        let view_shell_eject_effect = self
            .view_shell_eject_effect
            .xfile_deserialize_into(de, ())?;
        let world_shell_eject_effect = self
            .world_shell_eject_effect
            .xfile_deserialize_into(de, ())?;
        let view_last_shot_eject_effect = self
            .view_last_shot_eject_effect
            .xfile_deserialize_into(de, ())?;
        let world_last_shot_eject_effect = self
            .world_last_shot_eject_effect
            .xfile_deserialize_into(de, ())?;
        let reticle_center = self.reticle_center.xfile_deserialize_into(de, ())?;
        let reticle_side = self.reticle_side.xfile_deserialize_into(de, ())?;
        let active_reticle_type =
            FromPrimitive::from_u32(self.active_reticle_type).ok_or(Error::new_with_offset(
                file_line_col!(),
                de.stream_pos()? as _,
                ErrorKind::BadFromPrimitive(self.active_reticle_type as _),
            ))?;
        let world_model = if self.world_model.is_null() {
            None
        } else {
            Some(Box::new(
                self.world_model
                    .xfile_deserialize_into(de, ())?
                    .try_into()
                    .unwrap_or_default(),
            ))
        };
        let world_clip_model = self.world_clip_model.xfile_deserialize_into(de, ())?;
        let rocket_model = self.rocket_model.xfile_deserialize_into(de, ())?;
        let mounted_model = self.mounted_model.xfile_deserialize_into(de, ())?;
        let additional_melee_model = self.additional_melee_model.xfile_deserialize_into(de, ())?;
        let hud_icon = self.hud_icon.xfile_deserialize_into(de, ())?;
        let hud_icon_ratio =
            FromPrimitive::from_u32(self.hud_icon_ratio).ok_or(Error::new_with_offset(
                file_line_col!(),
                de.stream_pos()? as _,
                ErrorKind::BadFromPrimitive(self.hud_icon_ratio as _),
            ))?;
        let indicator_icon = self.indicator_icon.xfile_deserialize_into(de, ())?;
        let indicator_icon_ratio =
            FromPrimitive::from_u32(self.indicator_icon_ratio).ok_or(Error::new_with_offset(
                file_line_col!(),
                de.stream_pos()? as _,
                ErrorKind::BadFromPrimitive(self.indicator_icon_ratio as _),
            ))?;
        let ammo_counter_icon = self.ammo_counter_icon.xfile_deserialize_into(de, ())?;
        let ammo_counter_icon_ratio =
            FromPrimitive::from_u32(self.ammo_counter_icon_ratio).ok_or(Error::new_with_offset(
                file_line_col!(),
                de.stream_pos()? as _,
                ErrorKind::BadFromPrimitive(self.ammo_counter_icon_ratio as _),
            ))?;
        let ammo_counter_clip =
            FromPrimitive::from_u32(self.ammo_counter_clip).ok_or(Error::new_with_offset(
                file_line_col!(),
                de.stream_pos()? as _,
                ErrorKind::BadFromPrimitive(self.ammo_counter_clip as _),
            ))?;
        let shared_ammo_cap_name = self.shared_ammo_cap_name.xfile_deserialize_into(de, ())?;
        let explosion_tag = XString(self.explosion_tag.to_string(de).unwrap_or_default());
        let spin_loop_sound = self.spin_loop_sound.xfile_deserialize_into(de, ())?;
        let spin_loop_sound_player = self.spin_loop_sound_player.xfile_deserialize_into(de, ())?;
        let start_spin_sound = self.start_spin_sound.xfile_deserialize_into(de, ())?;
        let start_spin_sound_player = self
            .start_spin_sound_player
            .xfile_deserialize_into(de, ())?;
        let stop_spin_sound = self.stop_spin_sound.xfile_deserialize_into(de, ())?;
        let stop_spin_sound_player = self.stop_spin_sound_player.xfile_deserialize_into(de, ())?;
        let stack_sound = self.stack_sound.xfile_deserialize_into(de, ())?;
        let overlay_reticle =
            FromPrimitive::from_u32(self.overlay_reticle).ok_or(Error::new_with_offset(
                file_line_col!(),
                de.stream_pos()? as _,
                ErrorKind::BadFromPrimitive(self.overlay_reticle as _),
            ))?;
        let overlay_interface =
            FromPrimitive::from_u32(self.overlay_interface).ok_or(Error::new_with_offset(
                file_line_col!(),
                de.stream_pos()? as _,
                ErrorKind::BadFromPrimitive(self.overlay_interface as _),
            ))?;
        let kill_icon = self.kill_icon.xfile_deserialize_into(de, ())?;
        let kill_icon_ratio =
            FromPrimitive::from_u32(self.kill_icon_ratio).ok_or(Error::new_with_offset(
                file_line_col!(),
                de.stream_pos()? as _,
                ErrorKind::BadFromPrimitive(self.kill_icon_ratio as _),
            ))?;
        let spawned_grenade_weapon_name = self
            .spawned_grenade_weapon_name
            .xfile_deserialize_into(de, ())?;
        let dual_wield_weapon_name = self.dual_wield_weapon_name.xfile_deserialize_into(de, ())?;
        let projectile_model = self.projectile_model.xfile_deserialize_into(de, ())?;
        let proj_explosion =
            FromPrimitive::from_u32(self.proj_explosion).ok_or(Error::new_with_offset(
                file_line_col!(),
                de.stream_pos()? as _,
                ErrorKind::BadFromPrimitive(self.proj_explosion as _),
            ))?;
        let proj_explosion_effect = self.proj_explosion_effect.xfile_deserialize_into(de, ())?;
        let proj_explosion_effect_2 = self
            .proj_explosion_effect_2
            .xfile_deserialize_into(de, ())?;
        let proj_explosion_effect_3 = self
            .proj_explosion_effect_3
            .xfile_deserialize_into(de, ())?;
        let proj_explosion_effect_4 = self
            .proj_explosion_effect_4
            .xfile_deserialize_into(de, ())?;
        let proj_explosion_effect_5 = self
            .proj_explosion_effect_5
            .xfile_deserialize_into(de, ())?;
        let proj_dud_effect = self.proj_dud_effect.xfile_deserialize_into(de, ())?;
        let proj_explosion_sound = self.proj_explosion_sound.xfile_deserialize_into(de, ())?;
        let proj_dud_sound = self.proj_dud_sound.xfile_deserialize_into(de, ())?;
        let mortar_shell_sound = self.mortar_shell_sound.xfile_deserialize_into(de, ())?;
        let tank_shell_sound = self.tank_shell_sound.xfile_deserialize_into(de, ())?;
        let stickiness = FromPrimitive::from_u32(self.stickiness).ok_or(Error::new_with_offset(
            file_line_col!(),
            de.stream_pos()? as _,
            ErrorKind::BadFromPrimitive(self.stickiness as _),
        ))?;
        let rotate_type =
            FromPrimitive::from_u32(self.rotate_type).ok_or(Error::new_with_offset(
                file_line_col!(),
                de.stream_pos()? as _,
                ErrorKind::BadFromPrimitive(self.rotate_type as _),
            ))?;
        let parallel_bounce = if self.parallel_bounce.is_null() {
            None
        } else {
            Some(Box::new(
                self.parallel_bounce
                    .to_vec(de)?
                    .try_into()
                    .unwrap_or_default(),
            ))
        };
        let perpendicular_bounce = if self.perpendicular_bounce.is_null() {
            None
        } else {
            Some(Box::new(
                self.perpendicular_bounce
                    .to_vec(de)?
                    .try_into()
                    .unwrap_or_default(),
            ))
        };
        let proj_tail_effect = self.proj_tail_effect.xfile_deserialize_into(de, ())?;
        let guided_missile_type =
            FromPrimitive::from_u32(self.guided_missile_type).ok_or(Error::new_with_offset(
                file_line_col!(),
                de.stream_pos()? as _,
                ErrorKind::BadFromPrimitive(self.guided_missile_type as _),
            ))?;
        let proj_ignition_effect = self.proj_ignition_effect.xfile_deserialize_into(de, ())?;
        let proj_ignition_sound = self.proj_ignition_sound.xfile_deserialize_into(de, ())?;

        let mut accuracy_graph_name = [const { XString::new() }; 2];
        let mut accuracy_graph_knots = [const { Vec::new() }; 2];
        let mut original_accuracy_graph_knots = [const { Vec::new() }; 2];
        for i in 0..=1 {
            accuracy_graph_name[i] = self.accuracy_graph_name[i].xfile_deserialize_into(de, ())?;
            accuracy_graph_knots[i] = self.accuracy_graph_knots[i]
                .to_array(self.accuracy_graph_knot_count[i] as _)
                .to_vec_into(de)?;
            original_accuracy_graph_knots[i] = self.original_accuracy_graph_knots[i]
                .to_array(self.original_accuracy_graph_knot_count[i] as _)
                .to_vec_into(de)?;
        }

        let use_hint_string = self.use_hint_string.xfile_deserialize_into(de, ())?;
        let drop_hint_string = self.drop_hint_string.xfile_deserialize_into(de, ())?;
        let script = self.script.xfile_deserialize_into(de, ())?;
        let location_damage_multipliers = if self.location_damage_multipliers.is_null() {
            None
        } else {
            Some(Box::new(
                self.location_damage_multipliers
                    .to_vec(de)?
                    .try_into()
                    .unwrap_or_default(),
            ))
        };
        let fire_rumble = self.fire_rumble.xfile_deserialize_into(de, ())?;
        let melee_impact_rumble = self.melee_impact_rumble.xfile_deserialize_into(de, ())?;
        let reload_rumble = self.reload_rumble.xfile_deserialize_into(de, ())?;
        let flame_table_first_person = self
            .flame_table_first_person
            .xfile_deserialize_into(de, ())?;
        let flame_table_third_person = self
            .flame_table_third_person
            .xfile_deserialize_into(de, ())?;
        let flame_table_first_person_ptr = self
            .flame_table_first_person_ptr
            .xfile_deserialize_into(de, ())?;
        let flame_table_third_person_ptr = self
            .flame_table_third_person_ptr
            .xfile_deserialize_into(de, ())?;
        let tag_fx_preparation_effect = self
            .tag_fx_preparation_effect
            .xfile_deserialize_into(de, ())?;
        let tag_flash_preparation_effect = self
            .tag_flash_preparation_effect
            .xfile_deserialize_into(de, ())?;

        Ok(WeaponDef {
            overlay_name,
            gun_xmodel,
            hand_xmodel,
            mode_name,
            notetrack_sound_map_keys,
            notetrack_sound_map_values,
            player_anim_type: self.player_anim_type,
            weap_type,
            weap_class,
            penetrate_type,
            impact_type,
            inventory_type,
            fire_type,
            clip_type,
            item_index: self.item_index as _,
            parent_weapon_name,
            jam_fire_time: self.jam_fire_time,
            tracer_frequency: self.tracer_frequency,
            tracer_width: self.tracer_width,
            tracer_length: self.tracer_length,
            overheat_weapon: self.overheat_weapon,
            overheat_rate: self.overheat_rate,
            cooldown_rate: self.cooldown_rate,
            overheat_end_val: self.overheat_end_val,
            cool_while_firing: self.cool_while_firing,
            fuel_tank_weapon: self.fuel_tank_weapon,
            tank_life_time: self.tank_life_time,
            offhand_class,
            offhand_slot,
            stance,
            view_flash_effect,
            world_flash_effect,
            pickup_sound,
            pickup_sound_player,
            ammo_pickup_sound,
            ammo_pickup_sound_player,
            projectile_sound,
            pullback_sound,
            pullback_sound_player,
            fire_sound,
            fire_sound_player,
            fire_loop_sound,
            fire_loop_sound_player,
            fire_loop_end_sound,
            fire_loop_end_sound_player,
            fire_stop_sound,
            fire_stop_sound_player,
            fire_last_sound,
            fire_last_sound_player,
            empty_fire_sound,
            empty_fire_sound_player,
            crack_sound,
            whiz_by_sound,
            melee_swipe_sound,
            melee_swipe_sound_player,
            melee_hit_sound,
            melee_miss_sound,
            rechamber_sound,
            rechamber_sound_player,
            reload_sound,
            reload_sound_player,
            reload_empty_sound,
            reload_empty_sound_player,
            reload_start_sound,
            reload_start_sound_player,
            reload_end_sound,
            reload_end_sound_player,
            rotate_loop_sound,
            rotate_loop_sound_player,
            deploy_sound,
            deploy_sound_player,
            finish_deploy_sound,
            finish_deploy_sound_player,
            breakdown_sound,
            breakdown_sound_player,
            finish_breakdown_sound,
            finish_breakdown_sound_player,
            detonate_sound,
            detonate_sound_player,
            night_vision_wear_sound,
            night_vision_wear_sound_player,
            night_vision_remove_sound,
            night_vision_remove_sound_player,
            alt_switch_sound,
            alt_switch_sound_player,
            raise_sound,
            raise_sound_player,
            first_raise_sound,
            first_raise_sound_player,
            put_away_sound,
            put_away_sound_player,
            overheat_sound,
            overheat_sound_player,
            ads_zoom_sound,
            bounce_sound,
            stand_mounted_weapdef,
            crouch_mounted_weapdef,
            prone_mounted_weapdef,
            stand_mounted_index: self.stand_mounted_index as _,
            crouch_mounted_index: self.crouch_mounted_index as _,
            prone_mounted_index: self.prone_mounted_index as _,
            view_shell_eject_effect,
            world_shell_eject_effect,
            view_last_shot_eject_effect,
            world_last_shot_eject_effect,
            reticle_center,
            reticle_side,
            reticle_center_size: self.reticle_center_size,
            reticle_side_size: self.reticle_side_size,
            reticle_min_ofs: self.reticle_min_ofs,
            active_reticle_type,
            stand_move: self.stand_move.into(),
            stand_rot: self.stand_rot.into(),
            ducked_ofs: self.ducked_ofs.into(),
            ducked_move: self.ducked_move.into(),
            ducked_sprint_ofs: self.ducked_sprint_ofs.into(),
            ducked_sprint_rot: self.ducked_sprint_rot.into(),
            ducked_sprint_bob: self.ducked_sprint_bob.into(),
            ducked_sprint_cycle_scale: self.ducked_sprint_cycle_scale,
            sprint_ofs: self.sprint_ofs.into(),
            sprint_rot: self.sprint_rot.into(),
            sprint_bob: self.sprint_bob.into(),
            sprint_cycle_scale: self.sprint_cycle_scale,
            low_ready_ofs: self.low_ready_ofs.into(),
            low_ready_rot: self.low_ready_rot.into(),
            dtp_ofs: self.dtp_ofs.into(),
            dtp_rot: self.dtp_rot.into(),
            dtp_bob: self.dtp_bob.into(),
            dtp_cycle_scale: self.dtp_cycle_scale,
            mantle_ofs: self.mantle_ofs.into(),
            mantle_rot: self.mantle_rot.into(),
            slide_ofs: self.slide_ofs.into(),
            slide_rot: self.slide_rot.into(),
            ducked_rot: self.ducked_rot.into(),
            prone_ofs: self.prone_ofs.into(),
            prone_move: self.prone_move.into(),
            prone_rot: self.prone_rot.into(),
            strafe_move: self.strafe_move.into(),
            strafe_rot: self.strafe_rot.into(),
            pos_move_rate: self.pos_move_rate,
            pos_prone_move_rate: self.pos_prone_move_rate,
            stand_move_min_speed: self.stand_move_min_speed,
            ducked_move_min_speed: self.ducked_move_min_speed,
            prone_move_min_speed: self.prone_move_min_speed,
            pos_rot_rate: self.pos_rot_rate,
            pos_prone_rot_rate: self.pos_prone_rot_rate,
            stand_rot_min_speed: self.stand_rot_min_speed,
            ducked_rot_min_speed: self.ducked_rot_min_speed,
            prone_rot_min_speed: self.prone_rot_min_speed,
            world_model,
            world_clip_model,
            rocket_model,
            mounted_model,
            additional_melee_model,
            hud_icon,
            hud_icon_ratio,
            indicator_icon,
            indicator_icon_ratio,
            ammo_counter_icon,
            ammo_counter_icon_ratio,
            ammo_counter_clip,
            start_ammo: self.start_ammo,
            head_index: self.head_index as _,
            max_ammo: self.max_ammo,
            shot_count: self.shot_count,
            shared_ammo_cap_name,
            shared_ammo_cap_index: self.shared_ammo_cap_index as _,
            shared_ammo_cap: self.shared_ammo_cap,
            unlimited_ammo: self.unlimited_ammo,
            ammo_count_clip_relative: self.ammo_count_clip_relative,
            damage: self.damage,
            damage_duration: self.damage_duration,
            damage_interval: self.damage_interval,
            player_damage: self.player_damage,
            melee_damage: self.melee_damage,
            damage_type: self.damage_type,
            explosion_tag,
            fire_delay: self.fire_delay,
            melee_delay: self.melee_delay,
            melee_charge_delay: self.melee_charge_delay,
            detonate_delay: self.detonate_delay,
            spin_up_time: self.spin_up_time,
            spin_down_time: self.spin_down_time,
            spin_rate: self.spin_rate,
            spin_loop_sound,
            spin_loop_sound_player,
            start_spin_sound,
            start_spin_sound_player,
            stop_spin_sound,
            stop_spin_sound_player,
            fire_time: self.fire_time,
            last_fire_time: self.last_fire_time,
            rechamber_time: self.rechamber_time,
            rechamber_bolt_time: self.rechamber_bolt_time,
            hold_fire_time: self.hold_fire_time,
            detonate_fire_time: self.detonate_fire_time,
            melee_time: self.melee_time,
            melee_charge_time: self.melee_charge_time,
            reload_time_right: self.reload_time_right,
            reload_time_left: self.reload_time_left,
            reload_show_rocket_time: self.reload_show_rocket_time,
            reload_empty_time_left: self.reload_empty_time_left,
            reload_empty_add_time: self.reload_empty_add_time,
            reload_add_time: self.reload_add_time,
            reload_quick_add_time: self.reload_quick_add_time,
            reload_quick_empty_add_time: self.reload_quick_empty_add_time,
            reload_start_time: self.reload_start_time,
            reload_start_add_time: self.reload_start_add_time,
            reload_end_time: self.reload_end_time,
            drop_time: self.drop_time,
            raise_time: self.raise_time,
            alt_drop_time: self.alt_drop_time,
            quick_drop_time: self.quick_drop_time,
            quick_raise_time: self.quick_raise_time,
            first_raise_time: self.first_raise_time,
            empty_raise_time: self.empty_raise_time,
            empty_drop_time: self.empty_drop_time,
            sprint_in_time: self.sprint_in_time,
            sprint_loop_time: self.sprint_loop_time,
            sprint_out_time: self.sprint_out_time,
            low_ready_in_time: self.low_ready_in_time,
            low_ready_loop_time: self.low_ready_loop_time,
            low_ready_out_time: self.low_ready_out_time,
            cont_fire_in_time: self.cont_fire_in_time,
            cont_fire_loop_time: self.cont_fire_loop_time,
            cont_fire_out_time: self.cont_fire_out_time,
            dtp_in_time: self.dtp_in_time,
            dtp_loop_time: self.dtp_loop_time,
            dtp_out_time: self.dtp_out_time,
            slide_in_time: self.slide_in_time,
            deploy_time: self.deploy_time,
            breakdown_time: self.breakdown_time,
            night_vision_wear_time: self.night_vision_wear_time,
            night_vision_wear_time_fade_out_end: self.night_vision_wear_time_fade_out_end,
            night_vision_wear_time_power_up: self.night_vision_wear_time_power_up,
            night_vision_remove_time: self.night_vision_remove_time,
            night_vision_remove_time_power_down: self.night_vision_remove_time_power_down,
            night_vision_remove_time_fade_in_start: self.night_vision_remove_time_fade_in_start,
            fuse_time: self.fuse_time,
            ai_fuse_time: self.ai_fuse_time,
            lock_on_radius: self.lock_on_radius,
            lock_on_speed: self.lock_on_speed,
            require_lockon_to_fire: self.require_lockon_to_fire,
            no_ads_when_mag_empty: self.no_ads_when_mag_empty,
            avoid_drop_cleanup: self.avoid_drop_cleanup,
            stack_fire: self.stack_fire,
            stack_fire_spread: self.stack_fire_spread,
            stack_fire_accuracy_decay: self.stack_fire_accuracy_decay,
            stack_sound,
            auto_aim_range: self.auto_aim_range,
            aim_assist_range: self.aim_assist_range,
            mountable_weapon: self.mountable_weapon,
            aim_padding: self.aim_padding,
            enemy_crosshair_range: self.enemy_crosshair_range,
            crosshair_color_change: self.crosshair_color_change,
            move_speed_scale: self.move_speed_scale,
            ads_move_speed_scale: self.ads_move_speed_scale,
            sprint_duration_scale: self.sprint_duration_scale,
            overlay_reticle,
            overlay_interface,
            overlay_width: self.overlay_width,
            overlay_height: self.overlay_height,
            ads_bob_factor: self.ads_bob_factor,
            ads_view_bob_mult: self.ads_view_bob_mult,
            hip_spread_stand_min: self.hip_spread_stand_min,
            hip_spread_ducked_min: self.hip_spread_ducked_min,
            hip_spread_prone_min: self.hip_spread_prone_min,
            hip_spread_stand_max: self.hip_spread_stand_max,
            hip_spread_ducked_max: self.hip_spread_ducked_max,
            hip_spread_prone_max: self.hip_spread_prone_max,
            hip_spread_decay_rate: self.hip_spread_decay_rate,
            hip_spread_fire_add: self.hip_spread_fire_add,
            hip_spread_turn_add: self.hip_spread_turn_add,
            hip_spread_move_add: self.hip_spread_move_add,
            hip_spread_ducked_decay: self.hip_spread_ducked_decay,
            hip_spread_prone_decay: self.hip_spread_prone_decay,
            hip_reticle_side_pos: self.hip_reticle_side_pos,
            ads_idle_amount: self.ads_idle_amount,
            hip_idle_amount: self.hip_idle_amount,
            ads_idle_speed: self.ads_idle_speed,
            hip_idle_speed: self.hip_idle_speed,
            idle_crouch_factor: self.idle_crouch_factor,
            idle_prone_factor: self.idle_prone_factor,
            gun_max_pitch: self.gun_max_pitch,
            gun_max_yaw: self.gun_max_yaw,
            sway_max_angle: self.sway_max_angle,
            sway_lerp_speed: self.sway_lerp_speed,
            sway_pitch_scale: self.sway_pitch_scale,
            sway_yaw_scale: self.sway_yaw_scale,
            sway_horiz_scale: self.sway_horiz_scale,
            sway_vert_scale: self.sway_vert_scale,
            sway_shell_shock_scale: self.sway_shell_shock_scale,
            ads_sway_max_angle: self.ads_sway_max_angle,
            ads_sway_lerp_speed: self.ads_sway_lerp_speed,
            ads_sway_pitch_scale: self.ads_sway_pitch_scale,
            ads_sway_yaw_scale: self.ads_sway_yaw_scale,
            shared_ammo: self.shared_ammo,
            rifle_bullet: self.rifle_bullet,
            armor_piercing: self.armor_piercing,
            bolt_action: self.bolt_action,
            use_alt_tag_flesh: self.use_alt_tag_flesh,
            use_anti_lag_rewind: self.use_anti_lag_rewind,
            is_carried_killstreak_weapon: self.is_carried_killstreak_weapon,
            aim_down_sight: self.aim_down_sight,
            rechamber_while_ads: self.rechamber_while_ads,
            reload_while_ads: self.reload_while_ads,
            ads_view_error_min: self.ads_view_error_min,
            ads_view_error_max: self.ads_view_error_max,
            cook_off_hold: self.cook_off_hold,
            clip_only: self.clip_only,
            can_use_in_vehicle: self.can_use_in_vehicle,
            no_drops_or_raises: self.no_drops_or_raises,
            ads_fire_only: self.ads_fire_only,
            cancel_auto_holster_when_empty: self.cancel_auto_holster_when_empty,
            suppress_ammo_reserve_display: self.suppress_ammo_reserve_display,
            laser_sight_during_nightvision: self.laser_sight_during_nightvision,
            hide_third_person: self.hide_third_person,
            has_bayonet: self.has_bayonet,
            dual_wield: self.dual_wield,
            explode_on_ground: self.explode_on_ground,
            throw_back: self.throw_back,
            retrievable: self.retrievable,
            die_on_respawn: self.die_on_respawn,
            no_third_person_drops_or_raises: self.no_third_person_drops_or_raises,
            continuous_fire: self.continuous_fire,
            no_ping: self.no_ping,
            force_bounce: self.force_bounce,
            use_dropped_model_as_stowed: self.use_dropped_model_as_stowed,
            no_quick_drop_when_empty: self.no_quick_drop_when_empty,
            keep_crosshair_when_ads: self.keep_crosshair_when_ads,
            use_only_alt_weaopon_hide_tags_in_alt_mode: self
                .use_only_alt_weaopon_hide_tags_in_alt_mode,
            kill_icon,
            kill_icon_ratio,
            flip_kill_icon: self.flip_kill_icon,
            no_partial_reload: self.no_partial_reload,
            segmented_reload: self.segmented_reload,
            no_ads_auto_reload: self.no_ads_auto_reload,
            reload_ammo_add: self.reload_ammo_add,
            reload_start_add: self.reload_start_add,
            spawned_grenade_weapon_name,
            dual_wield_weapon_name,
            dual_wield_weapon_index: self.dual_wield_weapon_index as _,
            drop_ammo_min: self.drop_ammo_min,
            drop_ammo_max: self.drop_ammo_max,
            drop_clip_ammo_min: self.drop_clip_ammo_min,
            drop_clip_ammo_max: self.drop_clip_ammo_max,
            blocks_prone: self.blocks_prone,
            show_indicator: self.show_indicator,
            is_rolling_grenade: self.is_rolling_grenade,
            explosion_radius: self.explosion_radius,
            explosion_radius_min: self.explosion_radius_min,
            indicator_radius: self.indicator_radius,
            explosion_inner_damage: self.explosion_inner_damage,
            explosion_outer_damage: self.explosion_outer_damage,
            damage_cone_angle: self.damage_cone_angle,
            projectile_speed: self.projectile_speed,
            projectile_speed_up: self.projectile_speed_up,
            projectile_speed_relative_up: self.projectile_speed_relative_up,
            projectile_speed_forward: self.projectile_speed_forward,
            projectile_active_dist: self.projectile_active_dist,
            proj_lifetime: self.proj_lifetime,
            time_to_accelerate: self.time_to_accelerate,
            projectile_curvature: self.projectile_curvature,
            projectile_model,
            proj_explosion,
            proj_explosion_effect,
            proj_explosion_effect_force_normal_up: self.proj_explosion_effect_force_normal_up,
            proj_explosion_effect_2,
            proj_explosion_effect_2_force_normal_up: self.proj_explosion_effect_2_force_normal_up,
            proj_explosion_effect_3,
            proj_explosion_effect_3_force_normal_up: self.proj_explosion_effect_3_force_normal_up,
            proj_explosion_effect_4,
            proj_explosion_effect_4_force_normal_up: self.proj_explosion_effect_4_force_normal_up,
            proj_explosion_effect_5,
            proj_explosion_effect_5_force_normal_up: self.proj_explosion_effect_5_force_normal_up,
            proj_dud_effect,
            proj_explosion_sound,
            proj_dud_sound,
            mortar_shell_sound,
            tank_shell_sound,
            proj_impact_explode: self.proj_impact_explode,
            bullet_impact_explode: self.bullet_impact_explode,
            stickiness,
            rotate_type,
            plantable: self.plantable,
            has_detonator: self.has_detonator,
            time_detonation: self.time_detonation,
            no_crumple_missile: self.no_crumple_missile,
            rotate: self.rotate,
            keep_rolling: self.keep_rolling,
            hold_button_to_throw: self.hold_button_to_throw,
            offhand_hold_is_cancelable: self.offhand_hold_is_cancelable,
            freeze_movement_when_firing: self.freeze_movement_when_firing,
            low_ammo_warning_threshold: self.low_ammo_warning_threshold,
            melee_charge_range: self.melee_charge_range,
            use_as_melee: self.use_as_melee,
            is_camera_sensor: self.is_camera_sensor,
            is_acoustic_sensor: self.is_acoustic_sensor,
            parallel_bounce,
            perpendicular_bounce,
            proj_tail_effect,
            projectile_color: self.projectile_color.into(),
            guided_missile_type,
            max_steering_accel: self.max_steering_accel,
            proj_ignition_delay: self.proj_ignition_delay,
            proj_ignition_effect,
            proj_ignition_sound,
            ads_aim_pitch: self.ads_aim_pitch,
            ads_crosshair_in_frac: self.ads_crosshair_in_frac,
            ads_crosshair_out_frac: self.ads_crosshair_out_frac,
            ads_gun_kick_reduced_kick_bullets: self.ads_gun_kick_reduced_kick_bullets,
            ads_gun_kick_reduced_kick_percent: self.ads_gun_kick_reduced_kick_percent,
            ads_gun_kick_pitch_min: self.ads_gun_kick_pitch_min,
            ads_gun_kick_pitch_max: self.ads_gun_kick_pitch_max,
            ads_gun_kick_yaw_min: self.ads_gun_kick_yaw_min,
            ads_gun_kick_yaw_max: self.ads_gun_kick_yaw_max,
            ads_gun_kick_accel: self.ads_gun_kick_accel,
            ads_gun_kick_speed_max: self.ads_gun_kick_speed_max,
            ads_gun_kick_speed_decay: self.ads_gun_kick_speed_decay,
            ads_gun_kick_static_decay: self.ads_gun_kick_static_decay,
            ads_view_kick_pitch_min: self.ads_view_kick_pitch_min,
            ads_view_kick_pitch_max: self.ads_view_kick_pitch_max,
            ads_view_kick_yaw_min: self.ads_view_kick_yaw_min,
            ads_view_kick_yaw_max: self.ads_view_kick_yaw_max,
            ads_view_scatter_min: self.ads_view_scatter_min,
            ads_view_scatter_max: self.ads_view_scatter_max,
            ads_spread: self.ads_spread,
            hip_gun_kick_reduced_kick_bullets: self.hip_gun_kick_reduced_kick_bullets,
            hip_gun_kick_reduced_kick_percent: self.hip_gun_kick_reduced_kick_percent,
            hip_gun_kick_pitch_min: self.hip_gun_kick_pitch_min,
            hip_gun_kick_pitch_max: self.hip_gun_kick_pitch_max,
            hip_gun_kick_yaw_min: self.hip_gun_kick_yaw_min,
            hip_gun_kick_yaw_max: self.hip_gun_kick_yaw_max,
            hip_gun_kick_accel: self.hip_gun_kick_accel,
            hip_gun_kick_speed_max: self.hip_gun_kick_speed_max,
            hip_gun_kick_speed_decay: self.hip_gun_kick_speed_decay,
            hip_gun_kick_static_decay: self.hip_gun_kick_static_decay,
            hip_view_kick_pitch_min: self.hip_view_kick_pitch_min,
            hip_view_kick_pitch_max: self.hip_view_kick_pitch_max,
            hip_view_kick_yaw_min: self.hip_view_kick_yaw_min,
            hip_view_kick_yaw_max: self.hip_view_kick_yaw_max,
            hip_view_scatter_min: self.hip_view_scatter_min,
            hip_view_scatter_max: self.hip_view_scatter_max,
            fight_dist: self.fight_dist,
            max_dist: self.max_dist,
            accuracy_graph_name,
            accuracy_graph_knots,
            original_accuracy_graph_knots,
            accuracy_graph_knot_count: self.accuracy_graph_knot_count,
            original_accuracy_graph_knot_count: self.original_accuracy_graph_knot_count,
            position_reload_trans_time: self.position_reload_trans_time,
            left_arc: self.left_arc,
            right_arc: self.right_arc,
            top_arc: self.top_arc,
            bottom_arc: self.bottom_arc,
            accuracy: self.accuracy,
            ai_spread: self.ai_spread,
            player_spread: self.player_spread,
            min_turn_speed: self.min_turn_speed.into(),
            max_turn_speed: self.max_turn_speed.into(),
            pitch_convergence_time: self.pitch_convergence_time,
            yaw_convergence_time: self.yaw_convergence_time,
            suppress_time: self.suppress_time,
            max_range: self.max_range,
            anim_hor_rotate_inc: self.anim_hor_rotate_inc,
            player_position_dist: self.player_position_dist,
            use_hint_string,
            drop_hint_string,
            use_hint_string_index: self.use_hint_string_index as _,
            drop_hint_string_index: self.drop_hint_string_index as _,
            horiz_view_jitter: self.horiz_view_jitter,
            vert_view_jitter: self.vert_view_jitter,
            script,
            min_damage: self.min_damage,
            min_player_damage: self.min_player_damage,
            max_damage_range: self.max_damage_range,
            min_damage_range: self.min_damage_range,
            destabilization_rate_time: self.destabilization_rate_time,
            destabilization_curvature_max: self.destabilization_curvature_max,
            destabilize_distance: self.destabilize_distance,
            location_damage_multipliers,
            fire_rumble,
            melee_impact_rumble,
            reload_rumble,
            ads_dof_start: self.ads_dof_start,
            ads_dof_end: self.ads_dof_end,
            hip_dof_start: self.hip_dof_start,
            hip_dof_end: self.hip_dof_end,
            scan_speed: self.scan_speed,
            scan_accel: self.scan_accel,
            scan_pause_time: self.scan_pause_time,
            flame_table_first_person,
            flame_table_third_person,
            flame_table_first_person_ptr,
            flame_table_third_person_ptr,
            tag_fx_preparation_effect,
            tag_flash_preparation_effect,
            do_gibbing: self.do_gibbing,
            max_gib_distance: self.max_gib_distance,
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct FlameTableRaw<'a> {
    pub flame_var_stream_chunk_gravity_start: f32,
    pub flame_var_stream_chunk_gravity_end: f32,
    pub flame_var_stream_chunk_max_size: f32,
    pub flame_var_stream_chunk_start_size: f32,
    pub flame_var_stream_chunk_end_size: f32,
    pub flame_var_stream_chunk_start_size_rand: f32,
    pub flame_var_stream_chunk_end_size_rand: f32,
    pub flame_var_stream_chunk_dist_scalar: f32,
    pub flame_var_stream_chunk_dist_sway_scale: f32,
    pub flame_var_stream_chunk_dist_sway_vel_max: f32,
    pub flame_var_stream_chunk_speed: f32,
    pub flame_var_stream_chunk_decel: f32,
    pub flame_var_stream_chunk_velocity_add: f32,
    pub flame_var_stream_chunk_duration: f32,
    pub flame_var_stream_chunk_duration_scale_max_vel: f32,
    pub flame_var_stream_chunk_duration_vel_scalar: f32,
    pub flame_var_stream_chunk_size_speed_scale: f32,
    pub flame_var_stream_chunk_size_age_scale: f32,
    pub flame_var_stream_chunk_spawn_fire_interval_start: f32,
    pub flame_var_stream_chunk_spawn_fire_interval_end: f32,
    pub flame_var_stream_chunk_spawn_fire_min_life_frac: f32,
    pub flame_var_stream_chunk_spawn_fire_max_life_frac: f32,
    pub flame_var_stream_chunk_fire_min_life_frac: f32,
    pub flame_var_stream_chunk_fire_min_life_frac_start: f32,
    pub flame_var_stream_chunk_fire_min_life_frac_end: f32,
    pub flame_var_stream_chunk_drips_min_life_frac: f32,
    pub flame_var_stream_chunk_drips_min_life_frac_start: f32,
    pub flame_var_stream_chunk_drips_min_life_frac_end: f32,
    pub flame_var_stream_chunk_rotation_range: f32,
    pub flame_var_stream_size_rand_sin_wave: f32,
    pub flame_var_stream_size_rand_cos_wave: f32,
    pub flame_var_stream_drips_chunk_interval: f32,
    pub flame_var_stream_drips_chunk_min_frac: f32,
    pub flame_var_stream_drips_chunk_rand_frac: f32,
    pub flame_var_stream_smoke_chunk_interval: f32,
    pub flame_var_stream_smoke_chunk_min_frac: f32,
    pub flame_var_stream_smoke_chunk_rand_frac: f32,
    pub flame_var_stream_chunk_cull_dist_size_frac: f32,
    pub flame_var_stream_chunk_cull_min_life: f32,
    pub flame_var_stream_chunk_cull_max_life: f32,
    pub flame_var_stream_fuel_size_start: f32,
    pub flame_var_stream_fuel_size_end: f32,
    pub flame_var_stream_fuel_length: f32,
    pub flame_var_stream_fuel_num_segments: f32,
    pub flame_var_stream_fuel_anim_loop_time: f32,
    pub flame_var_stream_flame_size_start: f32,
    pub flame_var_stream_flame_size_end: f32,
    pub flame_var_stream_flame_length: f32,
    pub flame_var_stream_flame_num_sgments: f32,
    pub flame_var_stream_flame_anim_loop_time: f32,
    pub flame_var_stream_primary_light_radius: f32,
    pub flame_var_stream_primary_light_radius_flutter: f32,
    pub flame_var_stream_primary_light_r: f32,
    pub flame_var_stream_primary_light_g: f32,
    pub flame_var_stream_primary_light_b: f32,
    pub flame_var_stream_primary_light_flutter_r: f32,
    pub flame_var_stream_primary_light_flutter_g: f32,
    pub flame_var_stream_primary_light_flutter_b: f32,
    pub flame_var_fire_life: f32,
    pub flame_var_fire_life_rand: f32,
    pub flame_var_fire_speed_scale: f32,
    pub flame_var_fire_speed_scale_rand: f32,
    pub flame_var_fire_velocity_add_z: f32,
    pub flame_var_fire_velocity_add_z_rand: f32,
    pub flame_var_fire_velocity_add_sideways: f32,
    pub flame_var_fire_gravity: f32,
    pub flame_var_fire_gravity_end: f32,
    pub flame_var_fire_max_rot_vel: f32,
    pub flame_var_fire_friction: f32,
    pub flame_var_fire_end_size_add: f32,
    pub flame_var_fire_start_size_scale: f32,
    pub flame_var_fire_end_size_scale: f32,
    pub flame_var_drips_life: f32,
    pub flame_var_drips_life_rand: f32,
    pub flame_var_drips_speed_scale: f32,
    pub flame_var_drips_speed_scale_rand: f32,
    pub flame_var_drips_velocity_add_z: f32,
    pub flame_var_drips_velocity_add_z_rand: f32,
    pub flame_var_drips_velocity_add_sideways: f32,
    pub flame_var_drips_gravity: f32,
    pub flame_var_drips_gravity_end: f32,
    pub flame_var_drips_max_rot_vel: f32,
    pub flame_var_drips_friction: f32,
    pub flame_var_drips_end_size_add: f32,
    pub flame_var_drips_start_size_scale: f32,
    pub flame_var_drips_end_size_scale: f32,
    pub flame_var_smoke_life: f32,
    pub flame_var_smoke_life_rand: f32,
    pub flame_var_smoke_speed_scale: f32,
    pub flame_var_smoke_velocity_add_z: f32,
    pub flame_var_smoke_gravity: f32,
    pub flame_var_smoke_gravity_end: f32,
    pub flame_var_smoke_max_rotation: f32,
    pub flame_var_smoke_max_rot_vel: f32,
    pub flame_var_smoke_friction: f32,
    pub flame_var_smoke_end_size_add: f32,
    pub flame_var_smoke_start_size_add: f32,
    pub flame_var_smoke_origin_size_ofs_z_scale: f32,
    pub flame_var_smoke_origin_ofs_z: f32,
    pub flame_var_smoke_fadein: f32,
    pub flame_var_smoke_fadeout: f32,
    pub flame_var_smoke_max_alpha: f32,
    pub flame_var_smoke_brightness: f32,
    pub flame_var_smoke_origin_offset: f32,
    pub flame_var_collision_speed_scale: f32,
    pub flame_var_collision_volume_scale: f32,
    pub name: XStringRaw<'a>,
    pub fire: Ptr32<'a, techset::MaterialRaw<'a>>,
    pub smoke: Ptr32<'a, techset::MaterialRaw<'a>>,
    pub heat: Ptr32<'a, techset::MaterialRaw<'a>>,
    pub drips: Ptr32<'a, techset::MaterialRaw<'a>>,
    pub stream_fuel: Ptr32<'a, techset::MaterialRaw<'a>>,
    pub stream_fuel_2: Ptr32<'a, techset::MaterialRaw<'a>>,
    pub stream_flame: Ptr32<'a, techset::MaterialRaw<'a>>,
    pub stream_flame_2: Ptr32<'a, techset::MaterialRaw<'a>>,
    pub flame_off_loop_sound: XStringRaw<'a>,
    pub flame_ignite_sound: XStringRaw<'a>,
    pub flame_on_loop_sound: XStringRaw<'a>,
    pub flame_cooldown_sound: XStringRaw<'a>,
}
assert_size!(FlameTableRaw, 476);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Default, Debug)]
pub struct FlameTable {
    pub flame_var_stream_chunk_gravity_start: f32,
    pub flame_var_stream_chunk_gravity_end: f32,
    pub flame_var_stream_chunk_max_size: f32,
    pub flame_var_stream_chunk_start_size: f32,
    pub flame_var_stream_chunk_end_size: f32,
    pub flame_var_stream_chunk_start_size_rand: f32,
    pub flame_var_stream_chunk_end_size_rand: f32,
    pub flame_var_stream_chunk_dist_scalar: f32,
    pub flame_var_stream_chunk_dist_sway_scale: f32,
    pub flame_var_stream_chunk_dist_sway_vel_max: f32,
    pub flame_var_stream_chunk_speed: f32,
    pub flame_var_stream_chunk_decel: f32,
    pub flame_var_stream_chunk_velocity_add: f32,
    pub flame_var_stream_chunk_duration: f32,
    pub flame_var_stream_chunk_duration_scale_max_vel: f32,
    pub flame_var_stream_chunk_duration_vel_scalar: f32,
    pub flame_var_stream_chunk_size_speed_scale: f32,
    pub flame_var_stream_chunk_size_age_scale: f32,
    pub flame_var_stream_chunk_spawn_fire_interval_start: f32,
    pub flame_var_stream_chunk_spawn_fire_interval_end: f32,
    pub flame_var_stream_chunk_spawn_fire_min_life_frac: f32,
    pub flame_var_stream_chunk_spawn_fire_max_life_frac: f32,
    pub flame_var_stream_chunk_fire_min_life_frac: f32,
    pub flame_var_stream_chunk_fire_min_life_frac_start: f32,
    pub flame_var_stream_chunk_fire_min_life_frac_end: f32,
    pub flame_var_stream_chunk_drips_min_life_frac: f32,
    pub flame_var_stream_chunk_drips_min_life_frac_start: f32,
    pub flame_var_stream_chunk_drips_min_life_frac_end: f32,
    pub flame_var_stream_chunk_rotation_range: f32,
    pub flame_var_stream_size_rand_sin_wave: f32,
    pub flame_var_stream_size_rand_cos_wave: f32,
    pub flame_var_stream_drips_chunk_interval: f32,
    pub flame_var_stream_drips_chunk_min_frac: f32,
    pub flame_var_stream_drips_chunk_rand_frac: f32,
    pub flame_var_stream_smoke_chunk_interval: f32,
    pub flame_var_stream_smoke_chunk_min_frac: f32,
    pub flame_var_stream_smoke_chunk_rand_frac: f32,
    pub flame_var_stream_chunk_cull_dist_size_frac: f32,
    pub flame_var_stream_chunk_cull_min_life: f32,
    pub flame_var_stream_chunk_cull_max_life: f32,
    pub flame_var_stream_fuel_size_start: f32,
    pub flame_var_stream_fuel_size_end: f32,
    pub flame_var_stream_fuel_length: f32,
    pub flame_var_stream_fuel_num_segments: f32,
    pub flame_var_stream_fuel_anim_loop_time: f32,
    pub flame_var_stream_flame_size_start: f32,
    pub flame_var_stream_flame_size_end: f32,
    pub flame_var_stream_flame_length: f32,
    pub flame_var_stream_flame_num_sgments: f32,
    pub flame_var_stream_flame_anim_loop_time: f32,
    pub flame_var_stream_primary_light_radius: f32,
    pub flame_var_stream_primary_light_radius_flutter: f32,
    pub flame_var_stream_primary_light_r: f32,
    pub flame_var_stream_primary_light_g: f32,
    pub flame_var_stream_primary_light_b: f32,
    pub flame_var_stream_primary_light_flutter_r: f32,
    pub flame_var_stream_primary_light_flutter_g: f32,
    pub flame_var_stream_primary_light_flutter_b: f32,
    pub flame_var_fire_life: f32,
    pub flame_var_fire_life_rand: f32,
    pub flame_var_fire_speed_scale: f32,
    pub flame_var_fire_speed_scale_rand: f32,
    pub flame_var_fire_velocity_add_z: f32,
    pub flame_var_fire_velocity_add_z_rand: f32,
    pub flame_var_fire_velocity_add_sideways: f32,
    pub flame_var_fire_gravity: f32,
    pub flame_var_fire_gravity_end: f32,
    pub flame_var_fire_max_rot_vel: f32,
    pub flame_var_fire_friction: f32,
    pub flame_var_fire_end_size_add: f32,
    pub flame_var_fire_start_size_scale: f32,
    pub flame_var_fire_end_size_scale: f32,
    pub flame_var_drips_life: f32,
    pub flame_var_drips_life_rand: f32,
    pub flame_var_drips_speed_scale: f32,
    pub flame_var_drips_speed_scale_rand: f32,
    pub flame_var_drips_velocity_add_z: f32,
    pub flame_var_drips_velocity_add_z_rand: f32,
    pub flame_var_drips_velocity_add_sideways: f32,
    pub flame_var_drips_gravity: f32,
    pub flame_var_drips_gravity_end: f32,
    pub flame_var_drips_max_rot_vel: f32,
    pub flame_var_drips_friction: f32,
    pub flame_var_drips_end_size_add: f32,
    pub flame_var_drips_start_size_scale: f32,
    pub flame_var_drips_end_size_scale: f32,
    pub flame_var_smoke_life: f32,
    pub flame_var_smoke_life_rand: f32,
    pub flame_var_smoke_speed_scale: f32,
    pub flame_var_smoke_velocity_add_z: f32,
    pub flame_var_smoke_gravity: f32,
    pub flame_var_smoke_gravity_end: f32,
    pub flame_var_smoke_max_rotation: f32,
    pub flame_var_smoke_max_rot_vel: f32,
    pub flame_var_smoke_friction: f32,
    pub flame_var_smoke_end_size_add: f32,
    pub flame_var_smoke_start_size_add: f32,
    pub flame_var_smoke_origin_size_ofs_z_scale: f32,
    pub flame_var_smoke_origin_ofs_z: f32,
    pub flame_var_smoke_fadein: f32,
    pub flame_var_smoke_fadeout: f32,
    pub flame_var_smoke_max_alpha: f32,
    pub flame_var_smoke_brightness: f32,
    pub flame_var_smoke_origin_offset: f32,
    pub flame_var_collision_speed_scale: f32,
    pub flame_var_collision_volume_scale: f32,
    pub name: XString,
    pub fire: Option<Box<techset::Material>>,
    pub smoke: Option<Box<techset::Material>>,
    pub heat: Option<Box<techset::Material>>,
    pub drips: Option<Box<techset::Material>>,
    pub stream_fuel: Option<Box<techset::Material>>,
    pub stream_fuel_2: Option<Box<techset::Material>>,
    pub stream_flame: Option<Box<techset::Material>>,
    pub stream_flame_2: Option<Box<techset::Material>>,
    pub flame_off_loop_sound: XString,
    pub flame_ignite_sound: XString,
    pub flame_on_loop_sound: XString,
    pub flame_cooldown_sound: XString,
}

impl<'a> XFileDeserializeInto<FlameTable, ()> for FlameTableRaw<'a> {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<FlameTable> {
        let name = self.name.xfile_deserialize_into(de, ())?;
        let fire = self.fire.xfile_deserialize_into(de, ())?;
        let smoke = self.smoke.xfile_deserialize_into(de, ())?;
        let heat = self.heat.xfile_deserialize_into(de, ())?;
        let drips = self.drips.xfile_deserialize_into(de, ())?;
        let stream_fuel = self.stream_fuel.xfile_deserialize_into(de, ())?;
        let stream_fuel_2 = self.stream_fuel_2.xfile_deserialize_into(de, ())?;
        let stream_flame = self.stream_flame.xfile_deserialize_into(de, ())?;
        let stream_flame_2 = self.stream_flame_2.xfile_deserialize_into(de, ())?;
        let flame_off_loop_sound = self.flame_off_loop_sound.xfile_deserialize_into(de, ())?;
        let flame_ignite_sound = self.flame_ignite_sound.xfile_deserialize_into(de, ())?;
        let flame_on_loop_sound = self.flame_on_loop_sound.xfile_deserialize_into(de, ())?;
        let flame_cooldown_sound = self.flame_cooldown_sound.xfile_deserialize_into(de, ())?;

        Ok(FlameTable {
            flame_var_stream_chunk_gravity_start: self.flame_var_stream_chunk_gravity_start,
            flame_var_stream_chunk_gravity_end: self.flame_var_stream_chunk_gravity_end,
            flame_var_stream_chunk_max_size: self.flame_var_stream_chunk_max_size,
            flame_var_stream_chunk_start_size: self.flame_var_stream_chunk_start_size,
            flame_var_stream_chunk_end_size: self.flame_var_stream_chunk_end_size,
            flame_var_stream_chunk_start_size_rand: self.flame_var_stream_chunk_start_size_rand,
            flame_var_stream_chunk_end_size_rand: self.flame_var_stream_chunk_end_size_rand,
            flame_var_stream_chunk_dist_scalar: self.flame_var_stream_chunk_dist_scalar,
            flame_var_stream_chunk_dist_sway_scale: self.flame_var_stream_chunk_dist_sway_scale,
            flame_var_stream_chunk_dist_sway_vel_max: self.flame_var_stream_chunk_dist_sway_vel_max,
            flame_var_stream_chunk_speed: self.flame_var_stream_chunk_speed,
            flame_var_stream_chunk_decel: self.flame_var_stream_chunk_decel,
            flame_var_stream_chunk_velocity_add: self.flame_var_stream_chunk_velocity_add,
            flame_var_stream_chunk_duration: self.flame_var_stream_chunk_duration,
            flame_var_stream_chunk_duration_scale_max_vel: self
                .flame_var_stream_chunk_duration_scale_max_vel,
            flame_var_stream_chunk_duration_vel_scalar: self
                .flame_var_stream_chunk_duration_vel_scalar,
            flame_var_stream_chunk_size_speed_scale: self.flame_var_stream_chunk_size_speed_scale,
            flame_var_stream_chunk_size_age_scale: self.flame_var_stream_chunk_size_age_scale,
            flame_var_stream_chunk_spawn_fire_interval_start: self
                .flame_var_stream_chunk_spawn_fire_interval_start,
            flame_var_stream_chunk_spawn_fire_interval_end: self
                .flame_var_stream_chunk_spawn_fire_interval_end,
            flame_var_stream_chunk_spawn_fire_min_life_frac: self
                .flame_var_stream_chunk_spawn_fire_min_life_frac,
            flame_var_stream_chunk_spawn_fire_max_life_frac: self
                .flame_var_stream_chunk_spawn_fire_max_life_frac,
            flame_var_stream_chunk_fire_min_life_frac: self
                .flame_var_stream_chunk_fire_min_life_frac,
            flame_var_stream_chunk_fire_min_life_frac_start: self
                .flame_var_stream_chunk_fire_min_life_frac_start,
            flame_var_stream_chunk_fire_min_life_frac_end: self
                .flame_var_stream_chunk_fire_min_life_frac_end,
            flame_var_stream_chunk_drips_min_life_frac: self
                .flame_var_stream_chunk_drips_min_life_frac,
            flame_var_stream_chunk_drips_min_life_frac_start: self
                .flame_var_stream_chunk_drips_min_life_frac_start,
            flame_var_stream_chunk_drips_min_life_frac_end: self
                .flame_var_stream_chunk_drips_min_life_frac_end,
            flame_var_stream_chunk_rotation_range: self.flame_var_stream_chunk_rotation_range,
            flame_var_stream_size_rand_sin_wave: self.flame_var_stream_size_rand_sin_wave,
            flame_var_stream_size_rand_cos_wave: self.flame_var_stream_size_rand_cos_wave,
            flame_var_stream_drips_chunk_interval: self.flame_var_stream_drips_chunk_interval,
            flame_var_stream_drips_chunk_min_frac: self.flame_var_stream_drips_chunk_min_frac,
            flame_var_stream_drips_chunk_rand_frac: self.flame_var_stream_drips_chunk_rand_frac,
            flame_var_stream_smoke_chunk_interval: self.flame_var_stream_smoke_chunk_interval,
            flame_var_stream_smoke_chunk_min_frac: self.flame_var_stream_smoke_chunk_min_frac,
            flame_var_stream_smoke_chunk_rand_frac: self.flame_var_stream_smoke_chunk_rand_frac,
            flame_var_stream_chunk_cull_dist_size_frac: self
                .flame_var_stream_chunk_cull_dist_size_frac,
            flame_var_stream_chunk_cull_min_life: self.flame_var_stream_chunk_cull_min_life,
            flame_var_stream_chunk_cull_max_life: self.flame_var_stream_chunk_cull_max_life,
            flame_var_stream_fuel_size_start: self.flame_var_stream_fuel_size_start,
            flame_var_stream_fuel_size_end: self.flame_var_stream_fuel_size_end,
            flame_var_stream_fuel_length: self.flame_var_stream_fuel_length,
            flame_var_stream_fuel_num_segments: self.flame_var_stream_fuel_num_segments,
            flame_var_stream_fuel_anim_loop_time: self.flame_var_stream_fuel_anim_loop_time,
            flame_var_stream_flame_size_start: self.flame_var_stream_flame_size_start,
            flame_var_stream_flame_size_end: self.flame_var_stream_flame_size_end,
            flame_var_stream_flame_length: self.flame_var_stream_flame_length,
            flame_var_stream_flame_num_sgments: self.flame_var_stream_flame_num_sgments,
            flame_var_stream_flame_anim_loop_time: self.flame_var_stream_flame_anim_loop_time,
            flame_var_stream_primary_light_radius: self.flame_var_stream_primary_light_radius,
            flame_var_stream_primary_light_radius_flutter: self
                .flame_var_stream_primary_light_radius_flutter,
            flame_var_stream_primary_light_r: self.flame_var_stream_primary_light_r,
            flame_var_stream_primary_light_g: self.flame_var_stream_primary_light_g,
            flame_var_stream_primary_light_b: self.flame_var_stream_primary_light_b,
            flame_var_stream_primary_light_flutter_r: self.flame_var_stream_primary_light_flutter_r,
            flame_var_stream_primary_light_flutter_g: self.flame_var_stream_primary_light_flutter_g,
            flame_var_stream_primary_light_flutter_b: self.flame_var_stream_primary_light_flutter_b,
            flame_var_fire_life: self.flame_var_fire_life,
            flame_var_fire_life_rand: self.flame_var_fire_life_rand,
            flame_var_fire_speed_scale: self.flame_var_fire_speed_scale,
            flame_var_fire_speed_scale_rand: self.flame_var_fire_speed_scale_rand,
            flame_var_fire_velocity_add_z: self.flame_var_fire_velocity_add_z,
            flame_var_fire_velocity_add_z_rand: self.flame_var_fire_velocity_add_z_rand,
            flame_var_fire_velocity_add_sideways: self.flame_var_fire_velocity_add_sideways,
            flame_var_fire_gravity: self.flame_var_fire_gravity,
            flame_var_fire_gravity_end: self.flame_var_fire_gravity_end,
            flame_var_fire_max_rot_vel: self.flame_var_fire_max_rot_vel,
            flame_var_fire_friction: self.flame_var_fire_friction,
            flame_var_fire_end_size_add: self.flame_var_fire_end_size_add,
            flame_var_fire_start_size_scale: self.flame_var_fire_start_size_scale,
            flame_var_fire_end_size_scale: self.flame_var_fire_end_size_scale,
            flame_var_drips_life: self.flame_var_drips_life,
            flame_var_drips_life_rand: self.flame_var_drips_life_rand,
            flame_var_drips_speed_scale: self.flame_var_drips_speed_scale,
            flame_var_drips_speed_scale_rand: self.flame_var_drips_speed_scale_rand,
            flame_var_drips_velocity_add_z: self.flame_var_drips_velocity_add_z,
            flame_var_drips_velocity_add_z_rand: self.flame_var_drips_velocity_add_z_rand,
            flame_var_drips_velocity_add_sideways: self.flame_var_drips_velocity_add_sideways,
            flame_var_drips_gravity: self.flame_var_drips_gravity,
            flame_var_drips_gravity_end: self.flame_var_drips_gravity_end,
            flame_var_drips_max_rot_vel: self.flame_var_drips_max_rot_vel,
            flame_var_drips_friction: self.flame_var_drips_friction,
            flame_var_drips_end_size_add: self.flame_var_drips_end_size_add,
            flame_var_drips_start_size_scale: self.flame_var_drips_start_size_scale,
            flame_var_drips_end_size_scale: self.flame_var_drips_end_size_scale,
            flame_var_smoke_life: self.flame_var_smoke_life,
            flame_var_smoke_life_rand: self.flame_var_smoke_life_rand,
            flame_var_smoke_speed_scale: self.flame_var_smoke_speed_scale,
            flame_var_smoke_velocity_add_z: self.flame_var_smoke_velocity_add_z,
            flame_var_smoke_gravity: self.flame_var_smoke_gravity,
            flame_var_smoke_gravity_end: self.flame_var_smoke_gravity_end,
            flame_var_smoke_max_rotation: self.flame_var_smoke_max_rotation,
            flame_var_smoke_max_rot_vel: self.flame_var_smoke_max_rot_vel,
            flame_var_smoke_friction: self.flame_var_smoke_friction,
            flame_var_smoke_end_size_add: self.flame_var_smoke_end_size_add,
            flame_var_smoke_start_size_add: self.flame_var_smoke_start_size_add,
            flame_var_smoke_origin_size_ofs_z_scale: self.flame_var_smoke_origin_size_ofs_z_scale,
            flame_var_smoke_origin_ofs_z: self.flame_var_smoke_origin_ofs_z,
            flame_var_smoke_fadein: self.flame_var_smoke_fadein,
            flame_var_smoke_fadeout: self.flame_var_smoke_fadeout,
            flame_var_smoke_max_alpha: self.flame_var_smoke_max_alpha,
            flame_var_smoke_brightness: self.flame_var_smoke_brightness,
            flame_var_smoke_origin_offset: self.flame_var_smoke_origin_offset,
            flame_var_collision_speed_scale: self.flame_var_collision_speed_scale,
            flame_var_collision_volume_scale: self.flame_var_collision_volume_scale,
            name,
            fire,
            smoke,
            heat,
            drips,
            stream_fuel,
            stream_fuel_2,
            stream_flame,
            stream_flame_2,
            flame_off_loop_sound,
            flame_ignite_sound,
            flame_on_loop_sound,
            flame_cooldown_sound,
        })
    }
}
