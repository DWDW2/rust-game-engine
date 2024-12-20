extern "C" {
    fn change_color_of_bg(red: f32, green: f32, blue: f32, alpha: f32);
}

pub fn clear_screen_to_color(red: f32, green: f32, blue: f32, alpha: f32) {
    unsafe {
        change_color_of_bg(red, green, blue, alpha)
    }
}