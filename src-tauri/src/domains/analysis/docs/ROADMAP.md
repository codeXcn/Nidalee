# 完整实现路线图

## 🎯 **从数据分析到智能助手的进化**

```
阶段0：现状（已完成）✅
    ↓
阶段1：时间线分析 📊
    ↓
阶段2：智能建议系统 💡
    ↓
阶段3：三维战术系统 🎯
```

---

## ✅ **阶段0：现状（已完成）**

### **已完成的功能**

```
✅ Parser 层：隔离 LCU API 变化
✅ Strategy 层：决定分析深度（排位 vs 其他）
✅ Thresholds 模块：统一阈值管理
✅ 5层特征分析：
   1. 基础特征（胜率、KDA、连胜）
   2. 深度特征（参团率、伤害占比、稳定性）
   3. 位置特征（位置专精、全能选手）
   4. 分布特征（高光时刻、崩盘场次）
   5. 胜负模式（近期状态）
✅ 智能去重：输出 12/6 个精炼特征
```

**输出**：`PlayerMatchStats` + 特征标签

**问题**：只有"诊断"，没有"建议"

---

## 📊 **阶段1：时间线分析**

### **目标**

从"全场统计"升级到"分阶段分析"

### **实现内容**

#### **1.1 扩展 Parser（1-2小时）**

```rust
// parser.rs

pub struct TimelineData {
    // 对线期 (0-10分钟)
    pub cs_per_min_0_10: Option<f64>,
    pub gold_per_min_0_10: Option<f64>,
    pub xp_per_min_0_10: Option<f64>,
    pub cs_diff_0_10: Option<f64>,        // ⭐ 关键
    pub xp_diff_0_10: Option<f64>,        // ⭐ 关键

    // 发育期 (10-20分钟)
    pub cs_per_min_10_20: Option<f64>,
    pub gold_per_min_10_20: Option<f64>,
    pub cs_diff_10_20: Option<f64>,

    // 团战期 (20分钟+)
    pub cs_per_min_20_end: Option<f64>,
    pub gold_per_min_20_end: Option<f64>,
}

pub struct ParsedPlayerData {
    // ... 现有字段 ...
    pub timeline_data: Option<TimelineData>,  // 新增
}

// 解析 timeline 数据
fn parse_timeline_data(timeline: &Value) -> Option<TimelineData> {
    // 解析 creepsPerMinDeltas
    // 解析 goldPerMinDeltas
    // 解析 csDiffPerMinDeltas ⭐
    // 解析 xpDiffPerMinDeltas ⭐
    // ...
}
```

#### **1.2 创建时间线分析器（1-2小时）**

```rust
// timeline_analyzer.rs

/// 时间线特征分析
pub fn analyze_timeline_traits(
    games: &[ParsedGame],
    role: &str
) -> Vec<SummonerTrait> {
    let mut traits = Vec::new();

    // 对线期分析
    traits.extend(analyze_laning_phase(games, role));
    // → "对线压制"（CS差>+15）
    // → "对线弱势"（CS差<-15）

    // 发育曲线分析
    traits.extend(analyze_growth_curve(games));
    // → "爆发成长"（中期经济提升）
    // → "稳定发育"（各阶段均衡）

    traits
}
```

#### **1.3 集成到主流程（0.5小时）**

```rust
// matches/service.rs

if strategy.enable_advanced_analysis() {
    // ... 现有分析 ...

    // 新增：时间线分析
    traits.extend(analyze_timeline_traits(&parsed_games, &main_role));
}
```

**工作量**：2.5-4.5 小时
**收益**：⭐⭐⭐⭐⭐

---

## 💡 **阶段2：智能建议系统**

### **目标**

从"识别问题"升级到"提供解决方案"

### **实现内容**

#### **2.1 定义数据结构（0.5小时）**

