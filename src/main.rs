use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui,widgets,Skin,};
extern crate ears;
use ears::{Music, Sound, AudioController};

use std::fs;

mod animation;
use crate::animation::*;
mod utils;
use crate::utils::*;
extern crate tini;
use tini::Ini;

use std::process;
// TODO comment code more and refactor better
pub struct Creature {
    pub texture:Texture2D,
    pub textures:Vec<Texture2D>,
}
impl Creature {
    pub fn new(texture:Texture2D, textures:Vec<Texture2D>)-> Creature {
        Creature {
            texture:texture,
            textures:textures,
        }
    }
}
#[macroquad::main("Tamerz Gothca")]
async fn main() {
       // ------------------------------------------ SKIN
       let button_img:&[u8] = include_bytes!("../creatures/button.png");
       let glow_img:&[u8] = include_bytes!("../creatures/glow.png");
       let down_img:&[u8] = include_bytes!("../creatures/down.png");
       
       let skin1 = {
        let label_style = root_ui()
            .style_builder()
            .font_size(18)
            .build();

        let window_style = root_ui()
            .style_builder()
            .background(Image::from_file_with_format(
                include_bytes!("../creatures/bg.png"),
                None,
            ))
            .background_margin(RectOffset::new(20.0, 20.0, 10.0, 10.0))
            .margin(RectOffset::new(0.5, 0.5, 0.0, 0.0))
            .build();

        let widget_style = root_ui()
            .style_builder()
            .background(Image::from_file_with_format(
                button_img,
                None,
            ))
            .margin(RectOffset::new(10.0, 10.0, 0.0, 0.0))
            .background_hovered(Image::from_file_with_format(
                glow_img,
                None,
            ))
            .background_clicked(Image::from_file_with_format(
                down_img,
                None,
            ))
            .text_color(BLACK)
            .font_size(18)
            .build();

        let bar_style = root_ui()
            .style_builder()
            .background(Image::from_file_with_format(
                button_img,
                None,
            ))
            .margin(RectOffset::new(10.0, 10.0, 0.0, 0.0))
            .background_hovered(Image::from_file_with_format(
                glow_img,
                None,
            ))
            .background_clicked(Image::from_file_with_format(
                down_img,
                None,
            ))
            .text_color(BLACK)
            .font_size(18)
            .build();

        let button_style = root_ui()
            .style_builder()
            .background(Image::from_file_with_format(
                button_img,
                None,
            ))
            .margin(RectOffset::new(10.0, 10.0, 0.0, 0.0))
            .background_hovered(Image::from_file_with_format(
                glow_img,
                None,
            ))
            .background_clicked(Image::from_file_with_format(
                down_img,
                None,
            ))
            .text_color(BLACK)
            .font_size(18)
            .build();

        let checkbox_style = root_ui()
            .style_builder()
            .background(Image::from_file_with_format(
                button_img,
                None,
            ))
            .margin(RectOffset::new(10.0, 10.0, 0.0, 0.0))
            .background_hovered(Image::from_file_with_format(
                glow_img,
                None,
            ))
            .background_clicked(Image::from_file_with_format(
                down_img,
                None,
            ))
            .font_size(18)
            .build();

        let group_style = root_ui()
            .style_builder()
            .background(Image::from_file_with_format(
                button_img,
                None,
            ))
            .margin(RectOffset::new(10.0, 10.0, 0.0, 0.0))
            .background_hovered(Image::from_file_with_format(
                glow_img,
                None,
            ))
            .background_clicked(Image::from_file_with_format(
                down_img,
                None,
            ))
            .text_color(BLACK)
            .font_size(18)
            .build();

        Skin {
            label_style:label_style,
            button_style:widget_style,
            window_style:window_style,
            editbox_style:button_style,
            checkbox_style:checkbox_style,
            title_height:32.0,
            group_style:group_style,
            scrollbar_style:bar_style,
            ..root_ui().default_skin()
        }
    };
///////////////////////////////
//                           //
//    ------   _  __ __      //
//      |  \ / |) |_ |_      //
//      |   |  |  |_ _/      //
//                           //
// rock     - earth type     //
// plant    - green type     //
// water    - liquid type    //
// fire     - lava type      //
// electric - lightning type //
// spirit   - holy type      //
// light    - laser type     //
// wind     - tornado type   //
//                           //
///////////////////////////////
    let type_texture:Texture2D = load_texture("creatures/types.png").await.unwrap();
    const CREATURE_TYPES: [&str; 8]  = [ "rock", "plant", "water", "fire", "electric", "spirit", "light", "wind"];

    // file system
    let filename = fs::canonicalize("creatures/game.ini").unwrap().to_str().unwrap().to_owned();
    let asset_dir = "creatures/";

    // Sound
    let mut bg_music = Music::new("creatures/level_0.ogg").unwrap();
    // loop music
    bg_music.set_looping(true);
    // sound effects
    let mut click_sound = Sound::new("creatures/violin.ogg").unwrap();
    //let mut woo_sound = Sound::new("creatures/bells_win.ogg").unwrap();
    //let mut special_sound = Sound::new("creatures/trumpet.ogg").unwrap();
    let mut coin_sound = Sound::new("creatures/bells.ogg").unwrap();

// creature list
    const LIST: [&str; 6] = [ "green_guy", "topsy_turvy", "rodie", "pinkie", "frogger", "viola" ];
    let mut textures:Vec<Texture2D> = vec![];

//rewards
    let rewards:Texture2D = load_texture("creatures/rewards.png").await.unwrap();

// faces
    let skin:Texture2D = load_texture("creatures/skins.png").await.unwrap();
    let hair:Texture2D = load_texture("creatures/hair.png").await.unwrap();
    let eyes:Texture2D = load_texture("creatures/eyes.png").await.unwrap();
    let expression:Texture2D = load_texture("creatures/expressions.png").await.unwrap();
    //for saving
    let person_name = "default_person";
// spinners
    let spinner1:Texture2D = load_texture("creatures/spinning_disc.png").await.unwrap();
    let spinner2:Texture2D = load_texture("creatures/backwards_spinning_disc.png").await.unwrap();
    let spinner3:Texture2D = load_texture("creatures/backwards_spinning_disc_pink.png").await.unwrap();
    //let spinner4:Texture2D = load_texture("creatures/.png").await.unwrap();

    // ini file items
    let mut w:f32;
    let mut h:f32;
    let mut xp:f32;
    let mut mp:f32;
    let mut hp:f32;
    let mut level:f32;
    let mut special_name:String;
    // face choice holders
    let mut hair_choice;
    let mut skin_choice;
    let mut eye_choice;
    let mut expression_choice;
    // ui alignment / sizing
    let button_x:f32 = 10.0;
    let column_x:f32 = 100.0;
    let y_inc:f32 = 30.0;
    let view_w:f32 = 350.0;
    let view_h:f32 = 200.0;
    let sprite_w:f32 = 64.0;
    let sprite_h:f32 = 64.0;
    let spacer:f32 = 5.0;
    let item_spacer:f32 = 10.0;
    let big_spacer:f32 = 20.0;
    const GUAGE_W:f32 = 70.0;
    let creature_x:f32 = view_w + big_spacer;
    let creature_y:f32 = big_spacer;
    let guage_x:f32 = creature_x + sprite_w + big_spacer;
    const GUAGE_H:f32 = 20.0;
    let row2:f32 = view_h + (2.0 * item_spacer);
    // image frame
    let mut frame = Frame{x:0.0,y:0.0};
    let mut type_frame = Frame{x:0.0,y:0.0};
    // spinner 'frame' x coordinate
    let mut spinner_frame_x:f32 = 0.0;
    let mut spinner_frame_x_2:f32 = 0.0;
    let spinner_max:f32 = (sprite_w * 7.0).round(); // spinner animation frames
    let mut reward_frame_x:f32 = 0.0;
    let reward_max:f32 = (sprite_w * 14.0).round(); // reward changer frames

    // structs
    let my_controller = Controller::new();
    let mut data:Data = Data::new();

    // read the file
    let test_ini = Ini::from_file(&filename);
    if test_ini.is_err() {
        println!("ERROR!!! {:?} in {:?}",test_ini,filename);
        process::exit(0x0100);
    }
    let conf = test_ini.unwrap();
    let save_ini_string = conf.to_string().to_owned();
    

    // read the config for each item
    for item in LIST.iter() {
        let image:String = conf.get(item, "image").unwrap_or("".to_string());
        let mut image_file:String = asset_dir.to_string();
        image_file.push_str(image.as_str());
        //println!("Image file:{}",image_file);
        textures.push(load_texture(image_file.as_str()).await.unwrap());
    }

    // default creature image
    let texture:Texture2D = textures[0].to_owned();
    let mut creature = Creature::new(texture, textures);
    let mut section = LIST[0];
    // get the strings from the config file
    let w_s:String = conf.get(section, "w").unwrap_or("64.0".to_string());
    let h_s:String = conf.get(section, "h").unwrap_or("64.0".to_string());
    let xp_s:String = conf.get(section, "xp").unwrap_or("0.0".to_string());
    let mp_s:String = conf.get(section, "mp").unwrap_or("5.0".to_string());
    let hp_s:String = conf.get(section, "hp").unwrap_or("15.0".to_string());
    let level_s:String = conf.get(section, "level").unwrap_or("1".to_string());
    let c_type:String = conf.get(section, "creature_type").unwrap_or("rock".to_string());
    special_name = conf.get(section, "special").unwrap_or("Glow In The Dark".to_string());
    let hair_s:String  = conf.get(person_name, "hair").unwrap_or("0.0".to_string());
    let skin_s:String  = conf.get(person_name, "skin").unwrap_or("0.0".to_string());
    let eye_s:String  = conf.get(person_name, "eye").unwrap_or("0.0".to_string());
    let expr_s:String  = conf.get(person_name, "expression").unwrap_or("0.0".to_string());
    let r:String = conf.get(person_name, "r").unwrap_or("1.0".to_string());
    let g:String = conf.get(person_name, "g").unwrap_or("1.0".to_string());
    let b:String = conf.get(person_name, "b").unwrap_or("1.0".to_string());
        //COLOR
    let mut skin_color:Color = Color{
        r:r.parse::<f32>().ok().unwrap(),
        g:g.parse::<f32>().ok().unwrap(),
        b:b.parse::<f32>().ok().unwrap(),
        a:1.0,
    };

    // TODO read file for image location to buil textures
     //let dir:String = conf.get(section, "directory").unwrap_or("creature".to_string());
    let mut type_iter:f32 =  0.0;

    // find the 'type' texture frame
    for creature_type in CREATURE_TYPES.iter() {
        if c_type == creature_type.to_string() {
            type_frame.x = sprite_w * type_iter;
        }
        type_iter += 1.0;
    }
    // set our stat variables
    hp = hp_s.parse::<f32>().ok().unwrap();
    mp = mp_s.parse::<f32>().ok().unwrap();
    xp = xp_s.parse::<f32>().ok().unwrap();
    w = w_s.parse::<f32>().ok().unwrap();
    h = h_s.parse::<f32>().ok().unwrap();
    level = level_s.parse::<f32>().ok().unwrap();
    hair_choice = hair_s.parse::<f32>().ok().unwrap();
    skin_choice = skin_s.parse::<f32>().ok().unwrap();
    eye_choice = eye_s.parse::<f32>().ok().unwrap();
    expression_choice = expr_s.parse::<f32>().ok().unwrap();


    bg_music.play();

// GAME LOOP +++++++++++++++++++++++++++++
    loop {
       // bg color
       clear_background(LIME);

       // skin it!
       root_ui().push_skin(&skin1);
       let mut item_y:f32 = item_spacer;
       // make UI
       // check for user input to set sprite frame coordinates
       let old_frame = frame.x;
       if is_key_pressed(my_controller.up) {
           frame.x = 3.0 * (sprite_h);
           data.credits += 1.0;
           if old_frame == frame.x {frame.x = 0.0;}
       } else if is_key_down(my_controller.right) {
           frame.x = 2.0 * (sprite_h);
           data.credits += 1.0;
       } else if is_key_down(my_controller.left) {
           frame.x = sprite_h;
           data.credits += 1.0;
       } else if is_key_down(my_controller.down) {
           frame.x = 0.0;
           data.credits += 1.0;
       } else if is_key_down(my_controller.attack) {
           frame.x = 5.0 * (sprite_h);
           if old_frame == frame.x {frame.x = 0.0;}
           data.credits += 1.0;
           data.clicks += 1.0;
       } else if is_key_down(my_controller.special) {
           frame.x = 4.0 * (sprite_h);
           if old_frame == frame.x {frame.x = 0.0;}
           data.credits += 1.0;
           data.ids += 1.0;
       }
       let third_column = GUAGE_W + guage_x + item_spacer;
       //let next_row = view_h + item_spacer;


       //draw the creature
       draw_texture_ex(creature.texture,
                             creature_x,
                             creature_y,
                             WHITE,
                             DrawTextureParams{
                               source:Some(Rect::new(frame.x, frame.y, w, h)),
                               ..Default::default()
                             },);


// ------------------------------------------ Guages
       // set up variables
       let lvl = level;
       let xpm = lvl * 10.0;
       let clicks = data.clicks;
       let credits = data.credits;
       let ids = data.ids;

       // big box under image
       let surprise_y:f32 = creature_y + sprite_h + (2.0 * item_spacer);
       let surprise_box = FlatBox::new(creature_x, surprise_y, GUAGE_W, GUAGE_W, BLACK);
        surprise_box.draw();
        
        let mut lvl_check = 10.0;
        // harder as you progress
        if lvl >= 50.0 {
            lvl_check = lvl_check * 10.0;
            if lvl >= 100.0 {lvl_check = lvl_check * 10.0;}
        }
        
        let mut next_y = 10.0;
        let guage_spacer = GUAGE_H + item_spacer;
        // LVL Guage
        guage(get_value(lvl, lvl_check, GUAGE_W), guage_x, next_y, GUAGE_W, GUAGE_H, spacer, YELLOW, BLUE);
        // update next Y
        next_y += guage_spacer;
        // XP GUAGE
        guage(get_value(xp, xpm, GUAGE_W), guage_x, next_y, GUAGE_W, GUAGE_H, spacer, YELLOW, pick_color(xp));
        next_y += guage_spacer;
        //HP Guage
        guage(get_value(clicks, hp, GUAGE_W), guage_x, next_y, GUAGE_W, GUAGE_H, spacer, YELLOW, pick_color(clicks));
        next_y += guage_spacer;
        //MP Guage
        guage(get_value(data.ids, mp, GUAGE_W), guage_x, next_y, GUAGE_W, GUAGE_H, spacer, YELLOW, small_color(ids));
        next_y += guage_spacer;
        // fun color guages
        let hp_box = FlatBox::new(guage_x, next_y, GUAGE_W, GUAGE_H, pick_color(hp));
        hp_box.draw();
        next_y += guage_spacer;
        let extra_box = FlatBox::new(guage_x, next_y, GUAGE_W, GUAGE_H, pick_color(mp));
        extra_box.draw();
        next_y += guage_spacer;
        let win_box = FlatBox::new(guage_x, next_y, GUAGE_W, GUAGE_H, small_color(xp));
        win_box.draw();

// ------------------------------------------Creature Chooser window
        let chooser_width = big_spacer * 8.0;
        widgets::Window::new(hash!(), vec2(third_column, item_spacer), vec2(chooser_width, view_h)).titlebar(false).ui(&mut *root_ui(), |ui| {
                        let mut iter:usize = 0;
                        let mut this_next_y = item_spacer;
                        for item in LIST.iter() {
                            if ui.button(Vec2::new(button_x, this_next_y), item) {
                                section = LIST[iter];
                                creature.texture = creature.textures[iter].to_owned();
                                let w_s:String = conf.get(section, "w").unwrap_or("64.0".to_string());
                                let h_s:String = conf.get(section, "h").unwrap_or("64.0".to_string());
                                let xp_s:String = conf.get(section, "xp").unwrap_or("0.0".to_string());
                                let mp_s:String = conf.get(section, "mp").unwrap_or("5.0".to_string());
                                let hp_s:String = conf.get(section, "hp").unwrap_or("15.0".to_string());
                                let level_s:String = conf.get(section, "level").unwrap_or("1".to_string());
                                let c_type:String = conf.get(section, "creature_type").unwrap_or("rock".to_string());
                                special_name = conf.get(section, "special").unwrap_or("Glow In The Dark".to_string());
                                let mut type_iter:f32 =  0.0;
                                for creature_type in CREATURE_TYPES.iter() {
                                    if c_type == creature_type.to_string() {
                                        type_frame.x = sprite_w * type_iter;
                                    }
                                    type_iter += 1.0;
                                }
                                hp = hp_s.parse::<f32>().ok().unwrap();
                                mp = mp_s.parse::<f32>().ok().unwrap();
                                xp = xp_s.parse::<f32>().ok().unwrap();
                                w = w_s.parse::<f32>().ok().unwrap();
                                h = h_s.parse::<f32>().ok().unwrap();
                                level = level_s.parse::<f32>().ok().unwrap();
                                data.credits = 0.0;
                                data.clicks = 0.0;
                                data.ids = 0.0;
                                frame.x = 0.0;
                            }
                            this_next_y += y_inc;
                            iter += 1;
                        }

                    });
       next_y = row2;
       //draw the type icon
       draw_texture_ex(type_texture,
                             item_spacer,
                             next_y,
                             WHITE,
                             DrawTextureParams{
                               source:Some(Rect::new(type_frame.x, type_frame.y, w, h)),
                               ..Default::default()
                             },);
       
// ------------------------------------------LEVELING up window
       widgets::Window::new(hash!(), vec2(item_spacer, item_spacer), vec2(view_w, view_h)).titlebar(false)
            .ui(&mut *root_ui(), |ui| {
                ui.label(Vec2::new(column_x, item_y), &format!("LEVEL: {}",level));
                // increment the y drawing point!
                item_y = item_y + y_inc;
                ui.label(Vec2::new(column_x, item_y), &format!("XP: {}",xp));
                ui.label(Vec2::new(column_x, item_y + item_spacer), &format!("XP UP: {}",data.credits));
                if ui.button(Vec2::new(button_x, item_y), "Earn XP") {
                   data.credits += 1.0;
                   click_sound.play();
                }
                // increment the y drawing point!
                item_y = item_y + y_inc;
                ui.label(Vec2::new(column_x, item_y), &format!("HP: {}",hp));
                ui.label(Vec2::new(column_x, item_y + item_spacer), &format!("HP UP: {}",data.clicks));
                if ui.button(Vec2::new(button_x, item_y), "Earn HP") {
                   data.clicks += 1.0;
                   click_sound.play();

                }
                // increment the y drawing point!
                item_y = item_y + y_inc;
                ui.label(Vec2::new(column_x, item_y), &format!("MP: {}",mp));
                ui.label(Vec2::new(column_x, item_y + item_spacer), &format!("MP UP: {}",data.ids));
                if ui.button(Vec2::new(button_x, item_y), "Earn MP") {
                   click_sound.play();
                   data.ids += 1.0;
                }
                item_y = item_y + y_inc;
                ui.label(Vec2::new(column_x, item_y), &format!("Special Move: {:?}",special_name));
            });
// ------------------------------------------ Person Config Window
       let config_win_x = sprite_w + (2.0 * item_spacer);

       widgets::Window::new(hash!(), vec2(config_win_x, row2), vec2(view_w, sprite_h * 2.0 )).titlebar(false).ui(&mut *root_ui(), |ui| {
            // skin
            let old_skin = skin_choice.round();
            ui.slider(hash!(), "Skin", 0.0f32..5.0f32, &mut skin_choice);
            //save skin
            if  old_skin != skin_choice.round() {
                let save_ini = Ini::from_string(save_ini_string.to_owned()).unwrap();
                check_result(save_ini.section(person_name).item("skin",skin_choice).to_file(&filename));
            }
            //expression
            let old_expr = expression_choice.round();
            ui.slider(hash!(), "Expression", 0.0f32..5.0f32, &mut expression_choice);
            if  old_expr != expression_choice.round() {
               //save skin
               let save_ini = Ini::from_string(save_ini_string.to_owned()).unwrap();
               check_result(save_ini.section(person_name).item("expression",expression_choice).to_file(&filename));
            }

            //eyes
            let old_eye = eye_choice.round();
            ui.slider(hash!(), "Eye", 0.0f32..5.0f32, &mut eye_choice);
            if  old_eye != eye_choice.round() {
               //save skin
               let save_ini = Ini::from_string(save_ini_string.to_owned()).unwrap();
               check_result(save_ini.section(person_name).item("eye",eye_choice).to_file(&filename));
           }

            //hair
            let old_hair = hair_choice.round();
            ui.slider(hash!(), "Hair", 0.0f32..5.0f32, &mut hair_choice);
            if  old_hair != hair_choice.round() {
               //save skin
               let save_ini = Ini::from_string(save_ini_string.to_owned()).unwrap();
               check_result(save_ini.section(person_name).item("hair",hair_choice).to_file(&filename));
           }

       });

       let color_win_x:f32 = config_win_x +view_w + spacer;
       let old_color:Color = skin_color;
       widgets::Window::new(hash!(), vec2(color_win_x , row2), vec2(view_w, sprite_h * 2.0 )).titlebar(false).ui(&mut *root_ui(), |ui| {
           ui.slider(hash!(), "Red", 0.0f32..1.0f32, &mut skin_color.r);
           ui.slider(hash!(), "Blue", 0.0f32..1.0f32, &mut skin_color.b);
           ui.slider(hash!(), "Green", 0.0f32..1.0f32, &mut skin_color.g);
       });
       if old_color != skin_color {
           let save_ini = Ini::from_string(save_ini_string.to_owned()).unwrap();
               check_result(save_ini.section(person_name).item("r",skin_color.r).item("b",skin_color.b).item("g",skin_color.g).to_file(&filename));
       }
       // draw the face
       next_y += sprite_h + item_spacer;
       //skin
       draw_texture_ex(skin, item_spacer, next_y, skin_color,
                             DrawTextureParams{
                               source:Some(Rect::new(get_frame_x(w, skin_choice), 0.0, w, h)),
                               ..Default::default()
                             },);
       //expression
       draw_texture_ex(expression, item_spacer, next_y, WHITE,
                             DrawTextureParams{
                               source:Some(Rect::new(get_frame_x(w, expression_choice), 0.0, w, h)),
                               ..Default::default()
                             },);
      //eyes
      draw_texture_ex(eyes, item_spacer, next_y, WHITE,
                             DrawTextureParams{
                               source:Some(Rect::new(get_frame_x(w, eye_choice), 0.0, w, h)),
                               ..Default::default()
                             },);
      //hair
      draw_texture_ex(hair, item_spacer, next_y,WHITE,
                             DrawTextureParams{
                               source:Some(Rect::new(get_frame_x(w, hair_choice), 0.0, w, h)),
                               ..Default::default()
                     },);
        // GAME LOGIC
        if xp >= xpm && xpm > 0.0 {
            level = lvl + 1.0;
            coin_sound.play();
            xp = 0.0;
            reward_frame_x += sprite_w;
            // save new LEVEL
            let save_ini = Ini::from_string(save_ini_string.to_owned()).unwrap();
            check_result(save_ini.section(section).item("level",level).to_file(&filename));
        }

        if clicks > hp && hp > 0.0  {
            hp = hp + 1.0;
            data.clicks = 0.0;
            coin_sound.play();
            spinner_frame_x += sprite_w;
            // save new HP
            let save_ini = Ini::from_string(save_ini_string.to_owned()).unwrap();
            check_result(save_ini.section(section).item("hp",hp).to_file(&filename));
        }
        if credits >= xpm && xpm > 0.0  {
           data.credits = 0.0;
           coin_sound.play();
           xp += 1.0;
       }
       if ids >= mp && mp > 0.0  {
           data.ids = 0.0;
           coin_sound.play();
           spinner_frame_x_2 += sprite_w;
           mp += 1.0;
           // save new MP
           let save_ini = Ini::from_string(save_ini_string.to_owned()).unwrap();
           check_result(save_ini.section(section).item("mp",mp).to_file(&filename));
       }
//------------------------------Spinners
        // reset spinner if needed
        if spinner_frame_x >= spinner_max {spinner_frame_x = 0.0;}
        if spinner_frame_x < 0.0 {spinner_frame_x = 0.0;}
        if spinner_frame_x_2 >= spinner_max {spinner_frame_x_2 = 0.0;}
        if spinner_frame_x_2 < 0.0 {spinner_frame_x_2 = 0.0;}
        if reward_frame_x >= reward_max {reward_frame_x = 0.0;}
        if reward_frame_x < 0.0 {reward_frame_x = 0.0;}
        next_y += sprite_h + item_spacer;
        // set the initial x coordinate all "mathy mathy" here
        let mut spinner_x = item_spacer;
        //spinner1
        draw_texture_ex(spinner2, spinner_x, next_y, YELLOW, DrawTextureParams{source:Some(Rect::new(spinner_frame_x, 0.0, w, h)),..Default::default()},);
        spinner_x += item_spacer + sprite_w;
        //spinner2
        draw_texture_ex(spinner3, spinner_x, next_y, BLUE, DrawTextureParams{source:Some(Rect::new(spinner_frame_x_2, 0.0, w, h)),..Default::default()},);
        spinner_x += item_spacer + sprite_w;
        //spinner3
        draw_texture_ex(spinner3, spinner_x, next_y, WHITE, DrawTextureParams{source:Some(Rect::new(spinner_frame_x_2, 0.0, w, h)),..Default::default()},);
        spinner_x += item_spacer + sprite_w;
        //spinner4
        draw_texture_ex(spinner1, spinner_x, next_y, GREEN, DrawTextureParams{source:Some(Rect::new(spinner_frame_x, 0.0, w, h)),..Default::default()},);
        // rewards texture
        draw_texture_ex(rewards, creature_x, surprise_y, pick_color(hp), DrawTextureParams{source:Some(Rect::new(reward_frame_x, 0.0, w, h)),..Default::default()},);

       // not sure why we need to push/pop theme each time...
       root_ui().pop_skin();
       next_frame().await;
    }
}
