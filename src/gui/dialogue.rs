use macroquad::{
    ui::{
        hash, root_ui,
        widgets::{self},
    },
    prelude::*,
};

use crate::nodes::Actor;

pub fn draw_dialogue_window(scale: f32, player: &mut Actor) {
    let interaction = player.current_dialogue.clone();
    let x = screen_width() / 2.0 - 200.0 * scale;
    let y = screen_height() / 2.0 - 175.0 * scale;
    if let Some(interaction) = interaction {
        widgets::Window::new(hash!(), vec2(x, y), vec2(400.0 * scale, 350.0 * scale))
            .titlebar(false)
            //.label(&interaction.actor_name)
            .ui(&mut *root_ui(), |ui| {
                if interaction.body.len() > 0 {
                    ui.label(None, &format!("{}:", player.name));
                }
                for line in interaction.body.clone() {
                    ui.label(None, &format!(" {}", line));
                }
                ui.separator();
                if interaction.response.len() > 0 {
                    ui.label(None, &format!("{}:", interaction.actor_name));
                }
                for line in  interaction.response.clone() {
                    ui.label(None, &format!(" {}", line));
                }
                ui.separator();
                let options = interaction.get_options(player);
                if options.len() == 0 {
                    if ui.button(None, "Bye!") {
                        player.current_dialogue = None;
                    }
                } else {
                    for option in options {
                        if ui.button(None, option.title.clone()) {
                            option.apply_action(player);
                            player.current_dialogue = Some(option);
                        }
                    }
                }
            });
    }
}
