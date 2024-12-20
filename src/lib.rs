thread_local! {
    pub static EVENT_HANDLER: std::cell::RefCell<Box<dyn FnMut()>> = std::cell::RefCell::new(Box::new(||{}));
}

extern "C" {
    fn change_color_of_bg(red: f32, green: f32, blue: f32, alpha: f32);
}
#[no_mangle]
pub extern "C" fn key_pressed(){
    EVENT_HANDLER.with(|event_handler: &std::cell::RefCell<Box<dyn FnMut()>>| (event_handler.borrow_mut())())
}

pub fn set_event_handler(function:impl FnMut() + 'static ){
    EVENT_HANDLER.with(|event_handler: &std::cell::RefCell<Box<dyn FnMut()>> | {
        *event_handler.borrow_mut() = Box::new(function);
    })   
}
pub fn clear_screen_to_color(red: f32, green: f32, blue: f32, alpha: f32) {
    unsafe {
        change_color_of_bg(red, green, blue, alpha)
    }
}