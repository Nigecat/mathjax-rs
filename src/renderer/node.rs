use crate::InitError;
use std::process::Command;

static MATHJAX: &[u8] = include_bytes!("../../mathjax-data/data.zip");
static RENDERER_SRC: &str = include_str!("node-renderer.js");

/// Returns whether [Node.js](https://nodejs.org/en) is available on the system and can be used as a renderer.
pub fn available() -> bool {
    #[inline]
    fn _available() -> Option<bool> {
        let version: node_semver::Version =
            String::from_utf8_lossy(&Command::new("node").arg("-v").output().ok()?.stdout)
                .trim_end_matches('\n')
                .trim_end_matches('\r')
                .parse()
                .ok()?;

        // MathJax requires v6 or later
        Some(version.major >= 6)
    }

    _available().unwrap_or(false)
}

pub struct Node {
    /// The location of the MathJax library source files.
    mathjax_lib: tempfile::TempDir,
}

impl Node {
    pub fn create() -> Result<Self, InitError> {
        let mathjax_lib = tempfile::tempdir()?;
        zip_extract::extract(std::io::Cursor::new(MATHJAX), mathjax_lib.path(), true)?;
        Ok(Node { mathjax_lib })
    }

    pub fn render(&self, expression: &str) -> Result<super::Render, crate::RenderError> {
        let cmd = Command::new("node")
            .args(["-e", RENDERER_SRC, expression])
            .current_dir(self.mathjax_lib.path())
            .output()?;
        let stderr = String::from_utf8_lossy(&cmd.stderr).to_string();

        if stderr.is_empty() {
            let svg = String::from_utf8_lossy(&cmd.stdout).to_string();
            Ok(super::Render::new(svg))
        } else {
            Err(crate::RenderError::MathJaxError(stderr))
        }
    }
}