```rust
// types.rs

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct GameAdvice {
    pub title: String,
    pub problem: String,
    pub evidence: String,
    pub suggestions: Vec<String>,
    pub priority: u8,
    pub category: AdviceCategory,
    pub perspective: AdvicePerspective,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum AdviceCategory {
    Laning, Farming, Teamfight, Vision, Positioning, Decision, Champion
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum AdvicePerspective {
    SelfImprovement,  // 对自己
    Targeting,        // 对敌人
    Collaboration,    // 对队友 ⭐ NEW
}

// 扩展 PlayerMatchStats
pub struct PlayerMatchStats {
    // ... 现有字段 ...
    pub advice: Vec<GameAdvice>,  // 新增
}
```

#### **2.2 实现建议分析器（3-4小时）**

```rust
// advice/
├── mod.rs
├── advice_generator.rs       // 主生成器
├── laning_advice.rs          // 对线期建议
├── farming_advice.rs         // 发育建议
├── teamfight_advice.rs       // 团战建议
├── vision_advice.rs          // 视野建议
└── champion_advice.rs        // 英雄池建议

// advice_generator.rs
pub fn generate_advice(
    stats: &PlayerMatchStats,
    games: &[ParsedGame],
    role: &str,
    perspective: AdvicePerspective,
) -> Vec<GameAdvice> {
    let mut advice = Vec::new();

    // 1. 对线期建议
    advice.extend(analyze_laning_advice(stats, games, role, perspective));

    // 2. 发育建议
    advice.extend(analyze_farming_advice(stats, games, role, perspective));

    // 3. 团战建议
    advice.extend(analyze_teamfight_advice(stats, perspective));

    // 4. 视野建议
    advice.extend(analyze_vision_advice(stats, role, perspective));

    // 5. 英雄池建议
    advice.extend(analyze_champion_pool_advice(stats, perspective));

    // 按优先级排序，限制5条
    advice.sort_by_key(|a| std::cmp::Reverse(a.priority));
    advice.truncate(5);

    advice
}
```

#### **2.3 集成到主流程（0.5小时）**

```rust
// matches/service.rs

// 生成建议（仅排位模式）
if strategy == AnalysisStrategy::Ranked {
    player_stats.advice = generate_advice(
        &player_stats,
        &parsed_games,
        &main_role,
        AdvicePerspective::SelfImprovement  // Dashboard用
    );
}
```

#### **2.4 前端组件（2-3小时）**

```vue
// AdvicePanel.vue
// AdviceCard.vue
// 在 Dashboard.vue 中集成
```

**工作量**：6-8 小时
**收益**：⭐⭐⭐⭐⭐

---

## 🎯 **阶段3：三维战术系统**

### **目标**

从"个人工具"升级到"团队教练"

### **实现内容**

#### **3.1 团队分析数据结构（1小时）**

```rust
// types.rs

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct TeammateAnalysis {
    pub summoner_name: String,
    pub role: String,
    pub power_level: TeammatePowerLevel,
    pub win_rate: f64,
    pub avg_kda: f64,
    pub strengths: Vec<String>,
    pub weaknesses: Vec<String>,
    pub reliability: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct TeamTacticalAnalysis {
    pub teammates: Vec<TeammateAnalysis>,
    pub enemies: Vec<TeammateAnalysis>,
    pub strongest_lane: String,
    pub weakest_lane: String,
    pub enemy_strongest: String,
    pub enemy_weakest: String,
    pub tactical_advice: Vec<TacticalAdvice>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct TacticalAdvice {
    pub target_role: String,       // "打野"、"全队"
    pub priority: String,           // "主打上路"
    pub suggestions: Vec<String>,
}
```

#### **3.2 实现团队战术分析器（3-4小时）**

