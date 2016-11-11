extern crate curl;
extern crate rustc_serialize;

use curl::easy::Easy;
use self::rustc_serialize::json::Json;

use std::time::Duration;
use std::thread::sleep;

use std::str;

mod ssh;

static API_STATUS: &'static str = "https://api.planetteamspeak.com/serverstatus/";

fn main() {
    let info = ssh::data::get_info();
     loop {
         init_request(&info);
         sleep(Duration::new(3000, 0));
     }
}

fn init_request(info: &ssh::data::Info) {
    let mut dst = Vec::new();
    let url = format!("{}{}", API_STATUS, info.ip);
    let mut easy = Easy::new();
    easy.url(&url).unwrap();
    let mut transfer = easy.transfer();
    transfer.write_function(|data| {
        dst.extend_from_slice(data);
        if !is_online(&dst) {
            ssh::restart_ts(info);
        }
        Ok(data.len())
    }).unwrap();
    transfer.perform().unwrap();
}

fn is_online(data: &Vec<u8>) -> bool {
    let s = str::from_utf8(data.as_slice()).unwrap();
    let json = Json::from_str(&s[..]).unwrap();
    let obj = json.as_object().unwrap();
    let status = obj.get("status").unwrap();

    if status.as_string().unwrap() != "success" {
        return false;
    }
    return true;
}
