use std::{
    ffi::OsStr,
    fs::{create_dir_all, File},
    io::Write,
    path::PathBuf,
    sync::Mutex,
};

//------------------------------------------------------------------------------

static GLOBAL_ENABLE: Mutex<bool> = Mutex::new(false);

static TERMINAL_OUTPUT_ENABLE: Mutex<bool> = Mutex::new(false);
static TERMINAL_TIMESTAMP_ENABLE: Mutex<bool> = Mutex::new(false);
static FILE_OUTPUT_ENABLE: Mutex<bool> = Mutex::new(false);
static FILE_TIMESTAMP_ENABLE: Mutex<bool> = Mutex::new(false);
static FILE_HISTORY_ENABLE: Mutex<bool> = Mutex::new(false);

static PATH: Mutex<Option<Box<PathBuf>>> = Mutex::new(None);
static FILE_HANDLE: Mutex<Option<File>> = Mutex::new(None);

//------------------------------------------------------------------------------

#[allow(unused)]
pub fn terminal_output_enable() {
    set(&TERMINAL_OUTPUT_ENABLE);
}

#[allow(unused)]
pub fn terminal_timestamp_enable() {
    set(&TERMINAL_TIMESTAMP_ENABLE);
}

#[allow(unused)]
pub fn file_output_enable() {
    set(&FILE_OUTPUT_ENABLE);
}

#[allow(unused)]
pub fn file_timestamp_enable() {
    set(&FILE_TIMESTAMP_ENABLE);
}

#[allow(unused)]
pub fn file_history_enable() {
    set(&FILE_HISTORY_ENABLE);
}

#[allow(unused)]
pub fn enable() {
    set(&GLOBAL_ENABLE);
    if get(&FILE_OUTPUT_ENABLE) {
        if let Ok(file) = start_log_file() {
            set_file_handle(file);
        }
    }
}

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {{
        debug::__log(format_args!($($arg)*));
    }};
}

pub(crate) use log;

//------------------------------------------------------------------------------

fn start_log_file() -> Result<File, ()> {
    let pathbuf = match get_path() {
        Some(box_pathbuf) => *box_pathbuf.clone(),
        _ => return Err(()),
    };

    let mut new_name = OsStr::new("").to_owned();
    if get(&FILE_HISTORY_ENABLE) {
        new_name.push(get_timestamp());
    }
    let file_name = match pathbuf.file_name() {
        Some(name) => name.to_owned(),
        _ => return Err(()),
    };
    new_name.push(file_name);
    new_name.push(".log");

    let mut new_path = match pathbuf.parent() {
        Some(folder) => folder.to_owned(),
        _ => return Err(()),
    };

    new_path.push("debug");
    match create_dir_all(new_path.clone()) {
        Ok(_) => {}
        _ => return Err(()),
    };

    new_path.push(new_name);

    let result = File::create(new_path);
    match result {
        Ok(file) => Ok(file),
        _ => Err(()),
    }
}

//------------------------------------------------------------------------------

pub fn __log(args: std::fmt::Arguments) {
    if !get(&GLOBAL_ENABLE) {
        return;
    }

    if get(&FILE_OUTPUT_ENABLE) {
        let mut mutex_guard = FILE_HANDLE.lock().unwrap();
        if let Some(file) = &mut *mutex_guard {
            match get(&FILE_TIMESTAMP_ENABLE) {
                true => {
                    let _ = writeln!(file, "{} {}", get_timestamp(), args);
                }
                false => {
                    let _ = writeln!(file, "{}", args);
                }
            };
        }
    }

    if get(&TERMINAL_OUTPUT_ENABLE) {
        match get(&TERMINAL_TIMESTAMP_ENABLE) {
            true => println!("{} {}", get_timestamp(), args),
            false => println!("{}", args),
        }
    }
}

//------------------------------------------------------------------------------

fn set(flag: &Mutex<bool>) {
    let mut mutex_guard = flag.lock().unwrap();
    *mutex_guard = true;
}

fn get(flag: &Mutex<bool>) -> bool {
    let output = flag.lock().unwrap();
    *output
}

#[allow(unused)]
pub fn set_path(path: PathBuf) {
    let mut mutex_guard = PATH.lock().unwrap();
    *mutex_guard = Some(Box::new(path));
}

fn get_path() -> Option<Box<PathBuf>> {
    let mutex_guard = PATH.lock().unwrap();
    mutex_guard.clone()
}

fn set_file_handle(file_handle: File) {
    let mut mutex_guard = FILE_HANDLE.lock().unwrap();
    *mutex_guard = Some(file_handle);
}

//------------------------------------------------------------------------------

use std::time::{SystemTime, UNIX_EPOCH};

fn get_timestamp() -> String {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let mut milliseconds = u64::from(since_the_epoch.subsec_nanos()) / 1_000_000;
    milliseconds %= 1000;

    let time = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let datetime = time.as_secs();

    let year = 1970 + datetime / (365 * 24 * 60 * 60);
    let datetime = datetime % (365 * 24 * 60 * 60);
    let month = datetime / (30 * 24 * 60 * 60);
    let datetime = datetime % (30 * 24 * 60 * 60);
    let day = datetime / (24 * 60 * 60);
    let datetime = datetime % (24 * 60 * 60);
    let hour = datetime / (60 * 60);
    let datetime = datetime % (60 * 60);
    let minute = datetime / 60;
    let second = datetime % 60;

    format!(
        "{:04}{:02}{:02}_{:02}{:02}{:02}_{:03}",
        year, month, day, hour, minute, second, milliseconds
    )
}

//------------------------------------------------------------------------------
