extern crate tini;
use std::io::Error as Error;
use macroquad::prelude::*;
/// wrapper around Keycodes/Config storage
pub struct Controller {
	pub left:KeyCode,
	pub right:KeyCode,
	pub up:KeyCode,
	pub down:KeyCode,
	pub attack:KeyCode,
	pub special:KeyCode,
}
impl Controller {
     pub fn new()->Self where Self:Sized {
        Controller {
            left:KeyCode::Left,
            right:KeyCode::Right,
            up:KeyCode::Up,
            down:KeyCode::Down,
            attack:KeyCode::Enter,
            special:KeyCode::Space,
        }
    }
}
pub struct Data {
    pub clicks:f32,
    pub credits:f32,
    pub ids:f32,
}
impl Data {
    pub fn new()->Self where Self:Sized {
        Data {
            clicks: 0.0,
            credits: 0.0,
            ids:0.0,
            }
    }
}
pub struct FlatBox {
    x:f32,
    y:f32,
    w:f32,
    h:f32,
    color:Color,
}
impl FlatBox {
    pub fn new(x:f32, y:f32, w:f32, h:f32, color:Color)->Self where Self:Sized {
        draw_rectangle(x, y, w, h, color);
        FlatBox {x:x, y:y, w:w, h:h, color:color,}
    }
    pub fn draw(&self){draw_rectangle(self.x, self.y, self.w, self.h, self.color);}
}

pub fn check_result(res:Result<(), Error>) {
    if res.is_err() {
       println!("Error:{:?}",res.err());
    }
}

pub fn small_color(value:f32)->Color {
    if value <= 2.0 { return BLUE }
    else if value <= 4.0 { return GOLD }
    else if value <= 6.0 { return GREEN }
    else if value <= 8.0 { return VIOLET }
    else if value <= 12.0 { return MAGENTA }
    else if value <= 18.0 { return MAROON }
    else if value <= 24.0 { return ORANGE }
    else if value <= 28.0 { return PINK }
    else if value <= 36.0 { return PURPLE }
    else if value <= 40.0 { return RED }
    else if value <= 48.0 { return SKYBLUE }
    else if value <= 58.0 { return LIME }
    else if value <= 70.0 { return YELLOW }
    // WOAH that is too much!!
    BLACK
}
pub fn pick_color(value:f32)->Color {
    if value <= 5.0{ return ORANGE }
    else if value <= 9.0 { return GREEN }
    else if value <= 14.0 { return BLUE }
    else if value <= 19.0 { return MAROON }
    else if value <= 24.0 { return GOLD }
    else if value <= 29.0 { return GRAY }
    else if value <= 34.0 { return BEIGE }
    else if value <= 39.0 { return LIGHTGRAY }
    else if value <= 44.0 { return LIME }
    else if value <= 49.0 { return MAGENTA }
    else if value <= 54.0 { return BROWN }
    else if value <= 59.0 { return WHITE }
    else if value <= 64.0 { return PINK }
    else if value <= 69.0 { return PURPLE }
    else if value <= 74.0 { return RED }
    else if value <= 79.0 { return SKYBLUE }
    else if value <= 84.0 { return VIOLET }
    else if value <= 89.0 { return YELLOW }
    else if value <= 94.0 { return DARKBLUE }
    else if value <= 99.0 { return DARKBROWN }
    else if value <= 104.0 { return DARKGRAY }
    else if value <= 109.0 { return DARKGREEN }
    // WOAH that is too much!!
    DARKPURPLE
}
// make a guage percentage
pub fn get_value(current:f32, max:f32, width:f32)->f32 {
    let unit = max / 100.00;
    let percent = current / unit;
    let ret_unit = width / 100.00;
    ret_unit * percent
}


pub fn guage(value:f32, x:f32, y:f32, w:f32, h:f32, spacer:f32, bg:Color, fg:Color) {
    draw_rectangle(x, y, w, h, bg);
    let mut val = value;
    if value > (spacer * 2.0)
    {
       val = value - (spacer * 2.0)
    }
    // width = 100%
    draw_rectangle(x + spacer, y + spacer, val, h - (2.0 * spacer), fg);
}

// non-fancy function to get the next x 'frame' 
pub fn get_frame_x(frame_width:f32, which_frame:f32)->f32 {return frame_width * which_frame.round()}
