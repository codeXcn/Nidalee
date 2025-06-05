<template>
  <div class="game-helper">
    <div class="config-panel">
      <h2>功能设置</h2>

      <!-- 自动接受对局 -->
      <div class="config-item">
        <label>
          <input v-model="config.autoAccept" type="checkbox" />
          自动接受对局
        </label>
      </div>

      <!-- 自动选择英雄 -->
      <div class="config-item">
        <label>
          <input v-model="config.autoPickChampion.enabled" type="checkbox" />
          自动选择英雄
        </label>
        <select
          v-model="config.autoPickChampion.championId"
          :disabled="!config.autoPickChampion.enabled"
        >
          <option v-for="champion in champions" :key="champion.id" :value="champion.id">
            {{ champion.name }}
          </option>
        </select>
      </div>

      <!-- 自动禁用英雄 -->
      <div class="config-item">
        <label>
          <input v-model="config.autoBanChampion.enabled" type="checkbox" />
          自动禁用英雄
        </label>
        <select
          v-model="config.autoBanChampion.championId"
          :disabled="!config.autoBanChampion.enabled"
        >
          <option v-for="champion in champions" :key="champion.id" :value="champion.id">
            {{ champion.name }}
          </option>
        </select>
      </div>

      <!-- 自动符文配置 -->
      <div class="config-item">
        <label>
          <input v-model="config.autoRune.enabled" type="checkbox" />
          自动符文配置
        </label>
        <button
          :disabled="!config.autoRune.enabled || !config.autoPickChampion.championId"
          @click="updateRunePage"
        >
          更新符文页
        </button>
      </div>
    </div>

    <!-- 对局分析 -->
    <div v-if="matchAnalysis" class="match-analysis">
      <h2>对局分析</h2>

      <div class="analysis-overview">
        <div class="analysis-item">
          <h3>团队评分</h3>
          <div class="score">{{ matchAnalysis.teamScore }}/100</div>
        </div>

        <div class="analysis-item">
          <h3>阵容优势</h3>
          <div
            class="score"
            :class="{
              positive: matchAnalysis.compositionScore > 0,
              negative: matchAnalysis.compositionScore < 0
            }"
          >
            {{ matchAnalysis.compositionScore > 0 ? '+' : '' }}{{ matchAnalysis.compositionScore }}
          </div>
        </div>

        <div class="analysis-item">
          <h3>团战能力</h3>
          <div class="score">{{ matchAnalysis.teamfightScore }}/100</div>
        </div>
      </div>

      <div class="lane-advantages">
        <h3>各路对线形势</h3>
        <div
          v-for="(advantage, position) in matchAnalysis.laneAdvantages"
          :key="position"
          class="lane-item"
        >
          <span class="position">{{ getPositionName(position) }}</span>
          <div class="advantage-bar">
            <div
              class="advantage-indicator"
              :style="{ left: `${50 + advantage / 2}%` }"
              :class="{ positive: advantage > 0, negative: advantage < 0 }"
            ></div>
          </div>
        </div>
      </div>

      <div class="tactics">
        <h3>战术建议</h3>
        <ul>
          <li v-for="(tactic, index) in matchAnalysis.suggestedTactics" :key="index">
            {{ tactic }}
          </li>
        </ul>
      </div>
    </div>

    <!-- 队伍信息 -->
    <div class="teams-container">
      <div class="team-info">
        <h3>我方队伍</h3>
        <div
          v-for="player in gameInfo?.myTeam"
          :key="player.summoner.summonerId"
          class="player-card"
        >
          <div class="player-basic">
            <img
              :src="getChampionIcon(player.championId)"
              :alt="getChampionName(player.championId)"
              class="champion-icon"
            />
            <div class="player-info">
              <div class="player-name">{{ player.summoner.displayName }}</div>
              <div class="player-position">{{ getPositionName(player.assignedPosition) }}</div>
            </div>
          </div>

          <div v-if="player.stats" class="player-stats">
            <div class="stat-item">
              <span class="label">胜率</span>
              <span class="value"
                >{{ ((player.stats.wins / player.stats.totalGames) * 100).toFixed(1) }}%</span
              >
            </div>
            <div class="stat-item">
              <span class="label">KDA</span>
              <span class="value">{{ player.stats.kda.toFixed(2) }}</span>
            </div>
            <div class="stat-item">
              <span class="label">评分</span>
              <span class="value">{{ player.stats.performanceScore }}/100</span>
            </div>
          </div>
        </div>
      </div>

      <div class="team-info">
        <h3>敌方队伍</h3>
        <div
          v-for="player in gameInfo?.theirTeam"
          :key="player.summoner.summonerId"
          class="player-card"
        >
          <div class="player-basic">
            <img
              :src="getChampionIcon(player.championId)"
              :alt="getChampionName(player.championId)"
              class="champion-icon"
            />
            <div class="player-info">
              <div class="player-name">{{ player.summoner.displayName }}</div>
              <div class="player-position">{{ getPositionName(player.assignedPosition) }}</div>
            </div>
          </div>

          <div v-if="player.stats" class="player-stats">
            <div class="stat-item">
              <span class="label">胜率</span>
              <span class="value"
                >{{ ((player.stats.wins / player.stats.totalGames) * 100).toFixed(1) }}%</span
              >
            </div>
            <div class="stat-item">
              <span class="label">KDA</span>
              <span class="value">{{ player.stats.kda.toFixed(2) }}</span>
            </div>
            <div class="stat-item">
              <span class="label">评分</span>
              <span class="value">{{ player.stats.performanceScore }}/100</span>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'
