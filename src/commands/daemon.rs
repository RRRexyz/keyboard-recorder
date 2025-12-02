use std::{
    env, fs,
    path::PathBuf,
    process::{Command as ProcessCommand, Stdio},
};

use anyhow::{Context, Result, anyhow, bail};

use crate::logging;

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

#[cfg(target_os = "windows")]
use windows_sys::Win32::Foundation::CloseHandle;
#[cfg(target_os = "windows")]
use windows_sys::Win32::System::Threading::{
    GetExitCodeProcess, OpenProcess, PROCESS_QUERY_LIMITED_INFORMATION, PROCESS_TERMINATE,
    TerminateProcess,
};

#[cfg(target_os = "windows")]
const CREATE_NO_WINDOW: u32 = 0x0800_0000;
#[cfg(target_os = "windows")]
const STILL_ACTIVE: u32 = 259;

/// Start the recorder as a background daemon (Windows-only).
#[cfg(target_os = "windows")]
pub fn start_daemon() -> Result<()> {
    if let Some(pid) = read_pid()? {
        if is_process_running(pid) {
            bail!("Keyboard recorder is already running (PID {}).", pid);
        } else {
            remove_pid_file()?;
        }
    }

    let mut command =
        ProcessCommand::new(env::current_exe().context("Failed to resolve executable")?);
    command.arg("__daemon");
    command.stdin(Stdio::null());
    command.stdout(Stdio::null());
    command.stderr(Stdio::null());
    command.creation_flags(CREATE_NO_WINDOW);

    let child = command
        .spawn()
        .context("Failed to start background process")?;

    write_pid(child.id())?;
    logging::info(format!("Keyboard recorder started (PID {}).", child.id()));
    Ok(())
}

/// Start daemon stub for unsupported platforms.
#[cfg(not(target_os = "windows"))]
pub fn start_daemon() -> Result<()> {
    bail!("Background mode is only supported on Windows.");
}

/// Stop the recorder daemon (Windows-only).
#[cfg(target_os = "windows")]
pub fn stop_daemon() -> Result<()> {
    let pid = match read_pid()? {
        Some(pid) => pid,
        None => {
            logging::info("Keyboard recorder is not running.");
            return Ok(());
        }
    };

    if !is_process_running(pid) {
        remove_pid_file()?;
        logging::info("Keyboard recorder is not running.");
        return Ok(());
    }

    terminate_process(pid)?;
    remove_pid_file()?;
    logging::info("Keyboard recorder stopped.");
    Ok(())
}

/// Stop daemon stub for unsupported platforms.
#[cfg(not(target_os = "windows"))]
pub fn stop_daemon() -> Result<()> {
    bail!("Background mode is only supported on Windows.");
}

fn pid_dir() -> Result<PathBuf> {
    if let Some(appdata) = env::var_os("APPDATA") {
        let dir = PathBuf::from(appdata).join("kero");
        fs::create_dir_all(&dir).context("Failed to create application directory")?;
        return Ok(dir);
    }

    let dir = env::temp_dir().join("kero");
    fs::create_dir_all(&dir).context("Failed to create temporary directory")?;
    Ok(dir)
}

fn pid_file_path() -> Result<PathBuf> {
    Ok(pid_dir()?.join("kero.pid"))
}

fn read_pid() -> Result<Option<u32>> {
    let path = pid_file_path()?;
    if !path.exists() {
        return Ok(None);
    }
    let contents = fs::read_to_string(&path).context("Failed to read pid file")?;
    let pid = contents
        .trim()
        .parse::<u32>()
        .map_err(|_| anyhow!("PID file contains invalid data"))?;
    Ok(Some(pid))
}

fn write_pid(pid: u32) -> Result<()> {
    let path = pid_file_path()?;
    fs::write(&path, pid.to_string()).context("Failed to write pid file")
}

fn remove_pid_file() -> Result<()> {
    let path = pid_file_path()?;
    if path.exists() {
        fs::remove_file(&path).context("Failed to remove pid file")?;
    }
    Ok(())
}

#[cfg(target_os = "windows")]
fn is_process_running(pid: u32) -> bool {
    unsafe {
        let handle = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, 0, pid);
        if handle.is_null() {
            return false;
        }

        let mut exit_code: u32 = 0;
        let success = GetExitCodeProcess(handle, &mut exit_code) != 0;
        CloseHandle(handle);
        success && exit_code == STILL_ACTIVE
    }
}

#[cfg(not(target_os = "windows"))]
fn is_process_running(_pid: u32) -> bool {
    false
}

#[cfg(target_os = "windows")]
fn terminate_process(pid: u32) -> Result<()> {
    unsafe {
        let handle = OpenProcess(PROCESS_TERMINATE, 0, pid);
        if handle.is_null() {
            bail!("Cannot access process {}.", pid);
        }

        let success = TerminateProcess(handle, 0) != 0;
        CloseHandle(handle);
        if !success {
            bail!("Failed to terminate process {}.", pid);
        }
    }
    Ok(())
}

#[cfg(not(target_os = "windows"))]
fn terminate_process(_pid: u32) -> Result<()> {
    bail!("Background mode is only supported on Windows.");
}
