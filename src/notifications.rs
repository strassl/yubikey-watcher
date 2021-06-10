use notify_rust::Notification;
use log;

pub fn notify_touch(message: &str) {
  let result = Notification::new()
    .summary("YubiKey Touch Required")
    .body(message)
    .show();

    match result {
      Ok(_) => {
      }
      Err(err) => {
        log::warn!("Error posting notification: {:?}", err)
      }
    }
}