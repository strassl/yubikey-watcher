#[macro_use]
extern crate log;

use clap::{App, Arg};
use yubikey_watcher::watcher::watch;

fn main() {
  env_logger::init();

  let matches = App::new("yubikey-watcher")
    .version("1.0")
    .about("Simple daemon that watches for gpg events which indicate the yubikey is waiting for a touch and posts a notification")
    .arg(
      Arg::with_name("socket")
        .long("socket")
        .help("path to the socket to which the gpg agent log will be written")
        .takes_value(true)
        .required(true),
    )
    .get_matches();

  let socket_path = matches
    .value_of("socket")
    .expect("Invalid socket path");

  watch(socket_path).unwrap();
}
