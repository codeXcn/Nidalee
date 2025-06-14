use crate::lcu::types::{LcuAuthInfo, CurrentChampion, ChampSelectSession, ChampSelectPlayer};
use reqwest::Client;
use base64::{engine::general_purpose, Engine as _};
use serde_json::{Value, Number};
use crate::lcu::summoner::get_summoner_by_id;
use crate::lcu::types::SummonerInfo;

// ---------- 数据清洗函数 ----------

fn fix_team_array(team: &mut Vec<Value>) {
    for player in team {
        if let Some(player_obj) = player.as_object_mut() {
            // summonerId 转字符串
            if let Some(summoner_id) = player_obj.get("summonerId") {
                if let Some(id) = summoner_id.as_u64() {
                    player_obj.insert("summonerId".to_string(), Value::String(id.to_string()));
                }
            }
            // 处理其他大数值字段
            for field in ["championId", "championPickIntent", "selectedSkinId", "spell1Id", "spell2Id"] {
                if let Some(value) = player_obj.get(field) {
                    if let Some(num) = value.as_f64() {
                        if num == 1.8446744073709552e19 || num == 0.0 {
                            player_obj.insert(field.to_string(), Value::Null);
                        } else if let Some(number) = Number::from_f64(num) {
                            player_obj.insert(field.to_string(), Value::Number(number));
                        }
                    }
                }
            }
        }
    }
}

fn fix_bans(ban_list: &mut Vec<Value>) {
    for ban in ban_list {
        if let Some(num) = ban.as_f64() {
            if num == 0.0 {
                *ban = Value::Null;
            } else if let Some(number) = Number::from_f64(num) {
                *ban = Value::Number(number);
            }
        }
    }
}

/// 批量 enrich 召唤师信息
async fn enrich_champ_select_session(
    client: &Client,
    auth: &LcuAuthInfo,
    session: &mut ChampSelectSession,
) {
    // 收集所有 summoner_id
    let mut all_ids = vec![];
    for p in session.my_team.iter().chain(session.their_team.iter()) {
        if let Some(sid) = &p.summoner_id {
            if sid != "0" && !all_ids.contains(sid) {
                all_ids.push(sid.clone());
            }
        }
    }
    // 查询所有召唤师信息
    let mut info_map = std::collections::HashMap::new();
    for sid in &all_ids {
        if let Ok(id) = sid.parse::<u64>() {
            if let Ok(info) = get_summoner_by_id(client, auth, id).await {
                info_map.insert(sid.clone(), info);
            }
        }
    }
    // 补全 my_team
    for p in session.my_team.iter_mut() {
        enrich_player(p, &info_map);
    }
    // 补全 their_team
    for p in session.their_team.iter_mut() {
        enrich_player(p, &info_map);
    }
}

fn enrich_player(player: &mut ChampSelectPlayer, info_map: &std::collections::HashMap<String, SummonerInfo>) {
    if let Some(sid) = &player.summoner_id {
        if sid == "0" {
            player.display_name = Some("机器人".to_string());
            player.tag_line = None;
            player.profile_icon_id = None;
            player.tier = None;
        } else if let Some(info) = info_map.get(sid) {
            // 优先用 game_name + tag_line
            let display_name = if let (Some(game_name), Some(tag_line)) = (&info.game_name, &info.tag_line) {
                format!("{}#{}", game_name, tag_line)
            } else {
                info.display_name.clone()
            };
            player.display_name = Some(display_name);
            player.tag_line = info.tag_line.clone();
            player.profile_icon_id = Some(info.profile_icon_id);
            player.tier = info.solo_rank_tier.clone();
        }
    }
}

// ---------- 主函数 ----------

/// 获取当前选人阶段的完整 session 信息（最优实践版）
pub async fn get_champ_select_session(
    client: &Client,
    auth: &LcuAuthInfo,
) -> Result<ChampSelectSession, String> {
    let url = format!(
        "https://127.0.0.1:{}/lol-champ-select/v1/session",
        auth.app_port
    );

    let auth_string = format!("riot:{}", auth.remoting_auth_token);
    let base64_auth = general_purpose::STANDARD.encode(auth_string.as_bytes());

    let response = client
        .get(&url)
        .header("Authorization", format!("Basic {}", base64_auth))
        .send()
        .await
        .map_err(|e| format!("请求失败: {}", e))?;

    if !response.status().is_success() {
        return Err(format!(
            "获取选人 session 失败: {}",
            response.status()
        ));
    }

    let mut json: Value = response
        .json()
        .await
        .map_err(|e| format!("解析 session 响应失败: {}", e))?;

    // 数据清洗 -- myTeam & theirTeam
    if let Some(my_team) = json.get_mut("myTeam").and_then(|t| t.as_array_mut()) {
        fix_team_array(my_team);
    }
    if let Some(their_team) = json.get_mut("theirTeam").and_then(|t| t.as_array_mut()) {
        fix_team_array(their_team);
    }

    // 数据清洗 -- bans
    if let Some(bans) = json.get_mut("bans").and_then(|b| b.as_object_mut()) {
        for team in ["myTeamBans", "theirTeamBans"] {
            if let Some(ban_list) = bans.get_mut(team).and_then(|l| l.as_array_mut()) {
                fix_bans(ban_list);
            }
        }
    }

    // 反序列化为结构体
    let mut session = serde_json::from_value::<ChampSelectSession>(json)
        .map_err(|e| format!("解析 session 响应失败: {}", e))?;
    // enrich
    enrich_champ_select_session(client, auth, &mut session).await;
    Ok(session)
}
/// 获取当前选择的英雄
pub async fn get_current_champion(client: &Client, auth: &LcuAuthInfo) -> Result<CurrentChampion, String> {
    let url = format!(
        "https://127.0.0.1:{}/lol-champ-select/v1/current-champion",
        auth.app_port
    );

    // 使用正确的认证格式
    let auth_string = format!("riot:{}", auth.remoting_auth_token);
    let base64_auth = general_purpose::STANDARD.encode(auth_string.as_bytes());

    let response = client
        .get(&url)
        .header("Authorization", format!("Basic {}", base64_auth))
        .send()
        .await
        .map_err(|e| format!("请求失败: {}", e))?;

    if !response.status().is_success() {
        return Err(format!(
            "获取当前英雄失败: {}",
            response.status()
        ));
    }

    let champion = response
        .json::<CurrentChampion>()
        .await
        .map_err(|e| format!("解析响应失败: {}", e))?;

    Ok(champion)
}