import { LCPClient, Champion } from '@/lib/lcpClient'

interface GameInfo {
  myTeam: PlayerInfo[]
  theirTeam: PlayerInfo[]
}

interface PlayerInfo {
  summoner: {
    summonerId: number
    displayName: string
  }
  ranked: any
  history: any
  assignedPosition: string
  championId: number
}

interface GameConfig {
  autoAccept: boolean
  autoPickChampion: {
    enabled: boolean
    championId: number
  }
  autoBanChampion: {
    enabled: boolean
    championId: number
  }
  autoRune: {
    enabled: boolean
    pageId: number
    page: RunePage | null
  }
}

interface RunePage {
  id: number
  name: string
  primaryStyleId: number
  subStyleId: number
  selectedPerkIds: number[]
}

interface PlayerStats {
  totalGames: number
  wins: number
  kda: number
  mainPositions: string[]
  mainChampions: number[]
  performanceScore: number
}

interface MatchAnalysis {
  teamScore: number
  compositionScore: number
  laneAdvantages: Record<string, number>
  teamfightScore: number
  suggestedTactics: string[]
}

// 配置
const config = ref<GameConfig>({
  autoAccept: true,
  autoPickChampion: {
    enabled: false,
    championId: 0
  },
  autoBanChampion: {
    enabled: false,
    championId: 0
  },
  autoRune: {
    enabled: false,
    pageId: 0,
    page: null
  }
})

// 监听配置变化
watch(
  () => config.value.autoAccept,
  async enabled => {
    await invoke('set_auto_accept', { enabled })
  }
)

watch(
  () => config.value.autoPickChampion,
  async pick => {
    await invoke('set_auto_pick', { championId: pick.enabled ? pick.championId : null })
  },
  { deep: true }
)

watch(
  () => config.value.autoBanChampion,
  async ban => {
    await invoke('set_auto_ban', { championId: ban.enabled ? ban.championId : null })
  },
  { deep: true }
)

watch(
  () => config.value.autoRune,
  async rune => {
    if (rune.enabled && rune.page) {
      try {
        // TODO: 应用符文页到客户端
      } catch (error) {
        console.error('应用符文页失败:', error)
      }
    }
  },
  { deep: true }
)

// 英雄列表
const champions = ref<Champion[]>([])

// 游戏服务
const client = new LCPClient({
  token: '', // 需要从客户端获取
  port: '' // 需要从客户端获取
})

// 游戏信息
const gameInfo = ref<GameInfo | null>(null)
const matchAnalysis = ref<MatchAnalysis | null>(null)
const updateInterval = ref<NodeJS.Timeout | null>(null)

// 方法
async function updateRunePage() {
  if (!config.value.autoPickChampion.championId) return

  try {
    const runePage = await client.getRuneSuggestion(config.value.autoPickChampion.championId)
    config.value.autoRune.page = runePage
    // TODO: 应用符文页到客户端
  } catch (error) {
    console.error('更新符文页失败:', error)
  }
}

