use zed_extension_api::{self as zed, SlashCommand, SlashCommandOutput, SlashCommandOutputSection};

struct CheckExtension;

impl zed::Extension for CheckExtension {
    fn new() -> Self {
        CheckExtension
    }

    fn complete_slash_command_argument(
        &self,
        _command: SlashCommand,
        _args: Vec<String>,
    ) -> Result<Vec<zed_extension_api::SlashCommandArgumentCompletion>, String> {
        Ok(vec![])
    }

    fn run_slash_command(
        &self,
        _command: SlashCommand,
        args: Vec<String>,
        _worktree: Option<&zed::Worktree>,
    ) -> Result<SlashCommandOutput, String> {
        let cmd = args.join(" ");
        if cmd.is_empty() {
            return Err("Usage: /check <command>".to_string());
        }

        let client_id = std::env::var("GOL_CLIENT_ID").unwrap_or_default();

        if client_id.is_empty() {
            return Err(
                "GOL_CLIENT_ID not set. Get a free key instantly with `npx @golproductions/check --install`, then set GOL_CLIENT_ID in your environment.".to_string(),
            );
        }

        let body = serde_json::json!({
            "command": cmd,
            "platform": "zed",
            "channel": "zed",
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
            body: Some(body.to_string().into_bytes()),
            redirect_policy: zed::http_client::RedirectPolicy::FollowAll,
        };

        let response =
            zed::http_client::fetch(&req).map_err(|e| format!("Check API error: {e}"))?;

        let body_str = String::from_utf8(response.body).map_err(|e| format!("UTF-8 error: {e}"))?;
        let data: serde_json::Value =
            serde_json::from_str(&body_str).map_err(|e| format!("Parse error: {e}"))?;

        let verdict = data["verdict"].as_str().unwrap_or("error");
        let reason = data["reason"].as_str().unwrap_or("");

        let text = if verdict == "runnable" {
            format!("Runnable: {}", &cmd[..cmd.len().min(80)])
        } else if reason.is_empty() {
            format!("Blocked: {}", &cmd[..cmd.len().min(80)])
        } else {
            format!("Blocked: {}", reason)
        };

        Ok(SlashCommandOutput {
            text: text.clone(),
            sections: vec![SlashCommandOutputSection {
                range: (0..text.len()).into(),
                label: "GOL Check".to_string(),
            }],
        })
    }
}

zed::register_extension!(CheckExtension);
