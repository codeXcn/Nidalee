use crate::{http_client, lcu};

#[tauri::command]
pub async fn get_recent_matches_by_puuid(puuid: String, count: Option<usize>) -> Result<lcu::types::MatchStatistics, String> {
    let client = http_client::get_lcu_client();
    let count = count.unwrap_or(20);
    lcu::matches::service::get_recent_matches_by_puuid(&client, &puuid, count).await 
}

#[tauri::command]
pub async fn get_current_summoner() -> Result<lcu::types::SummonerInfo, String> {
    let client = http_client::get_lcu_client();
    lcu::summoner::service::get_current_summoner(client).await
}

#[tauri::command]
pub async fn get_summoner_by_id(id: u64) -> Result<Option<lcu::types::SummonerInfo>, String> {
    let client = http_client::get_lcu_client();
    match lcu::summoner::service::get_summoner_by_id(client, id).await {
        Ok(info) => Ok(Some(info)),
        Err(e) => {
            if e.contains("404") {
                Ok(None)
            } else {
                Err(e)
            }
        }
    }
}

#[tauri::command]
pub async fn get_summoners_and_histories(
    names: Vec<String>,
) -> Result<Vec<lcu::types::SummonerWithMatches>, String> {
    use lcu::summoner::service::fill_summoner_extra_info;
    let client = http_client::get_lcu_client();
    let mut summoners = lcu::summoner::service::get_summoners_by_names(client, names)
        .await
        .map_err(|e| format!("批量获取召唤师信息失败: {}", e))?;
    let mut result = Vec::new();
    for summoner in &mut summoners {
        let puuid = summoner.puuid.clone();
        if !puuid.is_empty() {
            fill_summoner_extra_info(client, summoner).await;
            match lcu::matches::service::get_recent_matches_by_puuid(client, &puuid, 20).await
            {
                Ok(matches) => {
                    result.push(lcu::types::SummonerWithMatches {
                        display_name: summoner.display_name.clone(),
                        summoner_info: summoner.clone(),
                        matches,
                    });
                }
                Err(e) => {
                    result.push(lcu::types::SummonerWithMatches {
                        display_name: summoner.display_name.clone(),
                        summoner_info: summoner.clone(),
                        matches: lcu::types::MatchStatistics {
                            total_games: 0,
                            wins: 0,
                            losses: 0,
                            win_rate: 0.0,
                            avg_kills: 0.0,
                            avg_deaths: 0.0,
                            avg_assists: 0.0,
                            avg_kda: 0.0,
                            favorite_champions: Vec::new(),
                            recent_performance: Vec::new(),
                        },
                    });
                }
            }
        }
    }
    Ok(result)
}

#[tauri::command]
pub async fn set_summoner_background_skin(skin_id: u64) -> Result<(), String> {
    let client = http_client::get_lcu_client();
    lcu::summoner::service::set_summoner_background(client, skin_id).await
}

#[tauri::command]
pub async fn set_summoner_chat_profile(
    status_message: Option<String>,
    queue: Option<String>,
    tier: Option<String>,
    division: Option<String>,
) -> Result<(), String> {
    let client = http_client::get_lcu_client();
    lcu::summoner::service::set_summoner_chat_profile(client, status_message, queue, tier, division)
        .await
}
