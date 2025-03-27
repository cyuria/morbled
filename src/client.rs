use std::{env, error::Error, io::Write, os::unix::net::UnixStream, path::Path};

use log::error;
use message::Message;

fn get_message() -> Result<Message, Box<dyn Error>> {
    let mut percent = false;
    let mut relative = false;
    let mut amount = None;
    let mut direction = 1;

    for arg in env::args().skip(1) {
        match arg.as_str() {
            "-" => {
                relative = true;
                direction = -1;
            }
            "+" => {
                relative = true;
                direction = 1;
            }
            "%" => {
                percent = true;
            }
            arg => amount = Some(arg.parse()?),
        }
    }

    Ok(match amount {
        Some(amount) => match relative {
            true => Message::SetRelative {
                amount: amount * direction,
                percent,
            },
            false => Message::SetAbsolute { amount, percent },
        },
        None => Message::None,
    })
}

fn main() -> Result<(), Box<dyn Error>> {
    let msg = get_message()?;

    let socket = Path::new(env::var("XDG_RUNTIME_DIR")?.as_str()).join("morbled.socket");
    if !socket.exists() {
        error!("Morbled daemon not running");
    }
    let mut stream = UnixStream::connect(socket)?;

    stream.write_all(&msg.serialize())?;

    Ok(())
}
