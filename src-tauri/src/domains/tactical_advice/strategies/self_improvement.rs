/// 改进建议策略（对自己）
///
/// 职责：
/// - 生成帮助玩家提升的建议
/// - 措辞：第二人称（"你"）
/// - 目标：长期提升

use super::base::{AdviceStrategy, ProblemData, ProblemType};
use super::super::types::{GameAdvice, AdviceCategory, AdvicePerspective};
use super::super::builder::AdviceBuilder;

pub struct SelfImprovementStrategy;

impl AdviceStrategy for SelfImprovementStrategy {
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
        "改进建议策略"
    }

    fn perspective(&self) -> AdvicePerspective {
        AdvicePerspective::SelfImprovement
    }
}

impl SelfImprovementStrategy {
    /// 补刀落后建议
    fn create_cs_deficit_advice(&self, data: &ProblemData) -> Option<GameAdvice> {
        AdviceBuilder::new()
            .title("对线补刀能力待提升")
            .problem(format!(
                "你的对线期平均落后{:.1}刀，经常被压制",
                -data.value
            ))
            .evidence(data.extra_info.clone().unwrap_or_else(|| "对线期补刀效率偏低".to_string()))
            .suggestion("🎯 练习补刀基本功：训练模式每天练习10分钟")
            .suggestion("📍 改善对线站位：避免被频繁消耗而漏刀")
            .suggestion("⚡ 优化技能释放：用技能补远程兵和炮车")
            .suggestion("🤝 加强辅助配合：沟通压制时机和换血节奏")
            .suggestion("🛡️ 选择稳健英雄：考虑手长安全型英雄")
            .priority(4)
            .category(AdviceCategory::Laning)
            .perspective(self.perspective())
            .build()
    }

    /// 对线被压制建议
    fn create_dominated_advice(&self, data: &ProblemData) -> Option<GameAdvice> {
        AdviceBuilder::new()
            .title("对线期被压制严重")
            .problem(format!(
                "你在{}位置的对线期表现不佳，经常处于劣势",
                data.role
            ))
            .evidence(data.extra_info.clone().unwrap_or_else(|| "多次被击杀或大幅落后".to_string()))
            .suggestion("🛡️ 学习防守对线：优先保证存活，补刀为次")
            .suggestion("👁️ 加强视野控制：避免被gank")
            .suggestion("📚 研究对线知识：了解英雄克制关系")
            .suggestion("🏃 掌握逃生技巧：保留位移技能用于逃跑")
            .suggestion("💪 选择抗压英雄：塞恩、奥恩等稳健型")
            .priority(5)
            .category(AdviceCategory::Laning)
            .perspective(self.perspective())
            .affected_role(data.role.clone())
            .build()
    }

    /// 中期经济下降建议
    fn create_midgame_decline_advice(&self, data: &ProblemData) -> Option<GameAdvice> {
        AdviceBuilder::new()
            .title("中期发育节奏需优化")
            .problem(format!(
                "你的中期经济效率下降{:.0}%，发育节奏有问题",
                data.severity * 100.0
            ))
            .evidence(data.extra_info.clone().unwrap_or_else(|| "频繁游走但收益不高".to_string()))
            .suggestion("⏰ 优化游走时机：只在炮车线或兵线推进时游走")
            .suggestion("💰 提升参团收益：参团后优先吃野怪和兵线")
            .suggestion("🎯 平衡游走和发育：避免无意义的游走")
            .suggestion("👁️ 提升地图意识：提前判断团战位置，减少赶路时间")
            .suggestion("📊 学习资源分配：野区资源及时清理")
            .priority(3)
            .category(AdviceCategory::Farming)
            .perspective(self.perspective())
            .build()
    }

    /// 补刀效率低建议
    fn create_poor_farming_advice(&self, data: &ProblemData) -> Option<GameAdvice> {
        AdviceBuilder::new()
            .title("补刀效率需要提升")
            .problem(format!(
                "你的平均补刀{:.1}/分钟，低于标准",
                data.value
            ))
            .evidence(data.extra_info.clone().unwrap_or_else(|| "发育效率偏低".to_string()))
            .suggestion("🎯 基础训练：训练模式练习补刀")
            .suggestion("⏰ 刷野时机：对线结束后及时清理野区")
            .suggestion("📊 兵线管理：合理控线和推线")
            .suggestion("🎮 熟练英雄：提升清兵效率")
            .priority(3)
            .category(AdviceCategory::Farming)
            .perspective(self.perspective())
            .build()
    }

