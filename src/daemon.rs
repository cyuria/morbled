use async_std::{
    io::ReadExt,
    os::unix::net::{UnixListener, UnixStream},
    stream::StreamExt,
};
use log::{error, warn};
use message::Message;
use std::{env, error::Error, path::Path, sync::Arc};
use tokio::{sync::Mutex, task::JoinHandle};
use tokio_util::sync::CancellationToken;

mod backlight;

struct BacklightStatus {
    task: JoinHandle<()>,
    token: CancellationToken,
    backlight: Arc<Mutex<backlight::Backlight>>,
}

impl BacklightStatus {
    fn new(backlight: backlight::Backlight) -> Self {
        Self {
            task: tokio::spawn(async {}),
            token: CancellationToken::new(),
            backlight: Arc::new(Mutex::new(backlight)),
        }
    }

    async fn fly(&mut self, new_target: i32) {
        self.token.cancel();
        self.backlight.lock().await.update_target(new_target);

        self.token = CancellationToken::new();
        self.task = tokio::spawn(BacklightStatus::run_down(
            self.backlight.clone(),
            self.token.clone(),
        ));
    }

    async fn run_down(backlight: Arc<Mutex<backlight::Backlight>>, token: CancellationToken) {
        backlight.lock().await.run_down(token).await
    }

    async fn message(&mut self, message: Message) {
        match message {
            Message::SetAbsolute {
                mut amount,
                percent,
            } => {
                self.token.cancel();
                if percent {
                    amount *= self.backlight.lock().await.max;
                    amount /= 100;
                }
                self.fly(amount).await;
            }
            Message::SetRelative {
                mut amount,
                percent,
            } => {
                self.token.cancel();
                if percent {
                    amount *= self.backlight.lock().await.max;
                    amount /= 100;
                }
                let current_target = self.backlight.lock().await.target;
                self.fly(current_target + amount).await;
            }
            Message::None => {}
        }
    }
}

impl Drop for BacklightStatus {
    fn drop(&mut self) {
        self.task.abort();
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    flexi_logger::init();

    let socket = Path::new(env::var("XDG_RUNTIME_DIR")?.as_str()).join("morbled.socket");

    if socket.exists() {
        warn!("Socket already exists");
        std::fs::remove_file(&socket)?;
    }

    let mut devices: Box<[_]> = backlight::get()
        .await?
        .into_iter()
        .map(BacklightStatus::new)
        .collect();

    let listener = UnixListener::bind(socket).await?;

    loop {
        let mut stream = listener.incoming().next().await.unwrap().unwrap();
        if let Err(err) = listen(&mut stream, &mut devices).await {
            error!("{}", err);
        }
    }
}

async fn listen(
    stream: &mut UnixStream,
    devices: &mut [BacklightStatus],
) -> Result<(), Box<dyn Error>> {
    let mut buf = [0u8; ::core::mem::size_of::<Message>()];
    stream.read_exact(&mut buf).await?;

    let msg = Message::deserialize(buf.as_slice().try_into()?);

    for dev in devices {
        dev.message(msg).await;
    }

    Ok(())
}
