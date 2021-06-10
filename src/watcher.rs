use std::io::ErrorKind::NotFound;
use std::os::unix::net::{UnixStream, UnixListener};
use std::io::{BufRead, BufReader};
use log;
use std::thread;
use regex::Regex;
use crate::notifications::notify_touch;

fn handle_client(stream: UnixStream) {
  let reader = BufReader::new(stream);

  let notify_touch_regex = Regex::new(r"^(.*): detected card with S/N (.*)$").unwrap();

  for line in reader.lines() {
    match line {
      Ok(line) => {
        if notify_touch_regex.is_match(&line) {
          log::debug!("Sending notification for '{}'", line);
          notify_touch(&line);
        } else {
          log::trace!("Ignoring line '{}'", line);
        }
      }
      Err(err) =>{
        log::warn!("Error reading from client {:?}", err);
        break;
      }
    }
  }
}

pub fn watch(socket_path: &str) -> std::io::Result<()> {
  log::debug!("Unbinding socket");
  match std::fs::remove_file(socket_path) {
    Ok(_) => {
    }
    Err(err) => {
      if err.kind() != NotFound {
        return Err(err);
      }
    }
  };

  log::info!("Starting watcher");
  let listener = UnixListener::bind(socket_path)?;

  for stream in listener.incoming() {
    match stream {
      Ok(stream) => {
        thread::spawn(|| handle_client(stream));
      }
      Err(err) => {
        log::warn!("Error accepting client {:?}", err);
      }
    }
  }

  Ok(())
}