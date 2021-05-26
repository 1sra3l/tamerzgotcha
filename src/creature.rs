use crate::animation::*;
use macroquad::prelude::*;
extern crate tini;
use tini::Ini;

#[derive (Clone, Debug)]
/// Our generic creature struct
pub struct Creature {
	/// name
	pub name:String,
	/// width
	pub w:f32,
	/// height
	pub h:f32,
	/// Health Points
	pub hp:f32,
	/// Mana Points
	pub mp:f32,
	/// Experience Points
	pub xp:f32,
	/// Max Heath Points
	pub hp_max:f32,
	/// Max Mana Points
	pub mp_max:f32,
	/// Current Level
	pub level:f32,
	/// The creature type
	pub creature_type:String,
	/// the image name
	pub image:String,
	/// The list of skins available
	pub texture:Texture2D,
	/// the frame to use (for animation)
	pub frame:Frame,
	/// the save file
	pub save_file:String,
}
impl Creature {
	pub fn new(w:f32, h:f32)->Creature {
		Creature {
			name:"default".to_string(),
			w:w,
			h:h,
			hp:15.0,
			mp:5.0,
			xp:0.0,
			hp_max:15.0,
			mp_max:5.0,
			level:1.0,
			creature_type:"rock".to_string(),
			image:"".to_string(),
			texture:Texture2D::empty(),
			frame:Frame{x:0.0,y:0.0},
			save_file:"default.ini".to_string(),
			
		}
	}
	pub fn save(self) {
		let mut conf = Ini::new().section(self.name).items(vec![
		                                                        ("w",self.w),
		                                                        ("h",self.h),
		                                                        ("hp",self.hp),
		                                                        ("mp",self.mp),
		                                                        ("xp",self.xp),
		                                                        ("level",self.level)
		                                                       ]);
	}
}

pub struct Controller {
	pub left:KeyCode,
	pub right:KeyCode,
	pub up:KeyCode,
	pub down:KeyCode,
	pub attack:KeyCode,
	pub special:KeyCode,
}
