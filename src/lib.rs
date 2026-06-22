use zed_extension_api::{self as zed, Command, SlashCommand, SlashCommandOutput, SlashCommandOutputSection};

struct CheckExtension;

impl zed::Extension for CheckExtension {
    fn new() -> Self {
        CheckExtension
    }

    fn complete_slash_command_argument(
        &self,
        command: &SlashCommand,
        _args: &[String],
    ) -> Result<Vec<zed_extension_api::SlashCommandArgumentCompletion>, String> {
        match command.name.as_str() {
            "check" => Ok(vec![]),
            _ => Ok(vec![]),
        }
    }

    fn run_slash_command(
        &self,
        command: &SlashCommand,
        args: &[String],
        _worktree: &zed::Worktree,
    ) -> Result<SlashCommandOutput, String> {
        match command.name.as_str() {
            "check" => {
                let cmd = args.join(" ");
                if cmd.is_empty() {
                    return Err("Usage: /check <command>".to_string());
                }

                let client_id = std::env::var("GOL_CLIENT_ID")
                    .unwrap_or_default();

                if client_id.is_empty() {
                    return Err(
                        "GOL_CLIENT_ID not set. Get one at golproductions.com/check.html"
                            .to_string(),
                    );
                }

                let body = serde_json::json!({
                    "command": cmd,
                    "platform": "zed",
                    "v": "1.0.0"
                });

                let req = zed::http_client::HttpRequest {
                    url: "https://triage.golproductions.com/preflight".to_string(),
                    method: zed::http_client::HttpMethod::Post,
                    headers: vec![
                        ("Content-Type".to_string(), "application/json".to_string()),
                        ("X-GOL-CLIENT-ID".to_string(), client_id),
                        ("User-Agent".to_string(), "zed/1.0.0".to_string()),
                    ],
                    body: Some(body.to_string()),
                    redirect_policy: zed::http_client::RedirectPolicy::FollowAll,
                };

                let response = zed::http_client::fetch(&req)
                    .map_err(|e| format!("Check API error: {e}"))?;

                let data: serde_json::Value = serde_json::from_str(&response.body)
                    .map_err(|e| format!("Parse error: {e}"))?;

                let verdict = data["verdict"].as_str().unwrap_or("error");
                let reason = data["reason"].as_str().unwrap_or("");

                let text = if verdict == "runnable" {
                    format!("✓ Runnable: {}", &cmd[..cmd.len().min(80)])
                } else {
                    format!("✗ Blocked: {}", if reason.is_empty() { &cmd[..cmd.len().min(80)] } else { reason })
                };

                Ok(SlashCommandOutput {
                    text: text.clone(),
                    sections: vec![SlashCommandOutputSection {
                        range: (0..text.len()).into(),
                        label: "GOL Check".to_string(),
                    }],
                })
            }
            _ => Err("Unknown command".to_string()),
        }
    }
}

zed::register_extension!(CheckExtension);
