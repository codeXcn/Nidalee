/// 针对建议策略（对敌人）
///
/// 职责：
/// - 生成针对对手弱点的战术建议
/// - 措辞：第三人称（"对手"/"该玩家"）
/// - 目标：赢得比赛

use super::base::{AdviceStrategy, ProblemData, ProblemType};
use super::super::types::{GameAdvice, AdviceCategory, AdvicePerspective};
use super::super::builder::AdviceBuilder;

pub struct TargetingStrategy;

impl AdviceStrategy for TargetingStrategy {
    fn generate_advice(&self, problem_type: ProblemType, data: &ProblemData) -> Option<GameAdvice> {
        match problem_type {
            ProblemType::LaningCsDeficit => self.create_cs_deficit_advice(data),
            ProblemType::LaningDominated => self.create_dominated_advice(data),
            ProblemType::MidGameDecline => self.create_midgame_decline_advice(data),
            ProblemType::PoorFarming => self.create_poor_farming_advice(data),
            ProblemType::LowKillParticipation => self.create_low_kp_advice(data),
            ProblemType::LowTeamfightParticipation => self.create_low_teamfight_advice(data),
            ProblemType::HighDeathRate => self.create_high_death_advice(data),
            ProblemType::PoorPositioning => self.create_positioning_advice(data),
            ProblemType::LowVisionScore => self.create_vision_advice(data),
            ProblemType::ChampionPoolNarrow => self.create_champion_pool_advice(data),
            ProblemType::ChampionDependency => self.create_dependency_advice(data),
        }
    }

    fn name(&self) -> &str {
        "针对战术策略"
    }

    fn perspective(&self) -> AdvicePerspective {
        AdvicePerspective::Targeting
    }
}

impl TargetingStrategy {
    /// 对手补刀弱 → 如何针对
    fn create_cs_deficit_advice(&self, data: &ProblemData) -> Option<GameAdvice> {
        let target = data.target_name.as_deref().unwrap_or("对手");

        AdviceBuilder::new()
            .title(format!("可针对的弱点：{}对线补刀能力弱", data.role))
            .problem(format!(
                "{}对线期平均落后{:.1}刀，容易被压制",
                target, -data.value
            ))
            .evidence("对手对线期补刀效率低，容易被打出经济差".to_string())
            .suggestion("🎯 选择压制型英雄：对线强势的英雄（刀妹、剑姬、杰斯等）")
            .suggestion("📊 对线打法：频繁消耗，打出血量和补刀优势")
            .suggestion("⏰ 关键时机：前3级建立优势，逼对方回城")
            .suggestion("🔍 配合打野：控线配合越塔，扩大优势")
            .suggestion("💰 装备优势：利用经济差，提前成型碾压")
            .priority(4)
            .category(AdviceCategory::Laning)
            .perspective(self.perspective())
            .target_player(data.target_name.as_deref().unwrap_or("对手"))
            .affected_role(data.role.clone())
            .build()
    }

    /// 对手被压制 → 如何利用
    fn create_dominated_advice(&self, data: &ProblemData) -> Option<GameAdvice> {
        let target = data.target_name.as_deref().unwrap_or("对手");

        AdviceBuilder::new()
            .title(format!("可针对：{}前期容易被击杀", data.role))
            .problem(format!(
                "{}抗压能力弱，前期容易死亡",
                target
            ))
            .evidence("对手对线期经常被击杀或大幅落后".to_string())
            .suggestion(format!("🎯 打野优先级：前期重点照顾{}路，优先gank", data.role))
            .suggestion("📊 英雄选择：选择前期强势的压制型英雄")
            .suggestion("⏰ 时机把握：3级/6级抓一波，滚雪球")
            .suggestion("🔍 视野压制：反掉对方视野，创造gank机会")
            .suggestion(format!("💪 资源倾斜：打野多反对方野区，针对{}", data.role))
            .priority(5)
            .category(AdviceCategory::Laning)
            .perspective(self.perspective())
            .target_player(data.target_name.as_deref().unwrap_or("对手"))
            .affected_role(data.role.clone())
            .build()
    }

