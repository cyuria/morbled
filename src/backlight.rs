use log::error;
use std::{
    error::Error,
    path::{Path, PathBuf},
    time::Duration,
};
use tokio::{fs, fs::File, io::AsyncReadExt};
use tokio_util::sync::CancellationToken;

pub struct Backlight {
    pub sysfs_path: PathBuf,
    pub max: i32,
    pub current: i32,
    pub target: i32,
    step: i32,
    wait: Duration,
}

impl Backlight {
    pub async fn new(path: PathBuf) -> Result<Self, Box<dyn Error>> {
        let mut new = Self {
            sysfs_path: path,
            max: 0,
            current: 0,
            target: 0,
            step: 0,
            wait: Duration::new(0, 0),
        };
        new.reload().await?;
        new.target = new.current;

        Ok(new)
    }

    pub async fn reload(&mut self) -> Result<(), Box<dyn Error>> {
        self.max = self.load_sysfs("max_brightness").await?;
        self.current = self.load_sysfs("brightness").await?;
        Ok(())
    }

    pub fn update_target(&mut self, target: i32) {
        self.target = target;
        if self.target < 0 {
            self.target = 0;
        }
        if self.target > self.max {
            self.target = self.max;
        }

        if self.target == self.current {
            return;
        }

        let diff = (self.target - self.current).unsigned_abs();

        let mut wait = 500 / diff;
        if wait < 10 {
            wait = 10;
        }

        let step = (wait * diff + 499) / 500;

        println!("diff {} wait {} step {}", diff, wait, step);

        self.wait = Duration::from_millis(wait as u64);
        self.step = step as i32;

        if self.target < self.current {
            self.step *= -1;
        }
    }

    pub async fn run_down(&mut self, token: CancellationToken) {
        while self.step.abs() < (self.target - self.current).abs() {
            self.current += self.step;
            if let Err(err) = self.write_sysfs("brightness", self.current).await {
                error!("Encountered {} while trying to write brightness", err);
            };
            tokio::select! {
                _ = token.cancelled() => {
                    return;
                }
                _ = tokio::time::sleep(self.wait) => {}
            }
        }
        self.current = self.target;
        if let Err(err) = self.write_sysfs("brightness", self.current).await {
            error!("Encountered {} while trying to write brightness", err);
        };
    }

    async fn write_sysfs(
        &mut self,
        component: impl AsRef<Path>,
        value: i32,
    ) -> Result<(), Box<dyn Error>> {
        let mut buf = value.to_string();
        buf.push('\n');
        fs::write(self.sysfs_path.join(component), buf).await?;
        Ok(())
    }

    async fn load_sysfs(&mut self, component: impl AsRef<Path>) -> Result<i32, Box<dyn Error>> {
        let mut buf = String::new();
        File::open(self.sysfs_path.join(component))
            .await?
            .read_to_string(&mut buf)
            .await?;
        Ok(buf.trim().parse()?)
    }
}

pub async fn get() -> Result<Box<[Backlight]>, std::io::Error> {
    Ok(futures::future::join_all(
        Path::new("/sys/class/backlight/")
            .read_dir()?
            .filter_map(|path| path.ok())
            .map(async |entry| Backlight::new(entry.path()).await),
    )
    .await
    .into_iter()
    .filter_map(|path| path.ok())
    .collect())
}
