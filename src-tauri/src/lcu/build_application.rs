use crate::lcu::request::{lcu_get, lcu_post, lcu_delete, lcu_put};
use crate::lcu::types::{RunePage, CreateRunePageRequest, ItemSet, ItemBlock, RecommendedItem};
use reqwest::Client;
use serde_json::json;

/// 获取当前所有符文页面
pub async fn get_rune_pages(client: &Client) -> Result<Vec<RunePage>, String> {
    log::info!("🔧 开始获取符文页面列表");
    let result: Result<Vec<RunePage>, String> = lcu_get(client, "/lol-perks/v1/pages").await;
    match &result {
        Ok(pages) => log::info!("🔧 成功获取到 {} 个符文页面", pages.len()),
        Err(e) => log::error!("🔧 获取符文页面失败: {}", e),
    }
    result
}

/// 获取当前活跃的符文页面
pub async fn get_current_rune_page(client: &Client) -> Result<Option<RunePage>, String> {
    let pages: Vec<RunePage> = get_rune_pages(client).await?;
    Ok(pages.into_iter().find(|page| page.current))
}

/// 创建新的符文页面
pub async fn create_rune_page(
    client: &Client,
    name: &str,
    primary_style_id: i32,
    sub_style_id: i32,
    selected_perk_ids: Vec<i32>,
) -> Result<RunePage, String> {
    log::info!("🔧 开始创建符文页面: {}", name);
    log::info!("🔧 主系ID: {}, 副系ID: {}", primary_style_id, sub_style_id);
    log::info!("🔧 符文IDs: {:?}", selected_perk_ids);

    let request = CreateRunePageRequest {
        name: name.to_string(),
        primary_style_id,
        sub_style_id,
        selected_perk_ids,
    };

    let body = serde_json::to_value(request)
        .map_err(|e| format!("序列化创建符文页面请求失败: {}", e))?;

    log::info!("🔧 发送创建符文页面请求到: /lol-perks/v1/pages");
    let result: Result<RunePage, String> = lcu_post(client, "/lol-perks/v1/pages", body).await;
    match &result {
        Ok(page) => log::info!("🔧 成功创建符文页面: {}", page.name),
        Err(e) => log::error!("🔧 创建符文页面失败: {}", e),
    }
    result
}

/// 更新现有符文页面
pub async fn update_rune_page(
    client: &Client,
    page_id: i64,
    name: &str,
    primary_style_id: i32,
    sub_style_id: i32,
    selected_perk_ids: Vec<i32>,
) -> Result<RunePage, String> {
    let body = json!({
        "name": name,
        "primaryStyleId": primary_style_id,
        "subStyleId": sub_style_id,
        "selectedPerkIds": selected_perk_ids
    });

    let path = format!("/lol-perks/v1/pages/{}", page_id);
    lcu_put(client, &path, body).await
}

/// 删除指定的符文页面
pub async fn delete_rune_page(client: &Client, page_id: i64) -> Result<(), String> {
    log::info!("🔧 开始删除符文页面: {}", page_id);
    let result: Result<(), String> = lcu_delete(client, &format!("/lol-perks/v1/pages/{}", page_id)).await;
    match &result {
        Ok(_) => log::info!("🔧 成功删除符文页面: {}", page_id),
        Err(e) => log::error!("🔧 删除符文页面失败: {}", e),
    }
    result
}

/// 应用符文配置到游戏中
/// 采用先删除后创建的策略，确保符文页面正确应用
pub async fn apply_rune_build(
    client: &Client,
    champion_name: &str,
    primary_style_id: i32,
    sub_style_id: i32,
    selected_perk_ids: Vec<i32>,
) -> Result<String, String> {
    // 1. 获取当前所有符文页面
    let pages: Vec<RunePage> = get_rune_pages(client).await?;

    // 2. 找到要删除的符文页面ID
    let mut delete_id = 0i64;

    // 优先删除当前使用的符文页
    for page in &pages {
        if page.current {
            delete_id = page.id;
            break;
        }
    }

    // 如果当前页面不可删除，找一个可删除的页面
    if delete_id == 0 {
        for page in &pages {
            if page.is_deletable {
                delete_id = page.id;
                break;
            }
        }
    }

    // 3. 删除旧的符文页面
    if delete_id > 0 {
        if let Err(e) = delete_rune_page(client, delete_id).await {
            // 删除失败时记录但不阻止创建新页面
            log::warn!("删除符文页面 {} 失败: {}", delete_id, e);
        }
    }

    // 4. 创建新的符文页面
    let page_name = format!("Nidalee : {}", champion_name);
    let new_page = create_rune_page(
        client,
        &page_name,
        primary_style_id,
        sub_style_id,
        selected_perk_ids,
    ).await?;

    Ok(format!("成功创建符文页面: {}", new_page.name))
}

/// 获取当前英雄的装备推荐
pub async fn get_item_sets(client: &Client) -> Result<Vec<ItemSet>, String> {
    lcu_get(client, "/lol-item-sets/v1/item-sets").await
}

/// 创建装备推荐套装
pub async fn create_item_set(client: &Client, item_set: ItemSet) -> Result<(), String> {
    let body = serde_json::to_value(item_set)
        .map_err(|e| format!("序列化装备推荐失败: {}", e))?;

    lcu_post::<serde_json::Value>(client, "/lol-item-sets/v1/item-sets", body).await?;
    Ok(())
}

/// 应用装备推荐配置
pub async fn apply_item_build(
    client: &Client,
    champion_name: &str,
    starter_items: Vec<String>,
    core_items: Vec<String>,
) -> Result<String, String> {
    // 创建装备块
    let mut blocks = Vec::new();

    // 初始装备块
    if !starter_items.is_empty() {
        blocks.push(ItemBlock {
            block_type: "起手装备".to_string(),
            items: starter_items.into_iter().map(|id| RecommendedItem {
                id,
                count: 1,
            }).collect(),
        });
    }

    // 核心装备块
    if !core_items.is_empty() {
        blocks.push(ItemBlock {
            block_type: "核心装备".to_string(),
            items: core_items.into_iter().map(|id| RecommendedItem {
                id,
                count: 1,
            }).collect(),
        });
    }

    // 创建装备套装
    let item_set = ItemSet {
        title: format!("{} 推荐出装", champion_name),
        champion: champion_name.to_string(),
        mode: "any".to_string(),
        map: "any".to_string(),
        blocks,
    };

    create_item_set(client, item_set).await?;
    Ok(format!("成功创建 {} 的装备推荐", champion_name))
}
