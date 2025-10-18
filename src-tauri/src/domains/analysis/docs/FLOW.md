# 完整数据流程解析

## 🚀 **从 API 调用到最终显示的完整流程**

---

## 📊 **流程总览图**

```
┌────────────────────────────────────────────────────────────────────────┐
│                         用户操作 / 自动触发                              │
└───────────────────────────┬────────────────────────────────────────────┘
                            ↓
┌────────────────────────────────────────────────────────────────────────┐
│ 前端 (Vue)                                                              │
│ - useSummonerAndMatchUpdater.ts                                        │
│ - invoke('get_match_history', { count, queue_id })                    │
└───────────────────────────┬────────────────────────────────────────────┘
                            ↓
┌────────────────────────────────────────────────────────────────────────┐
│ Tauri Command Layer                                                     │
│ - matches/commands.rs                                                   │
│ - #[tauri::command] get_match_history()                                │
└───────────────────────────┬────────────────────────────────────────────┘
                            ↓
┌────────────────────────────────────────────────────────────────────────┐
│ Service Layer                                                           │
│ - matches/service.rs                                                    │
│ - get_match_history() → analyze_match_list_data()                      │
└───────────────────────────┬────────────────────────────────────────────┘
                            ↓
                ┌───────────────────────────┐
                │  分析引擎 (5步流程)        │
                └───────────┬───────────────┘
                            ↓
        ┌───────────────────────────────────────────┐
        │                                           │
        ↓                                           ↓
[Parser 层]                                  [Strategy 层]
解析原始 JSON                                 选择分析策略
        ↓                                           ↓
[Analyzer 层]                               [Traits 层]
计算量化指标                                 生成特征标签
        ↓                                           ↓
        └───────────────────┬───────────────────────┘
                            ↓
                    [Merger 层]
                    去重优化特征
                            ↓
┌────────────────────────────────────────────────────────────────────────┐
│ 返回数据: PlayerMatchStats                                              │
│ {                                                                       │
│   total_games, wins, win_rate,                                          │
│   avg_kda, dpm, cspm, vspm,                                            │
│   traits: [12个精炼特征],                                               │
│   favorite_champions: [...],                                            │
│   recent_performance: [...]                                             │
│ }                                                                       │
└───────────────────────────┬────────────────────────────────────────────┘
                            ↓
┌────────────────────────────────────────────────────────────────────────┐
│ 前端显示                                                                │
│ - Dashboard.vue                                                         │
│ - GameStats.vue (统计数据)                                              │
│ - SummonerTraits.vue (特征标签)                                         │
│ - SummonerCard.vue (召唤师卡片)                                         │
└────────────────────────────────────────────────────────────────────────┘
```

---

## 🔬 **详细分析流程（5步骤）**

### **第1步：Parser 层 - 数据解析** 🔧

**文件**: `parser.rs`

**职责**: 将 LCU API 的原始 JSON 解析为统一的内部数据结构

```rust
// 输入：LCU 原始 JSON 数组
let games = match_list_data["games"]["games"].as_array();

// 调用 Parser
let parsed_games = parse_games(games, current_puuid);

// 输出：ParsedGame 数组
struct ParsedGame {
    game_id: u64,
    queue_id: i64,
    game_duration: i32,
    game_creation: i64,
    player_data: ParsedPlayerData {
        win, kills, deaths, assists, kda,
        damage_to_champions, damage_taken, gold_earned,
        vision_score, cs, champion_id, role, lane, ...
    },
    team_data: ParsedTeamData {
        team_total_kills, team_total_damage,
        team_total_vision_score, ...
    }
}
```

**关键逻辑**:
1. 查找当前玩家的 `participant_id`
2. 提取玩家的详细数据
3. 计算队伍总数据（用于后续的占比计算）
4. 识别位置信息

**优势**:
- ✅ LCU API 变更时，只需修改这个文件
- ✅ 其他分析器无需关心 API 结构

