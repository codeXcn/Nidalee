// Tauri 命令模块 - 集中管理所有的 Tauri 命令
use std::collections::HashMap;
use crate::{lcu, http_client};

#[tauri::command]
pub async fn get_game_version() -> Result<String, String> {
    // 尝试从公开的Riot API获取最新版本
    let client = http_client::get_public_client();

    match client
        .get("https://ddragon.leagueoflegends.com/api/versions.json")
        .send()
        .await
    {
        Ok(response) => {
            if let Ok(versions) = response.json::<Vec<String>>().await {
                if let Some(latest_version) = versions.first() {
                    return Ok(latest_version.clone());
                }
            }
        }
        Err(_) => {
            // 如果获取失败，返回一个相对较新的默认版本
            return Ok("14.23.1".to_string());
        }
    }

    // 备用默认版本
    Ok("14.23.1".to_string())
}



#[tauri::command]
pub async fn get_match_history() -> Result<lcu::types::MatchStatistics, String> {
    let client = http_client::get_lcu_client();
    lcu::match_history::get_match_history(client).await
}

#[tauri::command]
pub async fn get_game_detail(game_id: u64) -> Result<serde_json::Value, String> {
    let client = http_client::get_lcu_client();
    lcu::match_history::get_game_detail(client, game_id).await
}

#[tauri::command]
pub async fn start_matchmaking() -> Result<(), String> {
    let client = http_client::get_lcu_client();
    lcu::matchmaking::start_matchmaking(client).await
}

#[tauri::command]
pub async fn stop_matchmaking() -> Result<(), String> {
    let client = http_client::get_lcu_client();
    lcu::matchmaking::stop_matchmaking(client).await
}

#[tauri::command]
pub async fn accept_match() -> Result<(), String> {
    let client = http_client::get_lcu_client();
    lcu::matchmaking::accept_match(client).await
}

#[tauri::command]
pub async fn decline_match() -> Result<(), String> {
    let client = http_client::get_lcu_client();
    lcu::matchmaking::decline_match(client).await
}

#[tauri::command]
pub async fn get_current_summoner() -> Result<lcu::types::SummonerInfo, String> {
    let client = http_client::get_lcu_client();
    lcu::summoner::get_current_summoner(client).await
}

#[tauri::command]
pub async fn get_summoner_by_id(id: u64) -> Result<Option<lcu::types::SummonerInfo>, String> {
    let client = http_client::get_lcu_client();

    match crate::lcu::summoner::get_summoner_by_id(client, id).await {
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
pub async fn get_champselect_team_players_info(
) -> Result<HashMap<String, lcu::types::MatchStatistics>, String> {
    let client = http_client::get_lcu_client();
    lcu::champ_select::get_champselect_team_players_info(client).await
}

#[tauri::command]
pub async fn get_summoners_and_histories(
    names: Vec<String>,
) -> Result<Vec<lcu::types::SummonerWithMatches>, String> {
    use crate::lcu::summoner::fill_summoner_extra_info;

    println!("[get_summoners_and_histories] 查询召唤师列表: {:?}", names);
    let client = http_client::get_lcu_client();

    // 1. 批量查召唤师信息
    let mut summoners = lcu::summoner::get_summoners_by_names(client, names)
        .await
        .map_err(|e| format!("批量获取召唤师信息失败: {}", e))?;

    // 2. 合并所有召唤师的信息
    let mut result = Vec::new();
    for summoner in &mut summoners {
        let puuid = summoner.puuid.clone();
        println!("[get_summoners_and_histories] summoner puuid: {}", puuid);
        if !puuid.is_empty() {
            fill_summoner_extra_info(client, summoner).await;
            match lcu::match_history::get_recent_matches_by_summoner_id(client, &puuid, 10).await {
                Ok(matches) => {
                    result.push(lcu::types::SummonerWithMatches {
                        display_name: summoner.display_name.clone(),
                        summoner_info: summoner.clone(),
                        matches,
                    });
                }
                Err(e) => {
                    println!("获取召唤师 {} 的对局历史失败: {}", summoner.display_name, e);
                    // 即使获取对局历史失败，也添加召唤师信息（但没有对局数据）
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
    lcu::summoner::set_summoner_background(client, skin_id).await
}

// 选人相关 API
#[tauri::command]
pub async fn get_champ_select_session() -> Result<serde_json::Value, String> {
    let client = http_client::get_lcu_client();
    lcu::champ_select::get_champ_select_session_raw(client).await
}

#[tauri::command]
pub async fn get_champ_select_session_typed() -> Result<lcu::types::ChampSelectSession, String> {
    let client = http_client::get_lcu_client();
    lcu::champ_select::get_champ_select_session(client).await
}

#[tauri::command]
pub async fn pick_champion(action_id: u64, champion_id: u64, completed: bool) -> Result<String, String> {
    let client = http_client::get_lcu_client();
    match lcu::champ_select::pick_champion(client, action_id, champion_id, completed).await {
        Ok(()) => {
            let action_type = if completed { "锁定" } else { "预选" };
            let message = format!("{}英雄成功 (ActionID: {}, ChampionID: {})", action_type, action_id, champion_id);
            log::info!("[Commands] {}", message);
            Ok(message)
        }
        Err(e) => {
            let action_type = if completed { "锁定" } else { "预选" };
            let error_msg = format!("{}英雄失败: {}", action_type, e);
            log::error!("[Commands] {}", error_msg);
            Err(error_msg)
        }
    }
}

#[tauri::command]
pub async fn ban_champion(action_id: u64, champion_id: u64) -> Result<String, String> {
    let client = http_client::get_lcu_client();
    match lcu::champ_select::ban_champion(client, action_id, champion_id).await {
        Ok(()) => {
            let message = format!("禁用英雄成功 (ActionID: {}, ChampionID: {})", action_id, champion_id);
            log::info!("[Commands] {}", message);
            Ok(message)
        }
        Err(e) => {
            let error_msg = format!("禁用英雄失败: {}", e);
            log::error!("[Commands] {}", error_msg);
            Err(error_msg)
        }
    }
}


