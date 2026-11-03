use std::fs::File;
use std::path::Path;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

use crate::publication;

struct Execution {
    child: std::process::Child,
    owns_group: bool,
    waited: bool,
}

impl Drop for Execution {
    fn drop(&mut self) {
        if !self.waited {
            let pid = self.child.id();
            let target = if self.owns_group {
                format!("-{pid}")
            } else {
                pid.to_string()
            };
            let stop = Command::new("kill").args(["-KILL", "--", &target]).status();
            let wait = self.child.wait();
            if !stop.as_ref().is_ok_and(|s| s.success()) || wait.is_err() {
                eprintln!("execution cleanup failed: stop={stop:?}, wait={wait:?}");
            }
        }
    }
}

fn reap_group(pid: u32) {
    // Only children adopted by this process are eligible; other groups are untouched.
    while unsafe { libc::waitpid(-(pid as i32), std::ptr::null_mut(), libc::WNOHANG) } > 0 {}
}

pub(crate) fn capture(command: &mut Command) -> Result<Vec<u8>, String> {
    static NEXT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    let evidence = std::env::temp_dir().join(format!(
        "vanllia-capture-{}-{}",
        std::process::id(),
        NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
    ));
    run(command, &evidence, 600).map_err(|e| {
        format!(
            "{e}; evidence: {}\n{}",
            evidence.display(),
            std::fs::read_to_string(evidence.join("stderr.log")).unwrap_or_default()
        )
    })?;
    let output = std::fs::read(evidence.join("stdout.log")).map_err(|e| e.to_string())?;
    std::fs::remove_dir_all(&evidence)
        .map_err(|e| format!("cleanup {}: {e}", evidence.display()))?;
    Ok(output)
}

pub(super) fn run(command: &mut Command, evidence: &Path, timeout: u64) -> Result<(), String> {
    super::active()?;
    if timeout == 0 {
        return Err("timeout must be nonzero".into());
    }
    std::fs::create_dir_all(evidence).map_err(|e| e.to_string())?;
    let stdout = File::create(evidence.join("stdout.log")).map_err(|e| e.to_string())?;
    let stderr = File::create(evidence.join("stderr.log")).map_err(|e| e.to_string())?;
    command.stdin(Stdio::null()).stdout(stdout).stderr(stderr);
    // Each execution owns a process group. Cancellation of this CLI and timeout
    // terminate that entire group, then wait for the owned child before returning.
    use std::os::unix::process::CommandExt;
    let owns_group = std::env::var_os("VANLLIA_MANAGED_GROUP").is_none();
    if owns_group {
        command.process_group(0).env("VANLLIA_MANAGED_GROUP", "1");
    }
    let child = command.spawn().map_err(|e| e.to_string())?;
    let pid = child.id();
    let mut execution = Execution {
        child,
        owns_group,
        waited: false,
    };
    let started = Instant::now();
    publication::write(
        &evidence.join("execution.json"),
        &serde_json::to_vec_pretty(&serde_json::json!({
            "pid": pid, "command": format!("{command:?}"), "timeout_seconds": timeout,
        }))
        .unwrap(),
    )?;
    let result = loop {
        if let Some(status) = execution.child.try_wait().map_err(|e| e.to_string())? {
            execution.waited = true;
            break if status.success() {
                Ok(())
            } else {
                Err(format!("execution failed: {status}"))
            };
        }
        if started.elapsed() >= Duration::from_secs(timeout) || super::cancelled() {
            let target = if owns_group {
                format!("-{pid}")
            } else {
                pid.to_string()
            };
            let stop = Command::new("kill").args(["-KILL", "--", &target]).status();
            let wait = execution.child.wait();
            execution.waited = wait.is_ok();
            break Err(format!(
                "execution {}: stop={stop:?}, wait={wait:?}",
                if super::cancelled() {
                    "cancelled"
                } else {
                    "timed out"
                }
            ));
        }
        std::thread::sleep(Duration::from_millis(50));
    };
    // A successful leader may not leave a background driver/QEMU behind.
    if owns_group {
        reap_group(pid);
    }
    let remaining = Command::new("kill")
        .args(["-0", "--", &format!("-{pid}")])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map_err(|e| e.to_string())?;
    let result = if owns_group && remaining.success() {
        let cleanup = Command::new("kill")
            .args(["-KILL", "--", &format!("-{pid}")])
            .status();
        let mut confirmed = false;
        for _ in 0..20 {
            reap_group(pid);
            if !Command::new("kill")
                .args(["-0", "--", &format!("-{pid}")])
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status()
                .map_err(|e| e.to_string())?
                .success()
            {
                confirmed = true;
                break;
            }
            std::thread::sleep(Duration::from_millis(50));
        }
        if !confirmed {
            publication::write(
                &evidence.join("STOP_UNCONFIRMED"),
                format!("process group {pid}").as_bytes(),
            )?;
        }
        Err(format!(
            "runner left active descendants; cleanup={cleanup:?}, stop_confirmed={confirmed}; prior={result:?}"
        ))
    } else {
        result
    };
    publication::write(&evidence.join("result.json"), &serde_json::to_vec_pretty(&serde_json::json!({
        "success": result.is_ok(), "result": format!("{result:?}"), "elapsed_ms": started.elapsed().as_millis(),
    })).unwrap())?;
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn timeout_stops_owned_execution_and_keeps_evidence() {
        let path =
            std::env::temp_dir().join(format!("vanllia-timeout-test-{}", std::process::id()));
        let error = run(Command::new("sleep").arg("30"), &path, 1).unwrap_err();
        assert!(error.contains("timed out"), "{error}");
        let execution: serde_json::Value =
            serde_json::from_slice(&std::fs::read(path.join("execution.json")).unwrap()).unwrap();
        let pid = execution["pid"].as_u64().unwrap();
        assert!(!Path::new(&format!("/proc/{pid}")).exists());
        assert!(path.join("result.json").is_file());
        std::fs::remove_dir_all(path).unwrap();
    }

    #[test]
    fn successful_leader_cannot_leave_a_live_descendant() {
        assert_eq!(unsafe { libc::prctl(libc::PR_SET_CHILD_SUBREAPER, 1) }, 0);
        let path =
            std::env::temp_dir().join(format!("vanllia-descendant-test-{}", std::process::id()));
        let error = run(
            Command::new("sh").args(["-c", "sleep 30 & echo $!"]),
            &path,
            5,
        )
        .unwrap_err();
        assert!(error.contains("runner left active descendants"), "{error}");
        assert!(error.contains("stop_confirmed=true"), "{error}");
        let pid = std::fs::read_to_string(path.join("stdout.log")).unwrap();
        assert!(!Path::new(&format!("/proc/{}", pid.trim())).exists());
        std::fs::remove_dir_all(path).unwrap();
    }
}
