use serde_json;

async fn invoke_command_internal(
    command: &str,
    params: Vec<(String, String)>,
) -> Result<serde_json::Value, String> {
    match command {
        "get_champion_builds" => {
            let source = params
                .iter()
                .find(|(k, _)| k == "source")
                .map(|(_, v)| v.clone())
                .unwrap_or_default();
            let champion_alias = params
                .iter()
                .find(|(k, _)| k == "championAlias")
                .map(|(_, v)| v.clone())
                .unwrap_or_default();
            super::builds::get_champion_builds(source, champion_alias).await
        }
        _ => Err("未知命令".to_string()),
    }
}


