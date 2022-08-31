use dual_shock4_controller::gamepad::{Button, GamePad};
use dual_shock4_controller::joystick::{DeviceInfo, Joystick};
use enigo::*;
use std::borrow::Borrow;
use std::ops::Deref;
use std::{fs, thread};
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use egui::Context;
use eframe::egui;
use hex;

fn balance(mut number: i32) -> i32 {
    if number < 110 {
        number = -127 + number;
    } else if number > 140 {
        number = number - 127;
    } else {
        number = 0;
    }

    return number;
}

fn make_user_directory_and_config_file_and_get_divide_number(mut controller_mouse_path: String) -> i32
{
    let dir_path = Path::new(&controller_mouse_path);

    //check if the directory even exists, if not make it.
    if !dir_path.is_dir()
    {
        fs::create_dir(&dir_path).unwrap();
    }

    let config_file_path: String = controller_mouse_path+"/config.txt";

    match File::open(config_file_path.clone()) {
        Ok(ref _file) => { },
        Err(error) => {
            //the config file doesn't exist make it.
            File::create(config_file_path.clone()).expect("Could not create config file.");
            fs::write(config_file_path.clone(), "20\n0x054c\n0x09cc").expect("Could not write initial config file settings.");
        },
    };



    return get_config(0).parse::<i32>().unwrap();
}
fn handle_controller_input()
{
    let mut enigo: Enigo = Enigo::new();
    let mut count: i32 = 0;

    loop {
        let mut divide_number = make_user_directory_and_config_file_and_get_divide_number(home::home_dir().unwrap().to_str().unwrap().to_owned() + "/.controller_mouse");
        let can_click_button: bool = count % 10 == 0;
        let gamepad = get_gamepad();

        let mut x = gamepad.stick.left_stick.x as i32;
        let mut y = gamepad.stick.left_stick.y as i32;
        let mut y_scroll = gamepad.stick.right_stick.y as i32;
        let mut x_scroll = gamepad.stick.right_stick.x as i32;

        if gamepad.r2_button.pressed {
            divide_number = divide_number / 2;
        }
        if gamepad.l2_button.pressed {
            divide_number = divide_number * 4;
        }

        x = balance(x) / divide_number;
        y = balance(y) / divide_number;
        y_scroll = balance(y_scroll) / divide_number;
        x_scroll = balance(x_scroll) / divide_number;


        enigo.mouse_move_relative(x, y);


        if gamepad.square_button.pressed && can_click_button {
            enigo.mouse_click(MouseButton::Left);
        }

        if gamepad.square_button.pressed && can_click_button {
            enigo.mouse_click(MouseButton::Left);
        }

        if gamepad.square_button.pressed && can_click_button {
            enigo.mouse_down(MouseButton::Left);
        }

        if gamepad.triangle_button.pressed && can_click_button {
            enigo.mouse_up(MouseButton::Left);
        }

        if gamepad.o_button.pressed  && can_click_button{
            enigo.mouse_click(MouseButton::Right);
        }

        if gamepad.x_button.pressed && can_click_button {
            enigo.mouse_click(MouseButton::Left);
        }

        if gamepad.down_button.pressed && can_click_button {
            enigo.key_click(Key::Space);
        }

        if gamepad.right_button.pressed && can_click_button {
            enigo.key_click(Key::Return);
        }

        if gamepad.right_stick_button.pressed  && can_click_button{
            enigo.mouse_down(MouseButton::Middle);
        }

        if gamepad.left_stick_button.pressed && can_click_button {
            enigo.mouse_up(MouseButton::Middle);
        }

        enigo.mouse_scroll_y(y_scroll as i32);
        enigo.mouse_scroll_x(x_scroll as i32);


        count = count + 1;
    }
}

fn main() {
    handle_controller_input();
}

fn get_config(index: usize) -> String
{

    return fs::read_to_string(home::home_dir().unwrap().to_str().unwrap().to_owned() + "/.controller_mouse/config.txt")
        .unwrap()
        .as_str()
        .split("\n")
        .collect::<Vec<&str>>()
        .get(index)
        .unwrap()
        .to_string();
}

fn get_gamepad() -> GamePad {
    let vid = u16::from_str_radix(get_config(1).as_str().trim_start_matches("0x"), 16).unwrap();
    let pid = u16::from_str_radix(get_config(2).as_str().trim_start_matches("0x"), 16).unwrap();

    let joystick = Joystick::new();
    let device_info = DeviceInfo {
        vid: vid,
        pid: pid,
    }; // vendor id, product id HID\VID_054C&PID_05C4\7&3869AC07&0&0000
    let device = joystick.connect(device_info).expect("can't find device!");

    let mut buf = [0u8; 64];
    device.read_timeout(&mut buf[..], 1000).unwrap();

    return joystick.get_gamepad().get_state(&buf);
}
