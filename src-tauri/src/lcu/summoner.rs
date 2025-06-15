use reqwest::Client;
use tauri::command;
use crate::lcu::types::{SummonerInfo, RankInfo};
use crate::lcu::request::lcu_get;
use serde_json::Value;

pub async fn get_current_summoner(client: &Client) -> Result<SummonerInfo, String> {
    let mut summoner_info: SummonerInfo = lcu_get(client, "/lol-summoner/v1/current-summoner").await?;

    // 获取段位信息
    if let Ok(rank_info) = get_rank_info(client, &summoner_info.puuid).await {
        summoner_info.solo_rank_tier = rank_info.solo_tier;
        summoner_info.solo_rank_division = rank_info.solo_division;
        summoner_info.solo_rank_lp = rank_info.solo_lp;
        summoner_info.solo_rank_wins = rank_info.solo_wins;
        summoner_info.solo_rank_losses = rank_info.solo_losses;
        summoner_info.flex_rank_tier = rank_info.flex_tier;
        summoner_info.flex_rank_division = rank_info.flex_division;
        summoner_info.flex_rank_lp = rank_info.flex_lp;
        summoner_info.flex_rank_wins = rank_info.flex_wins;
        summoner_info.flex_rank_losses = rank_info.flex_losses;
    }

    if let (Some(game_name), Some(tag_line)) = (summoner_info.game_name.clone(), summoner_info.tag_line.clone()) {
        summoner_info.display_name = format!("{}#{}", game_name, tag_line);
    }

    Ok(summoner_info)
}

pub async fn get_rank_info(client: &Client, puuid: &str) -> Result<RankInfo, String> {
    let path = &format!("/lol-ranked/v1/ranked-stats/{}", puuid);
    let rank_data: Value = lcu_get(client, path).await?;

    let mut rank_info = RankInfo::default();
    if let Some(queues) = rank_data.get("queues").and_then(|q| q.as_array()) {
        for queue in queues {
            let queue_type = queue.get("queueType").and_then(|q| q.as_str()).unwrap_or("");
            match queue_type {
                "RANKED_SOLO_5x5" => {
                    rank_info.solo_tier = queue.get("tier").and_then(|t| t.as_str()).map(String::from);
                    rank_info.solo_division = queue.get("division").and_then(|d| d.as_str()).map(String::from);
                    rank_info.solo_lp = queue.get("leaguePoints").and_then(|l| l.as_i64()).map(|l| l as i32);
                    rank_info.solo_wins = queue.get("wins").and_then(|w| w.as_i64()).map(|w| w as i32);
                    rank_info.solo_losses = queue.get("losses").and_then(|l| l.as_i64()).map(|l| l as i32);
                }
                "RANKED_FLEX_SR" => {
                    rank_info.flex_tier = queue.get("tier").and_then(|t| t.as_str()).map(String::from);
                    rank_info.flex_division = queue.get("division").and_then(|d| d.as_str()).map(String::from);
                    rank_info.flex_lp = queue.get("leaguePoints").and_then(|l| l.as_i64()).map(|l| l as i32);
                    rank_info.flex_wins = queue.get("wins").and_then(|w| w.as_i64()).map(|w| w as i32);
                    rank_info.flex_losses = queue.get("losses").and_then(|l| l.as_i64()).map(|l| l as i32);
                }
                _ => {}
            }
        }
    }
    Ok(rank_info)
}

// 获取指定ID的召唤师
pub async fn get_summoner_by_id(client: &Client, summoner_id: u64) -> Result<SummonerInfo, String> {
    let path = &format!("/lol-summoner/v1/summoners/{}", summoner_id);
    lcu_get(client, path).await
}