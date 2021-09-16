use std::ops::Deref;

use crate::gui::*;

use super::MainMenuSelection;

pub(crate) async fn draw_character_select_menu() -> MainMenuSelection {
    let mut res = None;

    let mut selected_character_index = None;
    let mut delete_character_index = None;

    let mut characters = Vec::new();
    let mut should_refresh = true;

    loop {
        if should_refresh {
            let game_params = storage::get::<GameParams>();
            characters = get_available_characters(&game_params.characters_path).await.unwrap();
            characters.sort_by(|a, b| a.actor.name.cmp(&b.actor.name));
        }

        if let Some(i) = delete_character_index {
            let character: &SavedCharacter = characters.get(i).unwrap();

            let modal_body = vec!(
                "Are you sure you want to delete".to_string(),
                format!("'{}'?", character.actor.name),
            );

            match draw_confirmation_modal(&mut *root_ui(), modal_body) {
                Some(true) => {
                    should_refresh = delete_character(&character.actor.name);
                    delete_character_index = None;
                    selected_character_index = None;
                }
                Some(false) => delete_character_index = None,
                _ => {}
            }
        } else {
            if let Some(i) = selected_character_index {
                let character: &SavedCharacter = characters.get(i).unwrap();
                res = draw_character_details(&mut selected_character_index, &mut delete_character_index, character)
            } else {
                res = draw_character_list(&mut selected_character_index, &characters);
            }
        }

        if let Some(selection) = res {
            return selection;
        }

        next_frame().await;
    }
}

fn draw_character_list(select_i: &mut Option<usize>, characters: &Vec<SavedCharacter>) -> Option<MainMenuSelection> {
    const WINDOW_WIDTH: f32 = 300.0;
    const WINDOW_HEIGHT: f32 = 250.0;

    let size = vec2(WINDOW_WIDTH, WINDOW_HEIGHT);
    let position = get_centered_on_screen(size);

    let btn_size = vec2((WINDOW_WIDTH - GuiSkins::WINDOW_MARGIN_X * 2.0) / 2.0 - GuiSkins::ELEMENT_MARGIN, GuiSkins::BUTTON_HEIGHT);
    let btn_position_y = WINDOW_HEIGHT - GuiSkins::WINDOW_MARGIN_Y * 2.0 - GuiSkins::BUTTON_HEIGHT;

    let mut res = None;

    widgets::Window::new(hash!(), position, size)
        .titlebar(false)
        .ui(&mut *root_ui(), |ui| {
            let gui_skins = storage::get::<GuiSkins>();

            ui.push_skin(&gui_skins.header_label);
            ui.label(None, "Select Character");
            ui.pop_skin();

            let margins = vec2(GuiSkins::WINDOW_MARGIN_X, GuiSkins::WINDOW_MARGIN_Y) * 2.0;
            let size = vec2(size.x, size.y - 45.0 - GuiSkins::WINDOW_MARGIN_Y - GuiSkins::BUTTON_HEIGHT) - margins;

            widgets::Group::new(hash!(), size).position(vec2(0.0, 45.0)).ui(ui, |ui| {
                for i in 0..characters.len() {
                    let character = characters.get(i).unwrap();

                    let y_offset = i as f32 * 22.0;

                    ui.push_skin(&gui_skins.label_button);
                    if ui.button(vec2(2.0, y_offset), character.actor.name.deref()) {
                        *select_i = Some(i);
                    }
                    ui.pop_skin();
                }
            });

            let create_btn = widgets::Button::new("Create")
                .size(btn_size)
                .position(vec2(0.0, btn_position_y))
                .ui(ui);

            if create_btn {
                res = Some(MainMenuSelection::CreateCharacter);
            }

            let cancel_btn = widgets::Button::new("Cancel")
                .size(btn_size)
                .position(vec2(btn_size.x + GuiSkins::ELEMENT_MARGIN, btn_position_y))
                .ui(ui);

            if cancel_btn {
                res = Some(MainMenuSelection::Cancel);
            }
        });

    res
}

fn draw_character_details(select_i: &mut Option<usize>, delete_i: &mut Option<usize>, character: &SavedCharacter) -> Option<MainMenuSelection> {
    const WINDOW_WIDTH: f32 = 400.0;
    const WINDOW_HEIGHT: f32 = 500.0;

    let size = vec2(WINDOW_WIDTH, WINDOW_HEIGHT);
    let position = get_centered_on_screen(size);

    let btn_size = vec2((WINDOW_WIDTH - GuiSkins::WINDOW_MARGIN_X * 2.0) / 2.0 - GuiSkins::ELEMENT_MARGIN, GuiSkins::BUTTON_HEIGHT);
    let btn_position_y = WINDOW_HEIGHT - GuiSkins::WINDOW_MARGIN_Y * 2.0 - GuiSkins::BUTTON_HEIGHT;

    let mut res = None;

    widgets::Window::new(hash!(), position, size)
        .titlebar(false)
        .ui(&mut *root_ui(), |ui| {
            let gui_skins = storage::get::<GuiSkins>();

            ui.push_skin(&gui_skins.header_label);
            ui.label(None, &character.actor.name);
            ui.pop_skin();

            let delete_btn_size = vec2(WINDOW_WIDTH - GuiSkins::WINDOW_MARGIN_X * 2.0, GuiSkins::BUTTON_HEIGHT);
            let delete_btn_position = vec2(0.0, WINDOW_HEIGHT - GuiSkins::WINDOW_MARGIN_Y * 2.0 - GuiSkins::BUTTON_HEIGHT * 2.0 - GuiSkins::ELEMENT_MARGIN);

            let delete_btn = widgets::Button::new("Delete")
                .size(delete_btn_size)
                .position(delete_btn_position)
                .ui(ui);

            if delete_btn {
                let i = select_i.unwrap();
                *delete_i = Some(i);
            }

            let start_btn = widgets::Button::new("Start")
                .size(btn_size)
                .position(vec2(0.0, btn_position_y))
                .ui(ui);

            if start_btn {
                res = Some(MainMenuSelection::SelectCharacter(character.clone()));
            }

            let back_btn = widgets::Button::new("Back")
                .size(btn_size)
                .position(vec2(btn_size.x + GuiSkins::ELEMENT_MARGIN, btn_position_y))
                .ui(ui);

            if back_btn {
                *select_i = None;
            }
        });

    res
}
