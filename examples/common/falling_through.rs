use bevy::prelude::*;
use bevy_egui::egui;
use bevy_tnua::control_helpers::TnuaSimpleFallThroughPlatformsHelper;
use bevy_tnua::{TnuaGhostSensor, TnuaProximitySensor};

#[derive(Component, Debug, PartialEq)]
pub enum FallingThroughControlScheme {
    WithoutHelper,
    SingleFall,
    KeepFalling,
}

impl FallingThroughControlScheme {
    pub fn edit_with_ui(&mut self, ui: &mut egui::Ui) {
        egui::ComboBox::from_label("Falling Through Control Scheme")
            .selected_text(format!("{:?}", self))
            .show_ui(ui, |ui| {
                for variant in [
                    FallingThroughControlScheme::WithoutHelper,
                    FallingThroughControlScheme::SingleFall,
                    FallingThroughControlScheme::KeepFalling,
                ] {
                    if ui
                        .selectable_label(*self == variant, format!("{:?}", variant))
                        .clicked()
                    {
                        *self = variant;
                    }
                }
            });
    }

    pub fn perform_and_check_if_still_crouching(
        &self,
        crouch: bool,
        crouch_just_pressed: bool,
        fall_through_helper: &mut TnuaSimpleFallThroughPlatformsHelper,
        proximity_sensor: &mut TnuaProximitySensor,
        ghost_sensor: &TnuaGhostSensor,
        min_proximity: f32,
    ) -> bool {
        match self {
            FallingThroughControlScheme::WithoutHelper => {
                if let Some(ghost_platform) = ghost_sensor.0.first() {
                    if 1.0 <= ghost_platform.proximity {
                        if crouch {
                            return false;
                        } else {
                            proximity_sensor.output = Some(ghost_platform.clone());
                        }
                    }
                }
                crouch
            }
            FallingThroughControlScheme::SingleFall => {
                let mut fall_through_helper =
                    fall_through_helper.with(proximity_sensor, ghost_sensor, min_proximity);
                if crouch {
                    !fall_through_helper.try_falling_one_step_at_a_time(crouch_just_pressed)
                } else {
                    fall_through_helper.dont_fall();
                    false
                }
            }
            FallingThroughControlScheme::KeepFalling => {
                let mut fall_through_helper =
                    fall_through_helper.with(proximity_sensor, ghost_sensor, min_proximity);
                if crouch {
                    !fall_through_helper.try_falling()
                } else {
                    fall_through_helper.dont_fall();
                    false
                }
            }
        }
    }
}