```rust
// team_tactical_analyzer.rs

/// 分析团队战术
pub fn analyze_team_tactics(
    teammates: &[PlayerMatchStats],
    enemies: &[PlayerMatchStats],
) -> TeamTacticalAnalysis {
    // 1. 评估我方实力
    let our_power_map = evaluate_team_power(teammates);

    // 2. 评估敌方实力
    let enemy_power_map = evaluate_team_power(enemies);

    // 3. 找出最强/最弱路
    let strongest_lane = find_strongest_lane(&our_power_map);
    let weakest_lane = find_weakest_lane(&our_power_map);
    let enemy_strongest = find_strongest_lane(&enemy_power_map);
    let enemy_weakest = find_weakest_lane(&enemy_power_map);

    // 4. 生成战术建议
    let tactical_advice = generate_tactical_advice(
        &our_power_map,
        &enemy_power_map,
        &strongest_lane,
        &weakest_lane,
        &enemy_weakest,
    );

    TeamTacticalAnalysis {
        teammates,
        enemies,
        strongest_lane,
        weakest_lane,
        enemy_strongest,
        enemy_weakest,
        tactical_advice,
    }
}

/// 生成战术建议
fn generate_tactical_advice(...) -> Vec<TacticalAdvice> {
    let mut advice = Vec::new();

    // 给打野的建议
    advice.push(TacticalAdvice {
        target_role: "打野".to_string(),
        priority: if our_strongest_lane_power > enemy_strongest {
            format!("主打{}路，扩大优势", strongest_lane)
        } else {
            format!("主打敌方{}路，找突破口", enemy_weakest)
        },
        suggestions: vec![
            format!("前期重点照顾{}路", ...),
            "配合拿资源...",
        ],
    });

    // 给最弱路的建议
    advice.push(TacticalAdvice {
        target_role: weakest_lane.to_string(),
        priority: "求稳，避免崩盘".to_string(),
        suggestions: vec![
            "对线保守，塔下补刀",
            "主动要求打野反蹲",
            "宁愿漏刀，不要被击杀",
        ],
    });

    // 给团队的建议
    advice.push(TacticalAdvice {
        target_role: "全队".to_string(),
        priority: decide_strategy(...),
        suggestions: generate_team_strategy(...),
    });

    advice
}
```

#### **3.3 前端战术面板（3-4小时）**

```vue
// TeamTacticsPanel.vue
// RolePowerBar.vue
// TacticalAdviceCard.vue
```

#### **3.4 集成到对局分析（1小时）**

```rust
// analysis_data/service.rs

// 收集队友和敌方数据后
let team_tactics = analyze_team_tactics(&teammates, &enemies);

// 返回给前端
team_analysis.tactical_advice = team_tactics;
```

**工作量**：8-10 小时
**收益**：⭐⭐⭐⭐⭐

---

## 📊 **完整系统架构图**

```
┌─────────────────────────────────────────────────────────┐
│                   LCU API 数据                           │
└────────────────────┬────────────────────────────────────┘
                     ↓
┌─────────────────────────────────────────────────────────┐
│ Parser 层：解析为 ParsedGame                             │
│ - 基础数据：kills, deaths, assists, ...                 │
│ - 时间线数据：cs_diff, gold_per_min, ... ⭐ 阶段1       │
└────────────────────┬────────────────────────────────────┘
                     ↓
┌─────────────────────────────────────────────────────────┐
│ Strategy 层：选择分析深度                                │
│ - Ranked: 深度分析                                      │
│ - Other: 简化分析                                       │
└────────────────────┬────────────────────────────────────┘
                     ↓
┌─────────────────────────────────────────────────────────┐
│ Analyzer 层：计算量化指标                                │
│ - 基础统计、KDA、衍生指标                                │
│ - 使用 thresholds.rs 阈值                               │
└────────────────────┬────────────────────────────────────┘
                     ↓
┌─────────────────────────────────────────────────────────┐
│ Traits 层：生成特征标签                                  │
│ - 5层分析（基础→深度→位置→分布→胜负）                   │
│ - 时间线特征 ⭐ 阶段1                                    │
└────────────────────┬────────────────────────────────────┘
                     ↓
┌─────────────────────────────────────────────────────────┐
│ Advice 层：生成智能建议 ⭐ 阶段2                         │
│ - 基于特征和时间线数据                                   │
│ - 三种视角：改进/协作/针对                               │
└────────────────────┬────────────────────────────────────┘
                     ↓
┌─────────────────────────────────────────────────────────┐
│ Team Tactical 层：团队战术分析 ⭐ 阶段3                  │
│ - 评估我方/敌方实力                                      │
│ - 决定资源分配                                           │
│ - 生成战术建议                                           │
└────────────────────┬────────────────────────────────────┘
                     ↓
┌─────────────────────────────────────────────────────────┐
│ 最终输出：                                               │
│ - PlayerMatchStats（个人）                               │
│   ├─ 统计数据                                            │
│   ├─ 特征标签                                            │
│   └─ 个人建议 ⭐                                         │
│                                                          │
│ - TeamTacticalAnalysis（团队）⭐                         │
│   ├─ 队友分析                                            │
│   ├─ 敌方分析                                            │
│   └─ 战术建议                                            │
└─────────────────────────────────────────────────────────┘
```

