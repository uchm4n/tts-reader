//! macOS `say` command backend.

use std::process::{Child, Command, Stdio};

pub struct SayBackend {
    process: Option<Child>,
}

impl SayBackend {
    pub fn new() -> Self {
        Self { process: None }
    }

    pub fn speak(&mut self, text: &str, rate: f32) {
        self.stop();

        let rate_str = (rate * 200.0) as i32;
        let mut cmd = Command::new("say");
        cmd.arg("-r").arg(rate_str.to_string());

        if !text.is_empty() {
            cmd.arg(text);
        }

        match cmd.stdout(Stdio::null()).stderr(Stdio::null()).spawn() {
            Ok(child) => {
                self.process = Some(child);
            }
            Err(e) => {
                eprintln!("[TTS] Failed to start say command: {}", e);
            }
        }
    }

    pub fn stop(&mut self) {
        if let Some(mut child) = self.process.take() {
            let _ = child.kill();
            let _ = child.wait();
        }
    }

    pub fn set_voice(&mut self, _voice: &str) {
        // say backend doesn't support voice selection
    }

    pub fn is_speaking(&mut self) -> bool {
        if let Some(ref mut child) = self.process {
            match child.try_wait() {
                Ok(Some(_)) => {
                    self.process = None;
                    false
                }
                Ok(None) => true,
                Err(_) => {
                    self.process = None;
                    false
                }
            }
        } else {
            false
        }
    }
}

impl Default for SayBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for SayBackend {
    fn drop(&mut self) {
        self.stop();
    }
}
