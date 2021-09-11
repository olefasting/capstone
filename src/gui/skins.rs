use crate::{
    gui::*,
};

#[derive(Debug, Clone)]
pub struct GuiSkins {
    pub scale: f32,
    pub default: Skin,
    pub main_menu: Skin,
    pub warning_text: Skin,
    pub inactive_button: Skin,
    pub label_button: Skin,
    pub condensed_button: Skin,
    pub condensed_button_inactive: Skin,
}

impl GuiSkins {
    pub fn new(scale: f32) -> Self {
        let resources = storage::get::<Resources>();

        let panel_01 = resources.images.get("panel_01").cloned().unwrap();

        let editbox_01 = resources.images.get("editbox_01").unwrap();

        let btn_01 = resources.images.get("btn_01").unwrap();
        let btn_01_hover = resources.images.get("btn_01_hover").unwrap();
        let btn_01_click = resources.images.get("btn_01_click").unwrap();
        let btn_01_inactive = resources.images.get("btn_01_inactive").unwrap();

        let blank_image = resources.images.get("blank_image").unwrap();

        // let btn_02 = resources.images.get("btn_02").unwrap();
        // let btn_02_hover = resources.images.get("btn_02_hover").unwrap();
        // let btn_02_click = resources.images.get("btn_02_click").unwrap();
        //
        // let btn_03 = resources.images.get("btn_03").unwrap();
        // let btn_03_hover = resources.images.get("btn_03_hover").unwrap();
        // let btn_03_click = resources.images.get("btn_03_click").unwrap();

        let default = {
            let window_style = root_ui()
                .style_builder()
                .background(panel_01.clone())
                .margin(new_rect_offset(-10.0, -10.0, -10.0, -10.0, scale))
                .background_margin(new_rect_offset(35.0, 35.0, 35.0, 35.0, scale))
                .build();

            let label_style = root_ui()
                .style_builder()
                // .margin(RectOffset::new(15.0, 15.0, 15.0, 15.0))
                // .font(include_bytes!("../../assets/gui/fonts/MinimalPixel v2.ttf"))
                // .unwrap()
                .margin(new_rect_offset(0.0, 0.0, 2.0, 0.0, scale))
                .text_color(Color::from_rgba(255, 255, 160, 255))
                .font_size(get_scaled_font_size(14, scale))
                .build();

            let button_style = root_ui()
                .style_builder()
                .margin(new_rect_offset(8.0, 8.0, 0.0, 0.0, scale))
                .background_margin(new_rect_offset(4.0, 4.0, 4.0, 4.0, scale))
                .background(btn_01.clone())
                .background_hovered(btn_01_hover.clone())
                .background_clicked(btn_01_click.clone())
                // .font(include_bytes!("../../assets/gui/fonts/MinimalPixel v2.ttf"))
                // .unwrap()
                .text_color(Color::from_rgba(200, 200, 160, 255))
                .font_size(get_scaled_font_size(14, scale))
                .build();

            let editbox_style = root_ui()
                .style_builder()
                .background(editbox_01.clone())
                .margin(new_rect_offset(10.0, 10.0, 0.0, 0.0, scale))
                .background_margin(new_rect_offset(4.0, 4.0, 4.0, 4.0, scale))
                .text_color(Color::from_rgba(200, 200, 160, 255))
                .font_size(get_scaled_font_size(14, scale))
                .build();

            let group_style = root_ui()
                .style_builder()
                .margin(new_rect_offset(0.0, 0.0, 0.0, 0.0, scale))
                .color(Color::from_rgba(0, 0, 0, 0))
                .build();

            Skin {
                window_style,
                label_style,
                button_style,
                editbox_style,
                group_style,
                ..root_ui().default_skin()
            }
        };

        let main_menu = {
            let label_style = root_ui()
                .style_builder()
                .margin(new_rect_offset(0.0, 0.0, 5.0, 5.0, scale))
                .text_color(Color::from_rgba(255, 255, 160, 255))
                .font_size(get_scaled_font_size(16, scale))
                .build();

            let button_style = root_ui()
                .style_builder()
                .background(btn_01.clone())
                .margin(new_rect_offset(10.0, 10.0, 0.0, 0.0, scale))
                .background_margin(new_rect_offset(6.0, 6.0, 6.0, 6.0, scale))
                .background_hovered(btn_01_hover.clone())
                .background_clicked(btn_01_click.clone())
                .text_color(Color::from_rgba(200, 200, 160, 255))
                .font_size(get_scaled_font_size(16, scale))
                .build();

            Skin {
                label_style,
                button_style,
                ..default.clone()
            }
        };

        let warning_text = {
            let label_style = root_ui()
                .style_builder()
                .margin(new_rect_offset(0.0, 0.0, 5.0, 5.0, scale))
                .text_color(color::RED)
                .font_size(get_scaled_font_size(14, scale))
                .build();

            Skin {
                label_style,
                ..default.clone()
            }
        };

        let inactive_button = {
            let button_style = root_ui()
                .style_builder()
                .margin(new_rect_offset(8.0, 8.0, 0.0, 0.0, scale))
                .background_margin(new_rect_offset(4.0, 4.0, 4.0, 4.0, scale))
                .background(btn_01_inactive.clone())
                .background_hovered(btn_01_inactive.clone())
                .background_clicked(btn_01_inactive.clone())
                .text_color(Color::from_rgba(200, 200, 160, 255))
                .font_size(get_scaled_font_size(14, scale))
                .build();

            Skin {
                button_style,
                ..default.clone()
            }
        };

        let label_button = {
            let button_style = root_ui()
                .style_builder()
                .background(blank_image.clone())
                .background_hovered(blank_image.clone())
                .background_clicked(blank_image.clone())
                .margin(new_rect_offset(0.0, 0.0, 2.0, 0.0, scale))
                .background_margin(new_rect_offset(0.0, 0.0, 0.0, 0.0, scale))
                .text_color(Color::from_rgba(255, 255, 160, 255))
                .font_size(get_scaled_font_size(14, scale))
                .build();

            Skin {
                button_style,
                ..default.clone()
            }
        };

        let condensed_button = {
            let button_style = root_ui()
                .style_builder()
                .background(btn_01.clone())
                .background_hovered(btn_01_hover.clone())
                .background_clicked(btn_01_click.clone())
                .margin(new_rect_offset(2.0, 2.0, -2.0, -2.0, scale))
                .background_margin(new_rect_offset(4.0, 4.0, 4.0, 4.0, scale))
                .text_color(Color::from_rgba(255, 255, 160, 255))
                .font_size(get_scaled_font_size(14, scale))
                .build();

            Skin {
                button_style,
                ..default.clone()
            }
        };

        let condensed_button_inactive = {
            let button_style = root_ui()
                .style_builder()
                .background(btn_01_inactive.clone())
                .background_hovered(btn_01_inactive.clone())
                .background_clicked(btn_01_inactive.clone())
                .margin(new_rect_offset(2.0, 2.0, -2.0, -2.0, scale))
                .background_margin(new_rect_offset(4.0, 4.0, 4.0, 4.0, scale))
                .text_color(Color::from_rgba(255, 255, 160, 255))
                .font_size(get_scaled_font_size(14, scale))
                .build();

            Skin {
                button_style,
                ..default.clone()
            }
        };

        GuiSkins {
            scale,
            default,
            main_menu,
            warning_text,
            inactive_button,
            label_button,
            condensed_button,
            condensed_button_inactive,
        }
    }
}
