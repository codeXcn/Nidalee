use base64::{engine::general_purpose, Engine as _};
use reqwest::Client;
use crate::lcu::types::{LcuAuthInfo, SummonerInfo};
use serde_json::Value;

pub async fn get_current_summoner(client: &Client, auth_info: &LcuAuthInfo) -> Result<SummonerInfo, String> {
    println!("[LCU] 开始获取当前召唤师信息...");

    let auth_string = format!("riot:{}", auth_info.remoting_auth_token);
    let base64_auth = general_purpose::STANDARD.encode(auth_string.as_bytes());

    let url = format!(
        "https://127.0.0.1:{}/lol-summoner/v1/current-summoner",
        auth_info.app_port
    );
    println!("[LCU] 请求URL: {}", url);

    let response = client
        .get(&url)
        .header("Authorization", format!("Basic {}", base64_auth))
        .send()
        .await
        .map_err(|e| {
            println!("[LCU] ✗ 获取召唤师信息失败: {}", e);
            format!("获取当前召唤师信息失败: {}", e)
        })?;

    if !response.status().is_success() {
        let error_msg = format!("服务器返回错误: {}", response.status());
        println!("[LCU] ✗ {}", error_msg);
        return Err(error_msg);
    }

    let mut summoner_info: SummonerInfo = response.json().await.map_err(|e| {
        println!("[LCU] ✗ 解析召唤师信息失败: {}", e);
        format!("解析召唤师信息失败: {}", e)
    })?;

    // 获取段位信息
    if let Ok(rank_info) = get_rank_info(client, auth_info, &summoner_info.puuid).await {
        summoner_info.soloRankTier = rank_info.solo_tier;
        summoner_info.soloRankDivision = rank_info.solo_division;
        summoner_info.soloRankLP = rank_info.solo_lp;
        summoner_info.soloRankWins = rank_info.solo_wins;
        summoner_info.soloRankLosses = rank_info.solo_losses;
        summoner_info.flexRankTier = rank_info.flex_tier;
        summoner_info.flexRankDivision = rank_info.flex_division;
        summoner_info.flexRankLP = rank_info.flex_lp;
        summoner_info.flexRankWins = rank_info.flex_wins;
        summoner_info.flexRankLosses = rank_info.flex_losses;
    }

    // 如果有 gameName 和 tagLine，则组合它们
    if let (Some(game_name), Some(tag_line)) = (summoner_info.gameName.clone(), summoner_info.tagLine.clone()) {
        summoner_info.displayName = format!("{}#{}", game_name, tag_line);
    }

    println!("[LCU] ✓ 成功获取召唤师信息: {:?}", summoner_info);
    Ok(summoner_info)
}

// 获取段位信息
async fn get_rank_info(client: &Client, auth_info: &LcuAuthInfo, puuid: &str) -> Result<RankInfo, String> {
    println!("[LCU] 开始获取段位信息...");

    let auth_string = format!("riot:{}", auth_info.remoting_auth_token);
    let base64_auth = general_purpose::STANDARD.encode(auth_string.as_bytes());

    let url = format!(
        "https://127.0.0.1:{}/lol-ranked/v1/ranked-stats/{}",
        auth_info.app_port,
        puuid
    );
    println!("[LCU] 请求URL: {}", url);

    let response = client
        .get(&url)
        .header("Authorization", format!("Basic {}", base64_auth))
        .send()
        .await
        .map_err(|e| format!("获取段位信息失败: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("服务器返回错误: {}", response.status()));
    }

    let rank_data: Value = response.json().await.map_err(|e| format!("解析段位信息失败: {}", e))?;

    let mut rank_info = RankInfo::default();

    // 解析单双排信息
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

    println!("[LCU] ✓ 成功获取段位信息: {:?}", rank_info);
    Ok(rank_info)
}

#[derive(Debug, Default)]
struct RankInfo {
    solo_tier: Option<String>,
    solo_division: Option<String>,
    solo_lp: Option<i32>,
    solo_wins: Option<i32>,
    solo_losses: Option<i32>,

    flex_tier: Option<String>,
    flex_division: Option<String>,
    flex_lp: Option<i32>,
    flex_wins: Option<i32>,
    flex_losses: Option<i32>,
}

pub async fn get_summoner_by_id(client: &Client, auth_info: &LcuAuthInfo, summoner_id: u64) -> Result<SummonerInfo, String> {
    println!("[LCU] 开始获取召唤师信息 (ID: {})...", summoner_id);

    let auth_string = format!("riot:{}", auth_info.remoting_auth_token);
    let base64_auth = general_purpose::STANDARD.encode(auth_string.as_bytes());

    let url = format!(
        "https://127.0.0.1:{}/lol-summoner/v1/summoners/{}",
        auth_info.app_port,
        summoner_id
    );
    println!("[LCU] 请求URL: {}", url);

    let response = client
        .get(&url)
        .header("Authorization", format!("Basic {}", base64_auth))
        .send()
        .await
        .map_err(|e| {
            println!("[LCU] ✗ 获取召唤师信息失败: {}", e);
            format!("获取召唤师信息失败: {}", e)
        })?;

    if !response.status().is_success() {
        let error_msg = format!("服务器返回错误: {}", response.status());
        println!("[LCU] ✗ {}", error_msg);
        return Err(error_msg);
    }

    match response.json::<SummonerInfo>().await {
        Ok(info) => {
            println!("[LCU] ✓ 成功获取召唤师信息: {}", info.displayName);
            Ok(info)
        }
        Err(e) => {
            println!("[LCU] ✗ 解析召唤师信息失败: {}", e);
            Err(format!("解析召唤师信息失败: {}", e))
        }
    }
}