---

## 🎮 **完整应用场景**

### **场景1：个人提升（Dashboard）**

```
用户操作：
1. 打开 Dashboard
2. 查看个人战绩

系统分析：
1. 基础统计：20场 13胜 65%
2. 特征标签：大神、上单专精、对线压制 ⭐
3. 个人建议：
   💡 视野控制需加强
   💡 中期发育节奏需优化

用户收获：
✅ 知道自己强在哪里
✅ 知道自己弱在哪里
✅ 知道如何改进
```

### **场景2：对局准备（Match Analysis）**

```
用户操作：
1. 进入游戏大厅
2. 查看10名玩家

系统分析：
┌──────────────────────────────────────┐
│ 我方实力：                           │
│ 上单 ████████░░ 80% 强               │
│ 打野 ████░░░░░░ 40% 弱               │
│ 中单 █████░░░░░ 50% 一般             │
│ ADC  ███░░░░░░░ 30% 很弱             │
│ 辅助 ████░░░░░░ 40% 弱               │
│                                      │
│ 敌方实力：                           │
│ 上单 ███░░░░░░░ 30% 弱               │
│ 打野 ██████░░░░ 60% 一般             │
│ 中单 █████░░░░░ 50% 一般             │
│ ADC  ████████░░ 80% 强               │
│ 辅助 ██████░░░░ 60% 一般             │
└──────────────────────────────────────┘

战术建议：
┌──────────────────────────────────────┐
│ 💡 核心战术：主打上路！              │
├──────────────────────────────────────┤
│ 【给上单】：                         │
│ 💪 你是本局carry点！                 │
│ - 前期主动换血，压制对面             │
│ - 配合打野击杀                       │
│ - 拿到优势后多TP支援                 │
│                                      │
│ 【给打野】：                         │
│ 🎯 前期住上路！                      │
│ - 3级/6级抓上路                      │
│ - 配合上单拿峡谷                     │
│ - 下路适当反蹲，别让ADC崩            │
│                                      │
│ 【给ADC/辅助】：                     │
│ 🛡️ 求稳，等上单带飞！               │
│ - 对线猥琐，不要激进                 │
│ - 主动要求打野反蹲                   │
│ - 发育到2件套就行                    │
│                                      │
│ 【团队整体】：                       │
│ ⚔️ 前期主打上半区！                 │
│ - 峡谷先锋给上单                     │
│ - 下路龙可以让                       │
│ - 团战围绕上单打                     │
│ - 保护ADC别死，稳定输出              │
└──────────────────────────────────────┘

用户收获：
✅ 知道我方哪路强弱
✅ 知道敌方哪路强弱
✅ 知道该打哪里
✅ 知道该保护谁
✅ 清晰的战术方案
```