---

### **第2步：Strategy 层 - 选择分析策略** 🎯

**文件**: `strategy.rs`

**职责**: 根据队列类型决定分析深度

```rust
// 根据队列ID选择策略
let strategy = if let Some(qid) = queue_id {
    AnalysisStrategy::from_queue_id(qid as i64)
} else {
    AnalysisStrategy::from_games(&parsed_games)
};

// 策略决定了分析深度
match strategy {
    Ranked => {
        // 5层完整分析
        enable_advanced_analysis: true,
        enable_role_analysis: true,
        enable_distribution_analysis: true,
        max_traits: 12
    },
    Other => {
        // 2层简化分析
        enable_advanced_analysis: false,
        enable_role_analysis: false,
        enable_distribution_analysis: false,
        max_traits: 6
    }
}
```

**决策逻辑**:
- **420/440** (单排/灵活排) → `Ranked` 策略
- **其他所有队列** → `Other` 策略

**输出日志**:
```
🎯 Strategy: 排位模式 - 核心深度分析 (queueId=420)
📊 分析方案: 5层完整分析（基础→深度→位置→分布→胜负）
```

---

### **第3步：Analyzer 层 - 量化计算** 📊

**文件**: `analyzer.rs` + `thresholds.rs`

**职责**: 计算所有量化指标

```rust
let mut player_stats = analyze_player_stats(&parsed_games, current_puuid, context);

// 输出: PlayerMatchStats
{
    // 基础统计
    total_games: 20,
    wins: 13,
    losses: 7,
    win_rate: 65.0,

    // KDA 统计
    avg_kills: 8.5,
    avg_deaths: 4.2,
    avg_assists: 12.3,
    avg_kda: 4.95,

    // 今日统计
    today_games: 5,
    today_wins: 4,

    // 衍生指标
    dpm: 750.5,    // 每分钟伤害
    cspm: 7.2,     // 每分钟补刀
    vspm: 1.8,     // 每分钟视野

    // 常用英雄
    favorite_champions: [
        { champion_id: 236, games: 8, wins: 6, win_rate: 75.0 }
    ],

    // 最近战绩
    recent_performance: [
        { game_id, win, champion_id, kills, deaths, assists, kda, ... }
    ],

    // 特征（稍后填充）
    traits: []
}
```

**计算过程**:
1. 根据 `context` 过滤对局（队列ID匹配）
2. 遍历所有对局累加数据
3. 计算平均值和衍生指标
4. 使用 `thresholds.rs` 的阈值标准

---

### **第4步：Traits 层 - 特征分析** 🏷️

**文件**: 5个分析器 + `thresholds.rs`

#### **4.1 基础特征（所有模式）** - `traits_analyzer.rs`

```rust
traits.extend(analyze_traits(&player_stats));

// 分析维度：
// 1. 胜率特征
if stats.win_rate >= thresholds::win_rate::EXCELLENT_RANKED {
    // "大神" (≥65%)
}

// 2. KDA特征
if stats.avg_kda >= thresholds::kda::EXCELLENT_RANKED {
    // "大爹" (≥4.0)
}

// 3. 连胜特征
if win_streak >= thresholds::streak::WIN_STREAK_GOOD {
    // "连胜王" (≥5连胜)
}

// 输出示例：
[
    { name: "大神", description: "胜率65%超高", score: 65, type: "good" },
    { name: "大爹", description: "KDA 4.5超高", score: 45, type: "good" }
]
```

#### **4.2 深度特征（仅排位）** - `advanced_analyzer.rs`