    /// 对手中期弱 → 如何利用
    fn create_midgame_decline_advice(&self, data: &ProblemData) -> Option<GameAdvice> {
        let target = data.target_name.as_deref().unwrap_or("对手");

        AdviceBuilder::new()
            .title("可利用：对手中期发育效率低")
            .problem(format!(
                "{}中期经济效率下降，容易被拉开差距",
                target
            ))
            .evidence("对手中期发育节奏差，容易落后".to_string())
            .suggestion("⏰ 中期发力：10-20分钟主动找团战机会")
            .suggestion("🎯 资源控制：占据野区和龙坑资源")
            .suggestion("💰 拉开差距：利用对手发育慢，扩大经济优势")
            .suggestion("🤝 抓单机会：对手落单时果断击杀")
            .priority(3)
            .category(AdviceCategory::Farming)
            .perspective(self.perspective())
            .target_player(data.target_name.as_deref().unwrap_or("对手"))
            .build()
    }

    /// 其他针对建议（简化实现）
    fn create_poor_farming_advice(&self, _data: &ProblemData) -> Option<GameAdvice> {
        None  // 暂不实现
    }

    fn create_low_kp_advice(&self, data: &ProblemData) -> Option<GameAdvice> {
        let target = data.target_name.as_deref().unwrap_or("对手");

        AdviceBuilder::new()
            .title("可利用：对手参团意识差")
            .problem(format!("{}参团率低，容易5打4", target))
            .evidence(format!("参团率仅{:.0}%", data.value * 100.0))
            .suggestion("🎯 主动开团：利用人数优势，强开团战")
            .suggestion("📊 分带牵制：吸引对手，打5v4")
            .suggestion("⏰ 抓单机会：对手游离时击杀")
            .priority(3)
            .category(AdviceCategory::Teamfight)
            .perspective(self.perspective())
            .target_player(data.target_name.as_deref().unwrap_or("对手"))
            .build()
    }

    fn create_low_teamfight_advice(&self, data: &ProblemData) -> Option<GameAdvice> {
        let target = data.target_name.as_deref().unwrap_or("对手");
        
        let tactic = match data.role.as_str() {
            "打野" => "对方打野参团少，可以大胆入侵野区反野，控龙时他很难及时赶到",
            "中单" => "对方中单游走不积极，可以主动推线后游走支援，打人数差",
            "上单" => "对方上单支援慢，可以在下半区开龙开团，逼他交TP",
            "ADC" => "对方ADC参团不积极，抓住5v4机会主动开团",
            "辅助" => "对方辅助游走少，下路2v1时可以压制或配合打野越塔",
            _ => "利用对手参团意识差，制造多打少局面",
        };

        AdviceBuilder::new()
            .title(format!("战术突破口：{}参团意识薄弱", data.role))
            .problem(format!(
                "{}的助攻仅{:.1}次/场，团战中经常缺席或姗姗来迟",
                target, data.value
            ))
            .evidence(data.extra_info.clone().unwrap_or_else(|| "历史数据显示该玩家参团率极低".to_string()))
            .suggestion(format!("💡 核心战术：{}", tactic))
            .suggestion("⚡ 速战速决：发现对手不在时立即开团，打快攻")
            .suggestion("🐉 资源控制：小龙、先锋刷新时主动集合，逼对手选择")
            .suggestion("🎯 分带牵制：让队友单带吸引该玩家，正面4v4直接开团")
            .suggestion("📢 信号沟通：看到该玩家在上路，立即信号下路开团")
            .priority(4)
            .category(AdviceCategory::Teamfight)
            .perspective(self.perspective())
            .affected_role(data.role.clone())
            .target_player(data.target_name.as_deref().unwrap_or("对手"))
            .build()
    }