---

## 🎯 **三维建议对比**

### **同样的数据，三种视角**

#### **数据：上单胜率68%，对线压制力强**

##### **视角1：对自己（改进）**
```
💡 你的优势：
- 对线压制力强（前10分钟领先15刀）
- 建议：继续发挥优势，多练习carry型英雄
```

##### **视角2：对队友（协作）⭐**
```
🤝 如何配合上单：
【给打野】：
- 前期多帮上路，让上单滚雪球
- 3-6级抓上路2-3次

【给中单】：
- 6级后游走上路，配合越塔
- 峡谷先锋给上单

【给ADC/辅助】：
- 前期求稳，等上单建立优势
- 团战保护上单进场

→ 目标：让上单成为队伍carry点
```

##### **视角3：对敌人（针对）**
```
🎯 针对敌方上单：
- 对手上单较弱（胜率48%，容易被压制）
- 建议：
  • 选择压制型上单
  • 打野重点gank上路
  • 前期主打上路突破口

→ 目标：针对敌方弱点
```

---

## 📊 **团队战术决策树**

```
开始对局分析
    ↓
评估10名玩家实力
    ↓
识别我方最强路（A）和最弱路（B）
识别敌方最强路（C）和最弱路（D）
    ↓
┌─────────────────────────────────────────────┐
│ 决策1：核心战术方向                         │
├─────────────────────────────────────────────┤
│ IF 我方A路 > 敌方A路：                      │
│    → 核心战术：主打A路，扩大优势            │
│    → 资源倾斜：峡谷/小龙 优先给A路          │
│    → 打野重点：前期住A路                    │
│                                             │
│ IF 我方A路 = 敌方A路 但 我方A > 敌方D：     │
│    → 核心战术：A路打敌方D路，错位优势       │
│    → 资源倾斜：帮A路压制D路                 │
│                                             │
│ IF 我方整体弱于敌方：                       │
│    → 核心战术：打运营，避免正面团           │
│    → 资源倾斜：保护发育，拖后期             │
└─────────────────────────────────────────────┘
    ↓
┌─────────────────────────────────────────────┐
│ 决策2：弱势路保护方案                       │
├─────────────────────────────────────────────┤
│ IF 我方B路 vs 敌方C路（弱打强）：          │
│    → B路求稳，不要崩盘                      │
│    → 打野多反蹲B路                          │
│    → 队友不要期待B路carry                   │
│    → 选择稳健型英雄                         │
└─────────────────────────────────────────────┘
    ↓
┌─────────────────────────────────────────────┐
│ 决策3：针对敌方弱点                         │
├─────────────────────────────────────────────┤
│ 敌方D路有明显弱点：                         │
│    → 打野/中单频繁gank D路                  │
│    → 选择克制型英雄                         │
│    → 前期主打D路突破口                      │
└─────────────────────────────────────────────┘
    ↓
生成给每个位置的具体建议
```

---

## 🎮 **前端显示效果**

### **战术总览面板**