    /// 参团率低建议
    fn create_low_kp_advice(&self, data: &ProblemData) -> Option<GameAdvice> {
        AdviceBuilder::new()
            .title("团战参与度需要提升")
            .problem(format!(
                "你的参团率仅{:.0}%，参与度偏低",
                data.value * 100.0
            ))
            .evidence("经常错过团战或游离在外".to_string())
            .suggestion("👁️ 提升地图意识：时刻观察队友位置")
            .suggestion("🔔 关注信号：队友发信号时立即支援")
            .suggestion("⏰ TP使用：有TP的英雄要及时支援")
            .suggestion("📍 提前就位：预判团战位置，提前靠近")
            .priority(4)
            .category(AdviceCategory::Teamfight)
            .perspective(self.perspective())
            .build()
    }

    /// 低参团率建议
    fn create_low_teamfight_advice(&self, data: &ProblemData) -> Option<GameAdvice> {
        let role_specific = match data.role.as_str() {
            "打野" => "作为打野，你需要时刻关注河道视野和龙坑控制，提前预判团战位置",
            "中单" => "中路是支援最快的位置，多用技能推线后游走支援边路",
            "上单" => "注意TP的使用时机，不要等团战打完才传送",
            "辅助" => "作为辅助应该带头发起团战，多用信号引导队友",
            _ => "主动关注队友动向，及时支援",
        };

        AdviceBuilder::new()
            .title("团战参与度需要提升")
            .problem(format!(
                "你的平均助攻仅{:.1}次，在团战中的存在感较低",
                data.value
            ))
            .evidence(data.extra_info.clone().unwrap_or_else(|| "经常错过关键团战".to_string()))
            .suggestion("👁️ 强化地图意识：每10秒看一次小地图，观察队友和敌人位置")
            .suggestion("⏰ 重要目标倒计时：龙/先锋刷新前60秒就要开始移动")
            .suggestion(format!("💡 位置意识：{}", role_specific))
            .suggestion("📢 主动沟通：发现队友集结立即靠近，用信号确认你的位置")
            .suggestion("🎯 优先级判断：团战 > 刷野/补兵，除非你能1带1换取更大优势")
            .priority(4)
            .category(AdviceCategory::Teamfight)
            .perspective(self.perspective())
            .affected_role(data.role.clone())
            .build()
    }

    /// 死亡过多建议
    fn create_high_death_advice(&self, data: &ProblemData) -> Option<GameAdvice> {
        let role_advice = match data.role.as_str() {
            "ADC" => vec![
                "🛡️ 站位铁律：保持与前排至少600码距离，永远站在最后排",
                "⚡ 输出时机：等对方关键控制技能交出后再进场输出",
                "💰 生存装备：优先考虑守护天使、水银饰带、血手",
            ],
            "中单" | "法师" => vec![
                "📍 安全距离：利用技能射程优势，不要贴脸输出",
                "⚡ 技能释放：一套技能打完立即撤退，等CD再进场",
                "🔮 保命装备：金身、女妖、炽天使都是好选择",
            ],
            "上单" | "坦克" => vec![
                "🎯 选择时机：不要无脑开团，等队友就位或抓到落单敌人",
                "💪 装备优先：日炎、石像鬼板甲等坦度装备优先于伤害",
                "🏃 撤退意识：残血及时撤退，别想着1换1",
            ],
            "打野" => vec![
                "👁️ 视野优先：刷野前先做视野，避免被反野击杀",
                "⏰ 避免贪心：别在劣势时深入敌方野区",
                "🎯 Gank选择：只Gank有把握的路，失败了别强行越塔",
            ],
            "辅助" => vec![
                "🛡️ 保护C位：团战时站在ADC身边，别只顾着开团",
                "👀 视野安全：插眼时先确认周围安全，别盲目深入",
                "⚡ 技能控制：关键控制技能留给刺客，保护后排",
            ],
            _ => vec![
                "🛡️ 保守原则：优先保证存活，KDA比人头数更重要",
                "👀 地图意识：时刻观察敌人位置，避免被Gank",
                "📍 站位调整：团战时选择安全位置，不要冲太前",
            ]
        };

        let mut builder = AdviceBuilder::new()
            .title(format!("生存问题：场均死亡{:.1}次", data.value))
            .problem(format!(
                "你的{}位置生存能力较弱，频繁阵亡影响团队节奏",
                data.role
            ))
            .evidence(data.extra_info.clone().unwrap_or_else(|| "死亡次数明显高于平均水平".to_string()));

        for suggestion in role_advice {
            builder = builder.suggestion(suggestion);
        }

        builder
            .suggestion("📊 复盘分析：每次死亡后思考「为什么死」「如何避免」")
            .priority(5)
            .category(AdviceCategory::Teamfight)
            .perspective(self.perspective())
            .affected_role(data.role.clone())
            .build()
    }

