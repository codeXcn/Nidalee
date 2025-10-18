/// 责任链模式（Chain of Responsibility）
///
/// 职责：
/// - 管理多个建议分析器（责任链节点）
/// - 依次执行每个分析器
/// - 收集所有生成的建议
/// - 按优先级排序并限制数量

use super::context::AdviceContext;
use super::types::GameAdvice;
use super::analyzers::base::AdviceAnalyzer;  // 使用 analyzers::base 的 trait
use crate::domains::analysis::AnalysisStrategy;

/// 建议生成责任链
pub struct AdviceChain {
    analyzers: Vec<Box<dyn AdviceAnalyzer>>,
}

impl AdviceChain {
    /// 创建新的责任链
    pub fn new() -> Self {
        Self {
            analyzers: Vec::new(),
        }
    }

    /// 添加分析器到责任链
    pub fn add_analyzer(mut self, analyzer: Box<dyn AdviceAnalyzer>) -> Self {
        self.analyzers.push(analyzer);
        self
    }

    /// 执行责任链，生成所有建议
    ///
    /// 流程：
    /// 1. 依次执行每个分析器（如果启用）
    /// 2. 收集所有建议
    /// 3. 按优先级排序
    /// 4. 限制数量（前5条）
    pub fn generate(
        &self,
        context: &AdviceContext,
        strategy: &AnalysisStrategy,
    ) -> Vec<GameAdvice> {
        let mut all_advice = Vec::new();

        println!("🔗 开始执行建议生成责任链...");
        println!("   视角：{:?}", context.perspective);
        println!("   位置：{}", context.role);
        println!("   对局数：{}", context.game_count());

        // 依次执行每个分析器
        for analyzer in &self.analyzers {
            // 检查是否启用
            if !analyzer.is_enabled(strategy) {
                println!("   ⏭️  {} - 已跳过（策略不启用）", analyzer.name());
                continue;
            }

            // 执行分析
            let advice = analyzer.analyze(context);

            if advice.is_empty() {
                println!("   ✓  {} - 无建议（表现良好）", analyzer.name());
            } else {
                println!("   ✓  {} - 生成 {} 条建议", analyzer.name(), advice.len());
                all_advice.extend(advice);
            }
        }

        // 按优先级排序（从高到低）
        all_advice.sort_by_key(|a| std::cmp::Reverse(a.priority));

        // 限制数量（前5条最重要的）
        all_advice.truncate(5);

        println!("✅ 责任链执行完成：共生成 {} 条建议", all_advice.len());

        all_advice
    }
}

impl Default for AdviceChain {
    fn default() -> Self {
        Self::new()
    }
}