```rust
if strategy.enable_advanced_analysis() {
    traits.extend(analyze_advanced_traits(&player_stats, games, current_puuid));
}

// 分析维度：
// 1. 参团率 (KP%)
if avg_kp >= thresholds::kill_participation::HIGH {
    // "团战核心" (≥70%)
}

// 2. 伤害占比
if avg_damage_share >= thresholds::damage_share::HIGH {
    // "输出核心" (≥30%)
}

// 3. 稳定性分析
let kda_variance = calculate_variance(&kda_list);
if kda_variance <= thresholds::stability::KDA_VARIANCE_STABLE {
    // "稳定发挥" (方差≤1.0)
}

// 4. 趋势分析
let (early_wr, recent_wr) = calculate_trend(&games);
if recent_wr > early_wr + 10.0 {
    // "状态回升" (近期胜率提升)
}

// 输出示例：
[
    { name: "团战核心", description: "参团率73%极高", score: 73, type: "good" },
    { name: "输出核心", description: "伤害占比32%", score: 32, type: "good" }
]
```

#### **4.3 位置特征（仅排位）** - `role_analyzer.rs`

```rust
if strategy.enable_role_analysis() {
    let role_stats_map = identify_player_roles(games, current_puuid);
    traits.extend(analyze_role_based_traits(&player_stats, &role_stats_map));
}

// 分析维度：
// 1. 位置专精
if role_stats.win_rate >= thresholds::win_rate::EXCELLENT_OTHER {
    // "ADC专精" (该位置胜率≥60%)
}

// 2. 全能选手
if role_count >= 3 && stats.win_rate >= thresholds::win_rate::GOOD {
    // "全能选手" (能胜任3+位置)
}

// 输出示例：
[
    { name: "ADC专精", description: "打ADC胜率68%", score: 68, type: "good" }
]
```

#### **4.4 分布特征（仅排位）** - `distribution_analyzer.rs`

```rust
if strategy.enable_distribution_analysis() {
    traits.extend(analyze_distribution_traits(&player_stats.recent_performance));
}

// 分析维度：
// 1. 高光时刻
if s_count >= 3 {  // KDA ≥ thresholds::kda::S_GRADE (6.0)
    // "高光时刻" (3+场超神)
}

// 2. 稳定优秀
if excellent_rate >= 0.50 {
    // "稳定优秀" (50%以上KDA≥4.0)
}

// 3. 两极分化
if s_count >= 3 && d_count >= 3 {
    // "两极分化" (既有超神又有崩盘)
}

// 输出示例：
[
    { name: "高光时刻", description: "20场中5场超神", score: 5, type: "good" },
    { name: "稳定优秀", description: "60%对局KDA>4.0", score: 60, type: "good" }
]
```

#### **4.5 胜负模式（所有模式）** - `distribution_analyzer.rs`

```rust
traits.extend(analyze_win_loss_pattern(&player_stats.recent_performance));

// 分析维度：
// 1. 近期火热 (最近5场)
if wins_5 >= 4 {
    // "近期火热" (5场4胜+)
}

// 2. 近期强势 (最近10场)
if wins_10 >= 8 {
    // "近期强势" (10场8胜+)
}

// 输出示例：
[
    { name: "近期火热", description: "最近5场4胜", score: 4, type: "good" }
]
```

---

### **第5步：Merger 层 - 去重优化** ✨

**文件**: `trait_merger.rs`

**职责**: 合并相似特征，限制数量

```rust
let mut optimized_traits = optimize_traits(traits);
optimized_traits.truncate(strategy.max_traits());

// 去重逻辑：
// 1. 相似特征合并（如"输出核心"+"主要输出"→保留"输出核心"）
// 2. 按得分排序（从高到低）
// 3. 限制数量：
//    - 排位模式：12个
//    - 其他模式：6个

// 输入：可能有 20+ 个特征
[
    { name: "大神", score: 65 },
    { name: "大爹", score: 45 },
    { name: "团战核心", score: 73 },
    { name: "积极参团", score: 60 },  // ← 与"团战核心"相似，可能被合并
    { name: "输出核心", score: 32 },
    ...
]

// 输出：精炼后 12个（排位）或 6个（其他）
[
    { name: "团战核心", score: 73 },
    { name: "大神", score: 65 },
    { name: "大爹", score: 45 },
    { name: "输出核心", score: 32 },
    ...
]
```

