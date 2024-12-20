fn main(){
  let mut blue_amount:f64 = 0.0;

  rust_engine::set_event_handler(move || {
    blue_amount+=0.1;
    rust_engine::clear_screen_to_color(0.0, 0.0, blue_amount, 1.0);
  })
}