use std::str::FromStr;

use device_query::{Keycode, DeviceState, DeviceQuery};

pub fn get_keys_pressed() -> Vec<Keycode>{
    DeviceState::new().get_keys()
}

pub fn string_list_to_keycode_list(strings: Vec<String>) -> Vec<Keycode>{
    let mut temp: Vec<Keycode> = vec![];
    for i in strings{
        temp.push(Keycode::from_str(i.as_str()).unwrap());
    }
    temp
}

pub fn get_matching_keycodes(list: Vec<Keycode>, other: &Vec<Keycode>) -> Vec<Keycode>{
    list.into_iter().filter(|x| other.contains(x)).collect::<Vec<Keycode>>()
}