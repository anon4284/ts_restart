extern crate ssh2;
extern crate rustc_serialize;

use std::io::prelude::*;
use std::net::{TcpStream};

use self::ssh2::Session;

pub mod data;



pub fn restart_ts(info: &data::Info) {
    let cmd = format!("{}{}", "docker restart ", &info.container_id);
    let url = info.ip.to_owned() + ":22";
    let tcp = TcpStream::connect(&url[..]).unwrap();
    let mut sess = Session::new().unwrap();
    sess.handshake(&tcp).unwrap();
    let auth = data::get_auth();
    sess.userauth_password(&auth.username, &auth.password).unwrap();

    let mut channel = sess.channel_session().unwrap();
    channel.exec(&cmd).unwrap();
    let mut s = String::new();
    channel.read_to_string(&mut s).unwrap();
    println!("{}", s);
    println!("{}", channel.exit_status().unwrap());
}