// 更新游戏信息的同时获取分析数据
async function updateGameInfo() {
  try {
    const info = await client.getGameInfo()
    gameInfo.value = info as GameInfo

    if (gameInfo.value) {
      // 获取对局分析
      const analysis = await client.analyzeMatch(gameInfo.value)
      matchAnalysis.value = analysis
    }
  } catch (error) {
    console.error('更新游戏信息出错:', error)
  }
}

// 启动服务
onMounted(async () => {
  // 获取英雄列表
  try {
    champions.value = await client.getChampions()
  } catch (error) {
    console.error('获取英雄列表失败:', error)
  }

  // 初始化游戏助手
  try {
    await invoke('init_game_helper', {
      token: client.config.token,
      port: client.config.port
    })
  } catch (error) {
    console.error('初始化游戏助手失败:', error)
  }

  // 启动自动更新
  updateInterval.value = setInterval(updateGameInfo, 2000)
})

// 清理
onUnmounted(() => {
  if (updateInterval.value) {
    clearInterval(updateInterval.value)
  }
})

// 工具函数
function getChampionName(id: number): string {
  const champion = champions.value.find(c => c.id === id)
  return champion?.name || `英雄 ${id}`
}

function getChampionIcon(id: number): string {
  const champion = champions.value.find(c => c.id === id)
  return champion?.squarePortraitPath || `https://example.com/champion-icons/${id}.png`
}

function getPositionName(position: string): string {
  const positions: Record<string, string> = {
    TOP: '上路',
    JUNGLE: '打野',
    MIDDLE: '中路',
    BOTTOM: '下路',
    UTILITY: '辅助'
  }
  return positions[position] || position
}

function getRankedTier(ranked: any): string {
  // TODO: 实现段位显示
  return `${ranked?.tier || '未定级'} ${ranked?.rank || ''}`
}

function getWinRate(history: any): number {
  if (!history?.games?.length) return 0
  const wins = history.games.filter((game: any) => game.participants[0].stats.win).length
  return Math.round((wins / history.games.length) * 100)
}
</script>

<style scoped>
.game-helper {
  padding: 20px;
  max-width: 1200px;
  margin: 0 auto;
  font-family: Arial, sans-serif;
}

.config-panel,
.match-analysis,
.team-info {
  background: #fff;
  padding: 20px;
  border-radius: 8px;
  margin-bottom: 20px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.analysis-overview {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 20px;
  margin-bottom: 30px;
}

.analysis-item {
  text-align: center;
  padding: 15px;
  background: #f5f5f5;
  border-radius: 6px;
}

.score {
  font-size: 24px;
  font-weight: bold;
  margin-top: 10px;
}

.score.positive {
  color: #4caf50;
}
.score.negative {
  color: #f44336;
}

.lane-advantages {
  margin-bottom: 30px;
}

.lane-item {
  display: flex;
  align-items: center;
  margin: 10px 0;
}

.position {
  width: 100px;
  font-weight: 500;
}

.advantage-bar {
  flex: 1;
  height: 20px;
  background: #f5f5f5;
  border-radius: 10px;
  position: relative;
}

.advantage-indicator {
  position: absolute;
  width: 10px;
  height: 20px;
  border-radius: 5px;
  background: #2196f3;
  transform: translateX(-50%);
}

.advantage-indicator.positive {
  background: #4caf50;
}
.advantage-indicator.negative {
  background: #f44336;
}

.tactics ul {
  list-style: none;
  padding: 0;
}

.tactics li {
  padding: 10px;
  margin: 5px 0;
  background: #f5f5f5;
  border-radius: 4px;
}

.teams-container {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 20px;
}

.player-card {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 15px;
  border-bottom: 1px solid #eee;
}

.player-basic {
  display: flex;
  align-items: center;
  gap: 15px;
}

.champion-icon {
  width: 50px;
  height: 50px;
  border-radius: 25%;
  object-fit: cover;
}

.player-info {
  display: flex;
  flex-direction: column;
  gap: 5px;
}

.player-name {
  font-weight: 500;
  font-size: 16px;
}

.player-position {
  color: #666;
  font-size: 14px;
}

.player-stats {
  display: flex;
  gap: 20px;
}

.stat-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 5px;
}

.stat-item .label {
  font-size: 12px;
  color: #666;
}

.stat-item .value {
  font-weight: 500;
  font-size: 14px;
}
</style>