```vue
<template>
  <div class="team-tactics-panel">
    <!-- 标题 -->
    <div class="panel-header">
      <h2>🎯 团队战术分析</h2>
      <p class="subtitle">基于双方实力对比的智能战术建议</p>
    </div>

    <!-- 实力对比可视化 -->
    <div class="power-comparison">
      <div class="team-section our-team">
        <h3>🔵 我方</h3>
        <RolePowerBar
          v-for="player in ourTeam"
          :key="player.summonerName"
          :player="player"
          :highlight="player.role === strongestLane || player.role === weakestLane"
        />
        <div class="summary">
          <span class="strongest">最强：{{ strongestLane }}</span>
          <span class="weakest">最弱：{{ weakestLane }}</span>
        </div>
      </div>

      <div class="team-section enemy-team">
        <h3>🔴 敌方</h3>
        <RolePowerBar
          v-for="player in enemies"
          :key="player.summonerName"
          :player="player"
          :highlight="player.role === enemyStrongest || player.role === enemyWeakest"
        />
        <div class="summary">
          <span class="strongest">最强：{{ enemyStrongest }}</span>
          <span class="weakest">最弱：{{ enemyWeakest }}</span>
        </div>
      </div>
    </div>

    <!-- 核心战术 -->
    <div class="core-strategy">
      <h3>💡 核心战术</h3>
      <div class="strategy-card highlight">
        <div class="strategy-title">{{ coreStrategy }}</div>
        <div class="strategy-reason">{{ strategyReason }}</div>
      </div>
    </div>

    <!-- 分位置建议 -->
    <div class="role-advice-section">
      <h3>📋 分位置建议</h3>

      <div
        v-for="advice in tacticalAdvice"
        :key="advice.targetRole"
        class="role-advice-card"
      >
        <div class="role-header">
          <span class="role-icon">{{ getRoleIcon(advice.targetRole) }}</span>
          <span class="role-name">【给{{ advice.targetRole }}】</span>
        </div>
        <div class="priority-badge">{{ advice.priority }}</div>
        <ul class="suggestions">
          <li v-for="(suggestion, i) in advice.suggestions" :key="i">
            {{ suggestion }}
          </li>
        </ul>
      </div>
    </div>
  </div>
</template>
```

---

## 💡 **实际效果示例**

### **示例1：我方上单强，ADC弱，敌方整体均衡**

```
🎯 核心战术：主打上路，扩大优势！

实力对比：
我方 上单 ████████░░ 80% ✅ 最强
敌方 上单 ████░░░░░░ 40%

我方 ADC  ███░░░░░░░ 30% ⚠️ 最弱
敌方 ADC  ██████░░░░ 60%

战术建议：

【给打野】🎯 前期住上路！
1. 2-3级就去上路gank一次
2. 配合上单击杀敌方上单
3. 拿峡谷先锋给上路推塔
4. 下路适当反蹲，保护ADC不崩
→ 让上单成为雪球点

【给上单】💪 你是carry点！
1. 前期主动换血，压制对方
2. 配合打野击杀
3. 拿到优势后多TP支援
4. 团战要进场切后排
→ 建立优势，带动全队

【给ADC】🛡️ 求稳，别崩！
1. 对线保守，塔下补刀
2. 宁愿漏刀，不要死
3. 发育到2件套就行
4. 团战跟着上单打
→ 不崩盘，等上单带飞

【给中单】🎯 游走上路！
1. 6级后优先游走上路
2. 配合上单扩大优势
3. 后期团战保护ADC
→ 帮上单滚雪球

【给辅助】🛡️ 保护ADC！
1. 对线期多做视野
2. 帮ADC控线，安全补刀
3. 不要随意开团
4. 团战保护ADC输出
→ 让ADC稳住

【团队整体】⚔️ 前期主打上半区！
1. 峡谷先锋给上单
2. 下路小龙可以让
3. 团战围绕上单打
4. 上单进场，ADC跟输出
→ 以强打弱，滚雪球
```

---

### **示例2：我方整体弱，需要运营**

```
🎯 核心战术：打运营，拖后期！

实力对比：
我方平均实力：45%
敌方平均实力：58%

我方唯一优势：中单（胜率66%）

战术建议：

【核心战术】避免正面，打运营！

【给全队】⚠️ 前期不要团！
1. 前15分钟避免5v5团战（会输）
2. 猥琐发育，不要主动开团
3. 利用中单优势抓单
4. 拖到后期，等装备

【给中单】💪 你是唯一希望！
1. 对线压制，建立优势
2. 游走抓边路，帮队友缓解压力
3. 不要暴毙，你死了就输了
4. 团战要carry

【给打野】🎯 配合中单！
1. 常驻中路，反蹲保护中单
2. 配合中单抓边路
3. 不要贪，跟着中单走

【给其他路】🛡️ 求稳！
1. 对线不激进
2. 猥琐发育
3. 配合中单抓人
4. 团战保护中单

【资源控制】
1. 小龙可以让（别送）
2. 大龙不要开（会被团灭）
3. 利用中单优势打分带
4. 后期等对面失误

→ 目标：拖到后期，打运营
```

