use zed_extension_api::{self as zed, LanguageServerId, Result};

struct NixExtension;

impl zed::Extension for TabPredictionNextActionExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let path = worktree.which("nixd").ok_or_else(|| {
            "The Tab Prediction for Next Action language server (tab-prediction) is not available in your environment (PATH).
                You can install it from https://github.com/adnanwahab/zed-tab-prediction-next-action."
                .to_string()
        })?;

        Ok(zed::Command {
            command: path,
            args: vec![],
            env: vec![],
        })
    }
}

zed::register_extension!(TabPredictionNextActionExtension);