    /// 站位问题建议
    fn create_positioning_advice(&self, data: &ProblemData) -> Option<GameAdvice> {
        AdviceBuilder::new()
            .title("团战站位需要改善")
            .problem("你在团战中经常被先手击杀或承受过多伤害".to_string())
            .evidence(data.extra_info.clone().unwrap_or_else(|| "团战死亡率高".to_string()))
            .suggestion("📍 保持距离：与前排保持安全距离")
            .suggestion("👀 观察技能：等对方关键控制技能交出后再进场")
            .suggestion("🛡️ 出装调整：考虑水银鞋、复活甲等保命装")
            .suggestion("⏰ 把握时机：不要一开始就冲，等残血再收割")
            .priority(4)
            .category(AdviceCategory::Positioning)
            .perspective(self.perspective())
            .affected_role(data.role.clone())
            .build()
    }

    /// 视野得分低建议
    fn create_vision_advice(&self, data: &ProblemData) -> Option<GameAdvice> {
        let role_specific = match data.role.as_str() {
            "辅助" => "💡 辅助专属：优先升级辅助装备，解锁更多眼位",
            _ => "💡 购买真眼：身上始终保持1个真眼",
        };

        AdviceBuilder::new()
            .title("视野控制需要加强")
            .problem(format!(
                "你的视野得分仅{:.1}/分钟，低于{}位置标准",
                data.value, data.role
            ))
            .evidence(data.extra_info.clone().unwrap_or_else(|| "眼位放置不足".to_string()))
            .suggestion("🔍 养成买眼习惯：每次回城都购买控制守卫")
            .suggestion("📍 学习关键眼位：龙坑、野区入口、河道草丛")
            .suggestion("⏰ 提前做视野：打龙前1分钟就要布置视野")
            .suggestion("👁️ 注意排眼：使用扫描清理关键位置")
            .suggestion(role_specific)
            .priority(3)
            .category(AdviceCategory::Vision)
            .perspective(self.perspective())
            .affected_role(data.role.clone())
            .build()
    }

    /// 英雄池窄建议
    fn create_champion_pool_advice(&self, data: &ProblemData) -> Option<GameAdvice> {
        AdviceBuilder::new()
            .title("英雄池需要扩展")
            .problem("你的英雄池较窄，容易被针对".to_string())
            .evidence(data.extra_info.clone().unwrap_or_else(|| "主要使用1-2个英雄".to_string()))
            .suggestion("🎮 学习新英雄：至少掌握3个不同类型的英雄")
            .suggestion("📚 理解英雄类型：坦克、刺客、法师等")
            .suggestion("🎯 根据阵容选择：学会根据团队需要选英雄")
            .suggestion("🔄 练习模式：新英雄先在训练模式熟悉")
            .priority(2)
            .category(AdviceCategory::Champion)
            .perspective(self.perspective())
            .build()
    }

    /// 过度依赖单一英雄建议
    fn create_dependency_advice(&self, data: &ProblemData) -> Option<GameAdvice> {
        AdviceBuilder::new()
            .title("降低对单一英雄的依赖")
            .problem("你过度依赖某个英雄，但该英雄胜率不高".to_string())
            .evidence(data.extra_info.clone().unwrap_or_else(|| "单一英雄占比过高但效果不佳".to_string()))
            .suggestion("🔄 尝试其他英雄：不要局限于单一英雄")
            .suggestion("📊 分析胜率数据：找到真正适合你的英雄")
            .suggestion("🎯 扩展英雄池：学习类似定位的其他英雄")
            .suggestion("🎮 适应版本：根据版本强度调整英雄选择")
            .priority(2)
            .category(AdviceCategory::Champion)
            .perspective(self.perspective())
            .build()
    }
}

