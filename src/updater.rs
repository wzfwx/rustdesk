use hbb_common::{log, ResultType};
use std::{
    io::{self, Write},
    path::PathBuf,
    sync::{
        atomic::{AtomicUsize, Ordering},
        mpsc::{channel, Receiver, Sender},
        Mutex,
    },
    time::Duration,
};

enum UpdateMsg {
    CheckUpdate,
    Exit,
}

lazy_static::lazy_static! {
    static ref TX_MSG: Mutex<Sender<UpdateMsg>> = Mutex::new(start_auto_update_check());
}

static CONTROLLING_SESSION_COUNT: AtomicUsize = AtomicUsize::new(0);

pub fn update_controlling_session_count(count: usize) {
    CONTROLLING_SESSION_COUNT.store(count, Ordering::SeqCst);
}

pub fn start_auto_update() {
    let _sender = TX_MSG.lock().unwrap();
}

#[allow(dead_code)]
pub fn manually_check_update() -> ResultType<()> {
    log::info!("Manual update check disabled in this build.");
    Ok(())
}

#[allow(dead_code)]
pub fn stop_auto_update() {
    let sender = TX_MSG.lock().unwrap();
    sender.send(UpdateMsg::Exit).unwrap_or_default();
}

#[inline]
fn has_no_active_conns() -> bool {
    // 保留函数签名以兼容其他模块
    true
}

#[cfg(any(not(target_os = "windows"), feature = "flutter"))]
fn has_no_controlling_conns() -> bool {
    CONTROLLING_SESSION_COUNT.load(Ordering::SeqCst) == 0
}

#[cfg(not(any(not(target_os = "windows"), feature = "flutter")))]
fn has_no_controlling_conns() -> bool {
    true
}

fn start_auto_update_check() -> Sender<UpdateMsg> {
    let (tx, rx) = channel();
    std::thread::spawn(move || start_auto_update_check_(rx));
    tx
}

fn start_auto_update_check_(_rx_msg: Receiver<UpdateMsg>) {
    log::info!("Auto update check completely disabled in this build.");
    loop {
        if let Ok(UpdateMsg::Exit) = _rx_msg.recv() {
            break;
        }
    }
}

fn check_update(_manually: bool) -> ResultType<()> {
    log::info!("Software update check disabled in this build.");
    Ok(())
}

#[cfg(target_os = "windows")]
fn update_new_version(_is_msi: bool, _version: &str, _file_path: &PathBuf) {
    log::info!("Update feature disabled. Skipping new version installation.");
}

pub fn get_download_file_from_url(_url: &str) -> Option<PathBuf> {
    None
}