---

## 🎯 **数据结构完整定义**

```rust
// types.rs

/// 战术建议（给特定位置）
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../src/types/generated/TacticalAdvice.ts")]
#[serde(rename_all = "camelCase")]
pub struct TacticalAdvice {
    /// 目标角色："打野"、"上单"、"全队"
    pub target_role: String,

    /// 核心优先级："主打上路"、"求稳不崩"
    pub priority: String,

    /// 具体建议列表
    pub suggestions: Vec<String>,

    /// 重要性（1-5）
    pub importance: u8,

    /// 图标
    pub icon: String,  // "🎯", "🛡️", "💪"
}

/// 团队战术分析
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../src/types/generated/TeamTacticalAnalysis.ts")]
#[serde(rename_all = "camelCase")]
pub struct TeamTacticalAnalysis {
    /// 我方队员分析
    pub teammates: Vec<TeammateAnalysis>,

    /// 敌方队员分析
    pub enemies: Vec<TeammateAnalysis>,

    /// 我方最强路
    pub our_strongest_lane: String,

    /// 我方最弱路
    pub our_weakest_lane: String,

    /// 敌方最强路
    pub enemy_strongest_lane: String,

    /// 敌方最弱路
    pub enemy_weakest_lane: String,

    /// 核心战术
    pub core_strategy: String,

    /// 战术原因
    pub strategy_reason: String,

    /// 分位置战术建议
    pub tactical_advice: Vec<TacticalAdvice>,
}

/// 队友/敌人分析
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../src/types/generated/TeammateAnalysis.ts")]
#[serde(rename_all = "camelCase")]
pub struct TeammateAnalysis {
    pub summoner_name: String,
    pub role: String,
    pub champion_id: Option<i32>,

    /// 实力等级
    pub power_level: TeammatePowerLevel,

    /// 实力分数（0-100）
    pub power_score: f64,

    /// 核心数据
    pub win_rate: f64,
    pub avg_kda: f64,
    pub total_games: u32,

    /// 关键优势（前3个）
    pub strengths: Vec<String>,

    /// 关键劣势（前3个）
    pub weaknesses: Vec<String>,

    /// 可靠度（0.0-1.0，基于稳定性）
    pub reliability: f64,

    /// 危险度（对敌人）或信任度（对队友）
    pub trust_score: f64,
}

/// 实力等级
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum TeammatePowerLevel {
    VeryStrong,   // 90+分
    Strong,       // 70-90分
    Average,      // 50-70分
    Weak,         // 30-50分
    VeryWeak,     // <30分
}
```

---

## 🚀 **完整实现路线图**

### **总览**

```
当前状态（✅ 已完成）
    ↓
阶段1：时间线分析（2.5-4.5小时）
    ↓
阶段2：智能建议系统（6-8小时）
    ↓
阶段3：三维战术系统（8-10小时）
    ↓
最终状态：完整的智能助手系统

总计：16.5-22.5 小时
收益：⭐⭐⭐⭐⭐ 产品核心竞争力
```

---

### **详细时间规划**

