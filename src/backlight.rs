use log::error;
use std::{
    error::Error,
    path::{Path, PathBuf},
    time::Duration,
};
use tokio::{fs::File, io::AsyncReadExt};
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

        let mut wait = 400 / diff;
        if wait < 10 {
            wait = 10;
        }

        let step = (wait * diff) / 400 + 1;

        self.wait = Duration::from_millis(wait as u64);
        self.step = step as i32;

        if self.target < self.current {
            self.step *= -1;
        }
    }

    pub async fn run_down(&mut self, token: CancellationToken) {
        while self.step.abs() < (self.target - self.current).abs() {
            self.current += self.step;
            self.write().await;

            tokio::select! {
                _ = token.cancelled() => {
                    return;
                }
                _ = tokio::time::sleep(self.wait) => {}
            }
        }
        self.current = self.target;
        self.write().await;
    }

    async fn write(&mut self) {
        #[cfg(feature = "sd_dbus")]
        let write = self.write_sdbus(self.current);

        #[cfg(not(feature = "sd_dbus"))]
        let write = self.write_sysfs("brightness", self.current);

        if let Err(err) = write.await {
            error!("Encountered {} while trying to write brightness", err);
        };
    }

    #[cfg(not(feature = "sd_dbus"))]
    async fn write_sysfs(
        &mut self,
        component: impl AsRef<Path>,
        value: i32,
    ) -> Result<(), Box<dyn Error>> {
        let mut buf = value.to_string();
        buf.push('\n');
        tokio::fs::write(self.sysfs_path.join(component), buf).await?;
        Ok(())
    }

    #[cfg(feature = "sd_dbus")]
    async fn write_sdbus(&mut self, value: i32) -> Result<(), Box<dyn Error>> {
        use systemd::bus::{Bus, BusName, InterfaceName, MemberName, ObjectPath};
        use utf8_cstr::Utf8CStr;

        let mut bus = Bus::default_system()?;

        let mut method = bus.new_method_call(
            BusName::from_bytes("org.freedesktop.login1\0".as_bytes()).unwrap(),
            ObjectPath::from_bytes("/org/freedesktop/login1/session/auto\0".as_bytes()).unwrap(),
            InterfaceName::from_bytes("org.freedesktop.login1.Session\0".as_bytes()).unwrap(),
            MemberName::from_bytes("SetBrightness\0".as_bytes()).unwrap(),
        )?;

        let id = String::from(self.sysfs_path.file_name().unwrap().to_str().unwrap());
        method.append(Utf8CStr::from_bytes("backlight\0".as_bytes()).unwrap())?;
        method.append(Utf8CStr::from_bytes((id + "\0").as_bytes()).unwrap())?;
        method.append(value as u32)?;

        method.call(0)?;

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
