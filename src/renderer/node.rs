use std::process::Command;

/// Returns whether [Node.js](https://nodejs.org/en) is available on the system and can be used as a renderer.
pub fn available() -> bool {
    #[inline]
    fn _available() -> Option<bool> {
        let version: node_semver::Version =
            String::from_utf8_lossy(&Command::new("node").arg("-v").output().ok()?.stdout)
                .trim_end_matches('\n')
                .parse()
                .ok()?;

        // MathJax requires v6 or later
        Some(version.major >= 6)
    }

    _available().unwrap_or(false)
}

pub fn render(expression: &str) -> Result<super::Render, crate::RenderError> {
    unimplemented!(); // todo
}