| 阶段 | 内容 | 工作量 | 优先级 |
|------|------|--------|--------|
| **阶段1.1** | 扩展 Parser（时间线数据） | 1-2h | 🔴 高 |
| **阶段1.2** | 时间线分析器 | 1-2h | 🔴 高 |
| **阶段1.3** | 集成时间线特征 | 0.5-1h | 🔴 高 |
| **阶段2.1** | 定义建议数据结构 | 0.5h | 🔴 高 |
| **阶段2.2** | 实现建议分析器 | 3-4h | 🔴 高 |
| **阶段2.3** | 前端建议组件 | 2-3h | 🔴 高 |
| **阶段3.1** | 团队分析数据结构 | 1h | 🟡 中 |
| **阶段3.2** | 团队战术分析器 | 3-4h | 🟡 中 |
| **阶段3.3** | 前端战术面板 | 3-4h | 🟡 中 |
| **阶段3.4** | 集成到对局分析 | 1h | 🟡 中 |

---

## 📈 **价值分析**

### **对用户的价值**

```
个人提升维度：
✅ 知道自己强弱 → 清晰的自我认知
✅ 具体改进建议 → 明确的提升方向
✅ 时间线分析 → 深入的问题定位

团队协作维度：⭐ NEW
✅ 知道队友强弱 → 合理的期待和配合
✅ 资源分配建议 → 清晰的战术方向
✅ 保护/帮助建议 → 提高团队效率

对局针对维度：
✅ 知道对手弱点 → 明确的针对方向
✅ 战术建议 → 具体的执行方案
✅ 实力对比 → 清晰的局势判断
```

### **产品差异化**

```
市面上所有工具：
- OP.GG：只有个人数据展示
- U.GG：只有胜率统计
- 其他工具：泛泛的攻略

你的工具（三维系统）：
┌────────────────────────────────────┐
│ 个人教练                           │
│ - 诊断问题                         │
│ - 提供改进建议                     │
├────────────────────────────────────┤
│ 团队协调 ⭐                        │
│ - 分析队友强弱                     │
│ - 提供协作建议                     │
│ - 资源分配方案                     │
├────────────────────────────────────┤
│ 战术分析师                         │
│ - 识别对手弱点                     │
│ - 提供针对战术                     │
│ - 给出执行方案                     │
└────────────────────────────────────┘

= 市面上独一无二的完整智能助手！
```

---

## 🎯 **推荐实施策略**

### **方案A：快速原型（MVP）**

```
Week 1：
✅ 阶段1：时间线分析（基础）
✅ 阶段2.1-2.2：建议系统后端

Week 2：
✅ 阶段2.3：建议系统前端
✅ 阶段3.1-3.2：团队战术后端

Week 3：
✅ 阶段3.3-3.4：团队战术前端
✅ 测试和优化

总计：3周
```

### **方案B：分步迭代（推荐）**

```
Sprint 1（本周）：
✅ 阶段1：时间线分析
→ 输出：增强的特征标签

Sprint 2（下周）：
✅ 阶段2：智能建议系统
→ 输出：对自己的改进建议

Sprint 3（下下周）：
✅ 阶段3：三维战术系统
→ 输出：完整的团队战术分析

每个Sprint独立可用，逐步迭代！
```

---

## 🎉 **最终愿景**

```
从一个"数据展示工具"
    ↓
进化为
    ↓
"完整的智能助手系统"

包含：
✅ 个人教练：帮你变强
✅ 团队协调：帮队友配合
✅ 战术分析：帮团队赢

= LOL玩家的"AI教练"！
```

---

## 📝 **总结**

### **你的想法补充得太好了！**

```
原本：对自己 + 对敌人（二维）
补充：+ 对队友（三维）⭐

完整性：从 60% → 100%
```

### **团队协作建议的核心价值**

```
✅ 体现5v5团队游戏本质
✅ 提供资源分配方案
✅ 明确每个人的任务
✅ 提高团队配合效率
✅ 增加胜率
```

### **实现优先级**

```
必做：时间线分析 + 智能建议（基础）
推荐：团队战术系统（完整）
可选：持续优化和扩展
```

---

**这个三维战术系统将是你产品最大的杀手锏！** 🚀

**需要我帮你开始实现吗？我们可以从时间线分析开始，一步步构建完整系统！** 💪
