// Tauri 命令模块 - 集中管理所有的 Tauri 命令
use crate::{http_client, lcu};
use std::collections::HashMap;
use serde_json;
use reqwest::Client;
use serde_json::Value;

#[tauri::command]
pub async fn get_live_player_list() -> Result<String, String> {
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true) // 👈 忽略证书错误
        .build()
        .map_err(|e| e.to_string())?;

    let resp = client
        .get("https://127.0.0.1:2999/liveclientdata/playerlist")
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let text = resp.text().await.map_err(|e| e.to_string())?;
    Ok(text)
}

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
pub async fn get_game_detail(game_id: u64) -> Result<lcu::types::GameDetail, String> {
    log::info!("🔍 ===== 获取游戏详细信息 =====");
    log::info!("🎮 游戏ID: {}", game_id);
    let client = http_client::get_lcu_client();
    lcu::match_history::get_game_detail_logic(&client, game_id).await
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
            match lcu::match_history::get_recent_matches_by_summoner_id(client, &puuid, 20).await {
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

#[tauri::command]
pub async fn set_summoner_chat_profile(
    status_message: Option<String>,
    queue: Option<String>,
    tier: Option<String>,
    division: Option<String>,
) -> Result<(), String> {
    let client = http_client::get_lcu_client();
    lcu::summoner::set_summoner_chat_profile(client, status_message, queue, tier, division).await
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
pub async fn pick_champion(
    action_id: u64,
    champion_id: u64,
    completed: bool,
) -> Result<String, String> {
    let client = http_client::get_lcu_client();
    match lcu::champ_select::pick_champion(client, action_id, champion_id, completed).await {
        Ok(()) => {
            let action_type = if completed { "锁定" } else { "预选" };
            let message = format!(
                "{}英雄成功 (ActionID: {}, ChampionID: {})",
                action_type, action_id, champion_id
            );
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
            let message = format!(
                "禁用英雄成功 (ActionID: {}, ChampionID: {})",
                action_id, champion_id
            );
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

// ===== 英雄出装和符文相关命令 =====

#[tauri::command]
pub async fn get_champion_builds(source: String, champion_alias: String) -> Result<serde_json::Value, String> {
    let client = http_client::get_lcu_client();
    lcu::ddragon::get_builds_by_alias(&client, &source, &champion_alias).await
}

#[tauri::command]
pub async fn get_champion_runes(source: String, champion_alias: String) -> Result<Vec<lcu::types::DataDragonRune>, String> {
    let client = http_client::get_lcu_client();
    lcu::ddragon::get_runes_by_alias(&client, &source, &champion_alias).await
}

#[tauri::command]
pub async fn get_all_runes() -> Result<lcu::types::AllRunesResponse, String> {
    let client = http_client::get_lcu_client();
    lcu::ddragon::get_all_runes(&client).await
}

#[tauri::command]
pub async fn get_lcu_rune_styles() -> Result<Vec<lcu::types::RuneStyle>, String> {
    let client = http_client::get_lcu_client();
    lcu::perks::list_all_styles(&client).await
}

#[tauri::command]
pub async fn get_lcu_perks() -> Result<Vec<lcu::types::Perk>, String> {
    let client = http_client::get_lcu_client();
    lcu::perks::list_all_perks(&client).await
}

#[tauri::command]
pub async fn get_lcu_perk_icon(icon_path: String) -> Result<Vec<u8>, String> {
    let client = http_client::get_lcu_client();
    lcu::perks::get_perk_icon(&client, &icon_path).await
}

/// 应用英雄详细（符文配置）
#[tauri::command]
pub async fn apply_champion_build(
    champion_alias: String,
    build_index: usize,
) -> Result<String, String> {
    log::info!("🚀 开始应用英雄详细: {} (详细索引: {})", champion_alias, build_index);

    // 获取LCU连接
    let client = http_client::get_lcu_client();

    // 获取英雄详细数据
    let build_data = match get_champion_builds("op.gg".to_string(), champion_alias.clone()).await {
        Ok(data) => data,
        Err(e) => {
            log::error!("❌ 获取英雄详细数据失败: {}", e);
            return Err(format!("获取英雄详细数据失败: {}", e));
        }
    };

    // 解析详细数据
    let content = build_data
        .get("content")
        .and_then(|c| c.as_array())
        .and_then(|arr| arr.get(0))
        .ok_or("无法获取详细数据")?;

    let runes_array = content
        .get("runes")
        .and_then(|r| r.as_array())
        .ok_or("无法获取符文配置数组")?;

    // 检查详细索引是否有效
    if build_index >= runes_array.len() {
        let msg = format!("详细索引 {} 超出范围，总共有 {} 个详细", build_index, runes_array.len());
        log::error!("❌ {}", msg);
        return Err(msg);
    }

    let rune_build = &runes_array[build_index];
    log::info!("📋 使用符文详细索引: {}", build_index);

    // 提取符文配置信息
    let primary_style_id = rune_build
        .get("primaryStyleId")
        .and_then(|v| v.as_i64())
        .ok_or("无法获取主系符文ID")? as i32;

    let sub_style_id = rune_build
        .get("subStyleId")
        .and_then(|v| v.as_i64())
        .ok_or("无法获取副系符文ID")? as i32;

    let selected_perk_ids: Vec<i32> = rune_build
        .get("selectedPerkIds")
        .and_then(|v| v.as_array())
        .map(|arr| arr.iter().filter_map(|v| v.as_i64().map(|i| i as i32)).collect())
        .unwrap_or_default();

    log::info!("🔧 符文配置 - 主系: {}, 副系: {}, 符文数量: {}",
              primary_style_id, sub_style_id, selected_perk_ids.len());

    // 应用符文配置
    match lcu::build_application::apply_rune_build(
        client,
        &champion_alias,
        primary_style_id,
        sub_style_id,
        selected_perk_ids,
    ).await {
        Ok(message) => {
            log::info!("✅ 符文应用成功: {}", message);
            Ok(format!("符文配置应用成功！{}", message))
        }
        Err(e) => {
            log::error!("❌ 符文应用失败: {}", e);
            Err(format!("符文配置应用失败: {}", e))
        }
    }
}

/// 获取英雄详细数据 - 使用新的 OP.GG API
#[tauri::command]
pub async fn get_champion_build_new(
    region: String,
    mode: String,
    champion_id: i32,
    position: Option<String>,
    tier: String,
) -> Result<Value, String> {
    log::info!("🚀 获取英雄详细数据: 区域={}, 模式={}, 英雄ID={}, 位置={:?}, 段位={}",
              region, mode, champion_id, position, tier);

    let client = Client::new();

    // 详细 API URL
    let url = if mode == "arena" {
        format!("https://lol-api-champion.op.gg/api/{}/champions/{}/{}?tier={}",
               region, mode, champion_id, tier)
    } else {
        let pos = position.unwrap_or("MID".to_string());
        format!("https://lol-api-champion.op.gg/api/{}/champions/{}/{}/{}?tier={}",
               region, mode, champion_id, pos, tier)
    };

    log::info!("🌐 请求URL: {}", url);

    // 发送请求
    let response = client
        .get(&url)
        .header("Accept", "application/json")
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .send()
        .await
        .map_err(|e| format!("网络请求失败: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("API 请求失败: HTTP {}", response.status()));
    }

    let data: Value = response
        .json()
        .await
        .map_err(|e| format!("解析 JSON 失败: {}", e))?;

    log::info!("✅ 成功获取英雄详细数据");
    Ok(data)
}

/// 获取所有英雄列表
#[tauri::command]
pub async fn get_champions_list(
    region: String,
    mode: String,
    tier: String,
) -> Result<Value, String> {
    log::info!("🚀 获取英雄列表: 区域={}, 模式={}, 段位={}", region, mode, tier);

    let client = Client::new();

    let url = format!("https://lol-api-champion.op.gg/api/{}/champions/{}?tier={}",
                     region, mode, tier);

    log::info!("🌐 请求URL: {}", url);

    let response = client
        .get(&url)
        .header("Accept", "application/json")
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .send()
        .await
        .map_err(|e| format!("网络请求失败: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("API 请求失败: HTTP {}", response.status()));
    }

    let data: Value = response
        .json()
        .await
        .map_err(|e| format!("解析 JSON 失败: {}", e))?;

    log::info!("✅ 成功获取英雄列表数据");
    Ok(data)
}

// ===== OP.GG 相关命令 =====

/// 获取英雄详细数据（使用OP.GG API）- 原始返回
#[tauri::command]
pub async fn get_opgg_champion_build_raw(
    region: String,
    mode: String,
    champion_id: i32,
    position: Option<String>,
    tier: String,
) -> Result<Value, String> {
    log::info!("🚀 获取OP.GG英雄详细原始数据: 区域={}, 模式={}, 英雄ID={}, 位置={:?}, 段位={}",
              region, mode, champion_id, position, tier);

    let client = lcu::opgg::OpggClient::new();
    let pos = position.unwrap_or_else(|| "MID".to_string());

    // 获取原始数据
    let raw_data = client
        .get_champion_build(&region, &mode, champion_id, &pos, &tier)
        .await?;

    log::info!("✅ 成功获取OP.GG英雄详细原始数据");
    Ok(raw_data)
}

/// 获取英雄详细数据（使用OP.GG API）- 解析后返回
#[tauri::command]
pub async fn get_opgg_champion_build(
    region: String,
    mode: String,
    champion_id: i32,
    position: Option<String>,
    tier: String,
) -> Result<lcu::opgg::OpggChampionBuild, String> {
    log::info!("🚀 获取OP.GG英雄详细数据: 区域={}, 模式={}, 英雄ID={}, 位置={:?}, 段位={}",
              region, mode, champion_id, position, tier);

    // 先获取原始数据
    let raw_data = get_opgg_champion_build_raw(
        region.clone(),
        mode.clone(),
        champion_id,
        position.clone(),
        tier.clone()
    ).await?;

    let pos = position.unwrap_or_else(|| "MID".to_string());

    // 尝试解析数据
    match lcu::opgg::parse_champion_build(raw_data, &pos) {
        Ok(parsed_build) => {
            log::info!("✅ 成功解析OP.GG英雄详细数据");
            Ok(parsed_build)
        }
        Err(parse_error) => {
            log::error!("❌ 解析OP.GG数据失败: {}", parse_error);
            // 返回一个包含基本信息的默认详细
            let default_build = lcu::opgg::OpggChampionBuild {
                summary: lcu::opgg::OpggChampionSummary {
                    name: format!("Champion_{}", champion_id),
                    champion_id,
                    icon: format!("champion_{}.png", champion_id),
                    position: pos.clone(),
                    win_rate: None,
                    pick_rate: None,
                    ban_rate: None,
                    kda: None,
                    tier: None,
                    rank: None,
                },
                summoner_spells: vec![],
                champion_skills: lcu::opgg::OpggSkills {
                    masteries: vec![],
                    order: vec![],
                    play: 0,
                    win: 0,
                    pick_rate: 0.0,
                },
                items: lcu::opgg::OpggItems {
                    boots: vec![],
                    start_items: vec![],
                    core_items: vec![],
                    last_items: vec![],
                },
                counters: lcu::opgg::OpggCounters {
                    strong_against: vec![],
                    weak_against: vec![],
                },
                perks: vec![],
            };
            Ok(default_build)
        }
    }
}

/// 获取英雄层级列表（使用OP.GG API）
#[tauri::command]
pub async fn get_opgg_tier_list(
    region: String,
    mode: String,
    tier: String,
) -> Result<Value, String> {
    log::info!("🚀 获取OP.GG英雄层级列表: 区域={}, 模式={}, 段位={}", region, mode, tier);

    let client = lcu::opgg::OpggClient::new();
    let data = client.get_tier_list(&region, &mode, &tier).await?;

    log::info!("✅ 成功获取OP.GG英雄层级列表");
    Ok(data)
}

/// 获取英雄可用位置列表
#[tauri::command]
pub async fn get_opgg_champion_positions(
    region: String,
    champion_id: i32,
    tier: String,
) -> Result<Vec<String>, String> {
    log::info!("🚀 获取英雄位置列表: 区域={}, 英雄ID={}, 段位={}", region, champion_id, tier);

    let client = lcu::opgg::OpggClient::new();
    let positions = client.get_champion_positions(&region, champion_id, &tier).await?;

    log::info!("✅ 成功获取英雄位置列表: {:?}", positions);
    Ok(positions)
}

/// 应用OP.GG推荐符文配置
#[tauri::command]
pub async fn apply_opgg_runes(
    region: String,
    mode: String,
    champion_id: i32,
    champion_name: String,
    position: Option<String>,
    tier: String,
    build_index: Option<usize>,
) -> Result<String, String> {
    log::info!("🚀 应用OP.GG推荐符文: 英雄ID={}, 位置={:?}, 详细索引={:?}",
              champion_id, position, build_index);

    // 获取英雄详细数据
    let build = get_opgg_champion_build(region, mode, champion_id, position.clone(), tier).await?;

    let build_idx = build_index.unwrap_or(0);
    if build_idx >= build.perks.len() {
        return Err(format!("详细索引 {} 超出范围，总共有 {} 个详细", build_idx, build.perks.len()));
    }

    let selected_perk = &build.perks[build_idx];

    // 应用符文配置到游戏客户端
    let client = http_client::get_lcu_client();

    match lcu::build_application::apply_rune_build(
        client,
        &champion_name,
        selected_perk.primary_id,
        selected_perk.secondary_id,
        selected_perk.perks.clone(),
    ).await {
        Ok(message) => {
            log::info!("✅ OP.GG符文应用成功: {}", message);
            Ok(format!("OP.GG符文配置应用成功！{}", message))
        }
        Err(e) => {
            log::error!("❌ OP.GG符文应用失败: {}", e);
            Err(format!("OP.GG符文配置应用失败: {}", e))
        }
    }
}

// 内部辅助函数，用于调用其他命令
async fn invoke_command_internal(
    command: &str,
    params: Vec<(String, String)>,
) -> Result<serde_json::Value, String> {
    match command {
        "get_champion_builds" => {
            let source = params.iter().find(|(k, _)| k == "source").map(|(_, v)| v.clone()).unwrap_or_default();
            let champion_alias = params.iter().find(|(k, _)| k == "championAlias").map(|(_, v)| v.clone()).unwrap_or_default();
            get_champion_builds(source, champion_alias).await
        }
        _ => Err("未知命令".to_string())
    }
}