---

## 🎨 **阈值配置的使用**

**文件**: `thresholds.rs`

所有分析器统一使用 `thresholds.rs` 的阈值配置：

```rust
// 示例：胜率判断
use super::thresholds;

if stats.win_rate >= thresholds::win_rate::EXCELLENT_RANKED {
    // 排位大神标准（65%）
} else if stats.win_rate >= thresholds::win_rate::EXCELLENT_OTHER {
    // 其他模式大神标准（60%）
}

// 可用的阈值模块：
// - thresholds::win_rate::*
// - thresholds::kda::*
// - thresholds::kill_participation::*
// - thresholds::damage_share::*
// - thresholds::vision::*
// - thresholds::streak::*
// - thresholds::stability::*
// - thresholds::distribution::*
```

**优势**:
- ✅ 统一管理，一处修改全局生效
- ✅ 易于实验不同阈值
- ✅ 支持位置特定阈值

---

## 📈 **实际数据流示例**

### **场景：查看排位赛战绩**

```
1. 用户操作
   - 前端选择"单双排位"队列（queueId=420）
   - 点击刷新战绩

2. 前端调用
   fetchMatchHistory(420)
   ↓
   invoke('get_match_history', { count: 20, queue_id: 420 })

3. 后端处理
   ┌─────────────────────────────────────────────┐
   │ matches/commands.rs                          │
   │ get_match_history(count, queue_id)          │
   └──────────────┬──────────────────────────────┘
                  ↓
   ┌─────────────────────────────────────────────┐
   │ matches/service.rs                           │
   │ get_match_history(client, 20, Some(420))    │
   │ → analyze_match_list_data()                 │
   └──────────────┬──────────────────────────────┘
                  ↓
   ┌──────────────────────────────────────┐
   │ 第1步: Parser                        │
   │ parse_games(games, puuid)           │
   │ → 解析20场对局数据                   │
   └──────────────┬───────────────────────┘
                  ↓
   ┌──────────────────────────────────────┐
   │ 第2步: Strategy                      │
   │ AnalysisStrategy::from_queue_id(420) │
   │ → Ranked 策略                        │
   │ → 启用5层完整分析                    │
   └──────────────┬───────────────────────┘
                  ↓
   ┌──────────────────────────────────────┐
   │ 第3步: Analyzer                      │
   │ analyze_player_stats()              │
   │ → 计算: 胜率65%, KDA 4.5, ...       │
   └──────────────┬───────────────────────┘
                  ↓
   ┌──────────────────────────────────────┐
   │ 第4步: Traits (5层)                  │
   │ Layer 1: analyze_traits()           │
   │   → "大神"(65%), "大爹"(4.5 KDA)    │
   │ Layer 2: analyze_advanced_traits()  │
   │   → "团战核心"(73% KP)              │
   │ Layer 3: analyze_role_based_traits() │
   │   → "ADC专精"(68%)                  │
   │ Layer 4: analyze_distribution()     │
   │   → "高光时刻"(5场超神)              │
   │ Layer 5: analyze_win_loss_pattern() │
   │   → "近期火热"(5场4胜)               │
   │ → 共生成 18个特征                    │
   └──────────────┬───────────────────────┘
                  ↓
   ┌──────────────────────────────────────┐
   │ 第5步: Merger                        │
   │ optimize_traits()                   │
   │ → 去重、排序、限制12个               │
   └──────────────┬───────────────────────┘
                  ↓
   ┌──────────────────────────────────────┐
   │ 返回: PlayerMatchStats               │
   │ {                                    │
   │   total_games: 20,                   │
   │   wins: 13, win_rate: 65.0,          │
   │   avg_kda: 4.5,                      │
   │   dpm: 750, cspm: 7.2, vspm: 1.8,   │
   │   traits: [12个精炼特征],             │
   │   favorite_champions: [...],         │
   │   recent_performance: [...]          │
   │ }                                    │
   └──────────────┬───────────────────────┘
                  ↓
4. 前端显示
   Dashboard.vue
   ├── GameStats.vue
   │   ├── 统计数据：20场 13胜 65%
   │   ├── KDA: 8.5 / 4.2 / 12.3
   │   └── 衍生指标: DPM 750, CSPM 7.2
   ├── SummonerTraits.vue
   │   └── 展示12个特征标签
   └── 最近战绩列表
```

