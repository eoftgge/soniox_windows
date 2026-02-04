use eframe::egui::Color32;

pub(crate) fn get_interim_color(main_color: Color32) -> Color32 {
    if main_color == Color32::WHITE {
        Color32::from_gray(180)
    } else if main_color == Color32::YELLOW {
        Color32::from_rgb(200, 180, 100)
    } else {
        Color32::from_white_alpha(150)
    }
}