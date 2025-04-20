use std::fs::OpenOptions;
use std::io;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

fn get_log_folder() -> io::Result<PathBuf> {
    #[cfg(target_os = "linux")]
    {
        let home = std::env::var("HOME").unwrap();
        let dir = Path::new(&home).join(".local/share/edda/");
        if !dir.exists() {
            std::fs::create_dir_all(&dir)?;
        }
        Ok(dir)
    }
    #[cfg(target_os = "macos")]
    {
        let home = std::env::var("HOME").unwrap();
        let dir = Path::new(&home).join("Library/Logs/Edda/");
        if !dir.exists() {
            std::fs::create_dir_all(&dir)?;
        }
        Ok(dir)
    }
    #[cfg(target_os = "windows")]
    {
        let p = std::env::var("APPDATA")?;
        let dir = Path::new(&p).join("edda");
        if !dir.exists() {
            std::fs::create_dir_all(dir)?;
        }
        Ok(dir.to_path_buf())
    }
}

pub fn write(msg: String) -> io::Result<()> {
    let log_folder = get_log_folder()?;
    let du = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        / (60 * 60 * 24);
    let now = format!(
        "{}{}{}",
        1970 + (du / 365),
        (du % 365) / 30,
        (du % 365) % 30
    );
    let log = log_folder.join(format!("{}_edda.log", now));

    let mut f = OpenOptions::new().append(true).create(true).open(log)?;
    f.write_all(msg.as_bytes())?;

    println!("{msg}");
    Ok(())
}

#[macro_export]
macro_rules! log {
    (INF, $msg:expr) => {
        $crate::logs::write(format!("[INFO] {}", $msg)).unwrap()
    };
    (WAR, $msg:expr) => {
        $crate::logs::write(format!("[WARNING] {}", $msg)).unwrap()
    };
    (ERR, $msg:expr) => {
        $crate::logs::write(format!("[ERROR] {}", $msg)).unwrap()
    };
    (DBG, $msg:expr) => {
        $crate::logs::write(format!("[[[ DEBUG ]]] ==> {}", $msg)).unwrap()
    };
    (CRT, $msg:expr) => {
        $crate::logs::write(format!("[[[ CRITICAL ERROR ]]] ==> {}", $msg)).unwrap()
    };
}