---

## 🔄 **对比：排位 vs 其他模式**

### **排位模式（420/440）**

```
📊 分析方案: 5层完整分析
┌─────────────────────────────────────┐
│ Layer 1: 基础特征 ✅                 │
│ - 胜率、KDA、连胜                    │
├─────────────────────────────────────┤
│ Layer 2: 深度特征 ✅                 │
│ - 参团率、伤害占比、稳定性、趋势     │
├─────────────────────────────────────┤
│ Layer 3: 位置特征 ✅                 │
│ - 位置识别、专精分析                 │
├─────────────────────────────────────┤
│ Layer 4: 分布特征 ✅                 │
│ - 高光时刻、崩盘分析                 │
├─────────────────────────────────────┤
│ Layer 5: 胜负模式 ✅                 │
│ - 近期状态                           │
└─────────────────────────────────────┘
✨ 输出: 12个精炼特征
⚡ 计算开销: 高
🎯 目标: 深度洞察
```

### **其他模式（ARAM、匹配等）**

```
📊 分析方案: 2层简化分析
┌─────────────────────────────────────┐
│ Layer 1: 基础特征 ✅                 │
│ - 胜率、KDA、连胜                    │
├─────────────────────────────────────┤
│ Layer 2: 深度特征 ❌ 跳过             │
├─────────────────────────────────────┤
│ Layer 3: 位置特征 ❌ 跳过             │
├─────────────────────────────────────┤
│ Layer 4: 分布特征 ❌ 跳过             │
├─────────────────────────────────────┤
│ Layer 5: 胜负模式 ✅                 │
│ - 近期状态                           │
└─────────────────────────────────────┘
✨ 输出: 6个精炼特征
⚡ 计算开销: 低
🎯 目标: 快速展示
```

---

## 🎯 **关键决策点**

### **1. 何时使用 Parser？**
- ✅ 每次接收 LCU API 数据时
- ✅ 在任何分析之前

### **2. 何时选择 Strategy？**
- ✅ 根据 `queue_id` 自动选择
- ✅ 420/440 → Ranked
- ✅ 其他 → Other

### **3. 哪些分析器会执行？**
```rust
// 所有模式
✅ analyze_traits()           // 基础特征
✅ analyze_win_loss_pattern() // 胜负模式

// 仅排位模式
🔒 analyze_advanced_traits()     // 深度特征
🔒 analyze_role_based_traits()   // 位置特征
🔒 analyze_distribution_traits() // 分布特征
```

### **4. 如何使用阈值？**
```rust
// 统一使用 thresholds 模块
use super::thresholds;

if value >= thresholds::module::THRESHOLD {
    // 判断逻辑
}
```

---

## 📝 **总结**

### **核心优势**
1. **Parser 隔离** - LCU API 变更影响最小化
2. **Strategy 决策** - 自动选择分析深度
3. **Thresholds 统一** - 阈值集中管理
4. **5层分析** - 从基础到深度逐层递进
5. **智能去重** - 输出精炼特征

### **数据流**
```
LCU JSON → Parser → Strategy → Analyzer → [Traits×5] → Merger → 前端显示
```

### **灵活性**
- ✅ 修改阈值 → 只改 `thresholds.rs`
- ✅ 新增分析 → 新建分析器
- ✅ 调整策略 → 只改 `strategy.rs`
- ✅ API变更 → 只改 `parser.rs`

---

*完整流程解析 - 2025-10-17*

