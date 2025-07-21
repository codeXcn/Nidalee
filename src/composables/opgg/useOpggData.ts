import { ref } from 'vue'
import { fetchOpggChampionBuild, fetchOpggTierList } from '@/lib/dataApi'

export interface OpggConfig {
  region: string
  mode: string
  tier: string
  position: string
  championId: number
}

export interface OpggState {
  loading: boolean
  error: string | null
  message: string
  championBuild: any | null
  tierList: any | null
}

export function useOpggData() {
  // 状态管理
  const state = ref<OpggState>({
    loading: false,
    error: null,
    message: '',
    championBuild: null,
    tierList: null
  })

  // 配置选项
  const config = ref<OpggConfig>({
    region: 'global',
    mode: 'ranked',
    tier: 'all',
    position: 'MID',
    championId: 157
  })

  // 选项数据
  const regions = [
    { value: 'global', label: '全球' },
    { value: 'kr', label: '韩服' },
    { value: 'na', label: '北美' },
    { value: 'euw', label: '欧洲西部' },
    { value: 'eune', label: '欧洲北欧东部' },
    { value: 'jp', label: '日服' },
    { value: 'br', label: '巴西' },
    { value: 'lan', label: '拉丁美洲北部' },
    { value: 'las', label: '拉丁美洲南部' },
    { value: 'oce', label: '大洋洲' },
    { value: 'tr', label: '土耳其' },
    { value: 'ru', label: '俄罗斯' }
  ]

  const modes = [
    { value: 'ranked', label: '排位赛' },
    { value: 'aram', label: '大乱斗' },
    { value: 'arena', label: '斗魂竞技场' },
    { value: 'urf', label: '无限火力' }
  ]

  const tiers = [
    { value: 'all', label: '全部段位' },
    { value: 'iron', label: '黑铁' },
    { value: 'bronze', label: '青铜' },
    { value: 'silver', label: '白银' },
    { value: 'gold', label: '黄金' },
    { value: 'platinum', label: '铂金' },
    { value: 'emerald', label: '翡翠' },
    { value: 'diamond', label: '钻石' },
    { value: 'master', label: '大师' },
    { value: 'grandmaster', label: '宗师' },
    { value: 'challenger', label: '王者' }
  ]

  const positions = [
    { value: 'TOP', label: '上单' },
    { value: 'JUNGLE', label: '打野' },
    { value: 'MID', label: '中单' },
    { value: 'ADC', label: '下路' },
    { value: 'SUPPORT', label: '辅助' }
  ]

  // 获取英雄详细数据
  const loadChampionBuild = async () => {
    if (!config.value.championId) {
      state.value.error = '请输入英雄ID'
      return
    }

    state.value.loading = true
    state.value.error = null
    state.value.championBuild = null
    state.value.message = ''

    try {
      state.value.message = '正在获取英雄详细数据...'
      console.log('🔍 开始获取OP.GG数据:', {
        region: config.value.region,
        mode: config.value.mode,
        champion_id: config.value.championId,
        position: config.value.position,
        tier: config.value.tier
      })

      const result = await fetchOpggChampionBuild({
        region: config.value.region,
        mode: config.value.mode,
        champion_id: config.value.championId,
        position: config.value.position,
        tier: config.value.tier
      })

      console.log('📊 OP.GG数据获取结果:', result)

      if (result.success && result.data) {
        // 强制触发响应式更新
        state.value.championBuild = null
        state.value.championBuild = result.data
        state.value.message = '英雄详细数据加载成功！'
        console.log('✅ 数据设置成功:', result.data)
        console.log('🔍 状态检查 - championBuild:', state.value.championBuild)
      } else {
        state.value.error = result.error || '获取英雄详细数据失败'
        console.error('❌ 数据获取失败:', result.error)
      }
    } catch (err) {
      state.value.error = err instanceof Error ? err.message : '未知错误'
      console.error('💥 数据获取异常:', err)
    } finally {
      state.value.loading = false
    }
  }

  // 获取层级列表数据
  const loadTierList = async () => {
    state.value.loading = true
    state.value.error = null
    state.value.tierList = null
    state.value.message = ''

    try {
      state.value.message = '正在获取层级列表...'

      const result = await fetchOpggTierList({
        region: config.value.region,
        mode: config.value.mode,
        tier: config.value.tier
      })

      if (result.success && result.data) {
        state.value.tierList = result.data
        state.value.message = '层级列表加载成功！'
      } else {
        state.value.error = result.error || '获取层级列表失败'
      }
    } catch (err) {
      state.value.error = err instanceof Error ? err.message : '未知错误'
    } finally {
      state.value.loading = false
    }
  }

  // 清空数据
  const clearData = () => {
    state.value.championBuild = null
    state.value.tierList = null
    state.value.error = null
    state.value.message = ''
  }

  // 测试组件功能
  const testComponent = () => {
    state.value.message = '组件功能正常，可以与后端通信！'
    setTimeout(() => {
      state.value.message = ''
    }, 3000)
  }

  return {
    // 状态
    state,

    // 配置
    config,
    regions,
    modes,
    tiers,
    positions,

    // 方法
    loadChampionBuild,
    loadTierList,
    clearData,
    testComponent
  }
}
