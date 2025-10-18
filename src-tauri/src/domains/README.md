# 领域层 (Domain Layer)

> DDD架构 - 核心业务逻辑层

## 📁 目录结构

### `analysis/` - 对局分析领域
核心职责：解析对局数据、计算统计指标、分析玩家特征

```
analysis/
├─ analyzers/
│  ├─ core/          # 核心分析
│  │  ├─ parser.rs   - LCU数据解析
│  │  ├─ stats.rs    - 统计指标计算
│  │  └─ strategy.rs - 分析策略选择
│  └─ traits/        # 特征分析
│     ├─ basic.rs        - 基础特征（胜率、KDA）
│     ├─ advanced.rs     - 深度特征（参团率、伤害）
│     ├─ role.rs         - 位置特征
│     ├─ distribution.rs - 分布特征（高光、崩盘）
│     ├─ timeline.rs     - 时间线特征（对线、发育）
│     └─ merger.rs       - 智能去重
├─ thresholds.rs     - 阈值配置
└─ docs/             - 设计文档（17个.md）
```

### `tactical_advice/` - 战术建议领域
核心职责：基于分析结果生成智能建议

```
tactical_advice/
├─ analyzers/        # 问题分析器（责任链）
│  ├─ laning.rs      - 对线期问题
│  ├─ farming.rs     - 发育问题
│  ├─ teamfight.rs   - 团战问题
│  ├─ vision.rs      - 视野问题
│  └─ champion.rs    - 英雄池问题
├─ strategies/       # 建议策略（策略模式）
│  ├─ self_improvement.rs  - 自我提升视角
│  ├─ targeting.rs         - 针对敌人视角
│  └─ collaboration.rs     - 团队协作视角
├─ builder.rs        - 建造者模式
├─ chain.rs          - 责任链管理
├─ factory.rs        - 工厂模式
└─ context.rs        - 分析上下文
```

## 🎯 设计原则

### DDD分层
- **Domain** (领域层): 核心业务逻辑，不依赖外部
- **Application** (应用层): 用例编排 → 在 `lcu/matches/service.rs`
- **Infrastructure** (基础设施): LCU API调用 → 在 `lcu/*/`

### 依赖方向
```
Infrastructure → Application → Domain
```

## 🚀 使用示例

```rust
// 1. 对局分析
use crate::domains::analysis::*;

let games = parse_games(raw_data, puuid);
let stats = analyze_player_stats(&games, puuid, context);
let traits = analyze_traits(&stats);

// 2. 战术建议
use crate::domains::tactical_advice::*;

let advice = generate_advice(
    &stats, 
    &games, 
    "上单",
    AdvicePerspective::Targeting,
    Some("敌方玩家"),
    &strategy
);
```

## 📊 重构成果

- ✅ 代码和文档分离
- ✅ 领域逻辑清晰
- ✅ 分组结构合理
- ✅ 便于维护扩展
