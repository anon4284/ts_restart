extern crate rustc_serialize;

use std::io::prelude::*;
use std::fs::File;

use self::rustc_serialize::json::Json;



pub struct Info {
    pub ip: String,
    pub container_id: String
}

pub struct Auth {
    pub username: String,
    pub password: String
}

pub fn get_auth() -> Auth {
    let mut f = File::open("auth.json").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
    let json = Json::from_str(&s).unwrap();
    let obj = json.as_object().unwrap();
    let username = obj.get("username").unwrap().as_string().unwrap();
    let password = obj.get("password").unwrap().as_string().unwrap();
    return Auth {username: username.to_owned(), password: password.to_owned()};
}

pub fn get_info() -> Info {
    let mut f = File::open("info.json").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
    let json = Json::from_str(&s).unwrap();
    let obj = json.as_object().unwrap();
    let ip = obj.get("ip").unwrap().as_string().unwrap();
    let container_id = obj.get("container_id").unwrap().as_string().unwrap();
    return Info {ip: ip.to_owned(), container_id: container_id.to_owned()};
}