    fn create_high_death_advice(&self, data: &ProblemData) -> Option<GameAdvice> {
        let target = data.target_name.as_deref().unwrap_or("对手");
        
        let target_strategy = match data.role.as_str() {
            "ADC" => vec![
                "🎯 首要目标：团战开始立即找到该ADC位置，所有人集火秒杀",
                "🪝 开团首选：辅助/坦克优先钩/闪现开该ADC，基本必死",
                "👤 刺客首选：如果有刺客，让他专门切这个送死的ADC",
            ],
            "中单" | "法师" => vec![
                "⚡ 突进猛攻：该玩家站位激进，直接冲脸打他最有效",
                "🎯 诱导技能：故意露头骗他交技能，然后反杀",
                "🔍 视野陷阱：在草丛埋伏，该玩家容易盲目探草",
            ],
            "打野" => vec![
                "🌳 野区埋伏：在他常刷的野区蹲点，配合视野轻松击杀",
                "🐉 争夺目标：打龙时他会冲动进场，是击杀良机",
                "👥 反野入侵：该玩家死得多说明意识差，大胆入侵",
            ],
            "上单" | "坦克" => vec![
                "🎭 假开团：故意让他开团，然后集火反杀",
                "🏃 拉扯消耗：他开团时不要接，拉扯到他残血再反打",
                "⏰ 等技能CD：他交了关键技能后立即反攻",
            ],
            "辅助" => vec![
                "🎯 先杀辅助：该辅助容易死，先秒他让对方失去控制和保护",
                "🪝 钩子致命：该玩家走位差，钩到基本等于击杀",
                "👀 抓单机会：他做视野时容易深入，是最佳击杀时机",
            ],
            _ => vec![
                "🎯 优先目标：团战锁定该玩家，优先集火",
                "👀 寻找破绽：观察他的走位习惯，抓失误击杀",
                "📍 埋伏蹲点：在他常走的路线埋伏",
            ]
        };

        let mut builder = AdviceBuilder::new()
            .title(format!("软柿子：{}生存能力极差", data.role))
            .problem(format!(
                "{}场均死亡{:.1}次，是团队最大弱点",
                target, data.value
            ))
            .evidence(data.extra_info.clone().unwrap_or_else(|| "该玩家频繁暴毙，是最容易击杀的目标".to_string()));

        for suggestion in target_strategy {
            builder = builder.suggestion(suggestion);
        }

        builder
            .suggestion("💬 团队共识：开局就告诉队友重点针对这个送死的，全队配合击杀")
            .priority(5)
            .category(AdviceCategory::Teamfight)
            .perspective(self.perspective())
            .affected_role(data.role.clone())
            .target_player(data.target_name.as_deref().unwrap_or("对手"))
            .build()
    }

    fn create_positioning_advice(&self, _data: &ProblemData) -> Option<GameAdvice> {
        None  // 暂不实现
    }

    fn create_vision_advice(&self, data: &ProblemData) -> Option<GameAdvice> {
        let target = data.target_name.as_deref().unwrap_or("对手");

        AdviceBuilder::new()
            .title("可利用：对手视野控制薄弱")
            .problem(format!("{}视野得分低，关键位置经常没视野", target))
            .evidence(format!("视野得分仅{:.1}/分钟", data.value))
            .suggestion("🎯 野区入侵：大胆入侵对方野区，对方视野薄弱")
            .suggestion("📊 绕后gank：利用视野盲区，频繁绕后")
            .suggestion("⏰ 偷龙战术：对方做视野不及时，可以偷龙")
            .suggestion("🔍 扫描压制：多带扫描，清理对方仅有的眼位")
            .priority(3)
            .category(AdviceCategory::Vision)
            .perspective(self.perspective())
            .target_player(data.target_name.as_deref().unwrap_or("对手"))
            .build()
    }

    fn create_champion_pool_advice(&self, _data: &ProblemData) -> Option<GameAdvice> {
        None  // 英雄池窄不是针对点
    }

    fn create_dependency_advice(&self, data: &ProblemData) -> Option<GameAdvice> {
        let target = data.target_name.as_deref().unwrap_or("对手");

        AdviceBuilder::new()
            .title("可针对：对手依赖单一英雄")
            .problem(format!("{}过度依赖某个英雄，但胜率不高", target))
            .evidence("单一英雄占比高，可以针对".to_string())
            .suggestion("🚫 Ban掉绝活：优先ban对方最常用的英雄")
            .suggestion("🎯 克制选择：选择克制该英雄的英雄")
            .suggestion("📚 研究打法：了解该英雄的弱点")
            .priority(3)
            .category(AdviceCategory::Champion)
            .perspective(self.perspective())
            .target_player(data.target_name.as_deref().unwrap_or("对手"))
            .build()
    }
}

