<template>
  <div class="space-y-6">
    <!-- 用户信息卡片 -->
    <Card v-if="summonerInfo" class="overflow-hidden py-0">
      <!-- 头部渐变背景 -->
      <div class="bg-gradient-to-br from-blue-500 via-purple-600 to-indigo-700 p-6 text-white">
        <div class="flex items-center justify-between">
          <div class="flex items-center space-x-4">
            <!-- 头像 -->
            <div class="relative">
              <div
                class="h-20 w-20 rounded-full bg-white/20 backdrop-blur-sm border-2 border-white/30 overflow-hidden"
              >
                <!-- 头像图片 -->
                <img
                  v-if="summonerInfo.profileIconId && !imageLoadError"
                  :src="getProfileIconUrl(summonerInfo.profileIconId)"
                  :alt="`${summonerInfo.displayName} 的头像`"
                  class="w-full h-full object-cover transition-opacity duration-300"
                  :class="{ 'opacity-0': imageLoading }"
                  @error="handleImageError"
                  @load="handleImageLoad"
                />

                <!-- 加载中的骨架屏 -->
                <div
                  v-if="imageLoading && summonerInfo.profileIconId && !imageLoadError"
                  class="absolute inset-0 w-full h-full flex items-center justify-center"
                >
                  <div
                    class="w-6 h-6 border-2 border-white/30 border-t-white rounded-full animate-spin"
                  ></div>
                </div>

                <!-- 备用显示（无头像ID或加载失败时） -->
                <div
                  v-if="
                    !summonerInfo.profileIconId ||
                    imageLoadError ||
                    (!imageLoading && imageLoadError)
                  "
                  class="w-full h-full flex items-center justify-center text-white font-bold text-2xl"
                >
                  {{ summonerInfo.displayName.charAt(0).toUpperCase() }}
                </div>
              </div>
              <div
                class="absolute -bottom-1 -right-1 h-8 w-8 rounded-full bg-blue-500 flex items-center justify-center text-white text-sm font-bold border-2 border-white"
              >
                {{ summonerInfo.summonerLevel }}
              </div>
              <!-- 挑战水晶等级 -->
              <div
                v-if="summonerInfo.challengeCrystalLevel"
                class="absolute -top-1 -left-1 h-6 w-6 rounded-full bg-yellow-500 flex items-center justify-center text-white text-xs font-bold border border-white"
              >
                {{ getChallengeIcon(summonerInfo.challengeCrystalLevel) }}
              </div>
            </div>

            <!-- 基本信息 -->
            <div>
              <h2 class="text-2xl font-bold text-white">{{ summonerInfo.displayName }}</h2>
              <p class="text-white/80">等级 {{ summonerInfo.summonerLevel }} 召唤师</p>
              <div class="flex items-center space-x-2 mt-2">
                <div class="h-2 w-2 rounded-full bg-green-400 animate-pulse"></div>
                <span class="text-white/90 font-medium">已连接</span>
                <span v-if="summonerInfo.availability" class="text-white/70 text-sm">
                  • {{ formatAvailability(summonerInfo.availability) }}
                </span>
              </div>
            </div>
          </div>

          <!-- 挑战点数和会话时长 -->
          <div class="text-right text-white">
            <div v-if="summonerInfo.challengePoints" class="mb-2">
              <p class="text-white/80 text-sm">挑战点数</p>
              <p class="text-xl font-bold">
                {{ formatChallengePoints(summonerInfo.challengePoints) }}
              </p>
            </div>
            <div>
              <p class="text-white/80 text-sm">会话时长</p>
              <p class="text-xl font-bold">{{ sessionDuration }}</p>
            </div>
          </div>
        </div>

        <!-- 经验条 -->
        <div v-if="summonerInfo.percentCompleteForNextLevel" class="mt-4">
          <div class="flex justify-between text-white/80 text-sm mb-1">
            <span>升级进度</span>
            <span>{{ summonerInfo.percentCompleteForNextLevel }}%</span>
          </div>
          <div class="w-full bg-white/20 rounded-full h-2">
            <div
              class="bg-white rounded-full h-2 transition-all duration-300"
              :style="{ width: `${summonerInfo.percentCompleteForNextLevel}%` }"
            ></div>
          </div>
          <div class="flex justify-between text-white/60 text-xs mt-1">
            <span>{{ summonerInfo.xpSinceLastLevel }} XP</span>
            <span>还需 {{ summonerInfo.xpUntilNextLevel }} XP</span>
          </div>
        </div>
      </div>

      <!-- 排位信息部分 -->
      <div class="p-6 bg-background">
        <h3 class="text-lg font-semibold mb-4 flex items-center">
          <Trophy class="h-5 w-5 mr-2 text-yellow-500" />
          排位统计
        </h3>

        <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
          <!-- 单人排位 -->
          <div class="space-y-3">
            <h4 class="font-medium text-foreground flex items-center">
              <User class="h-4 w-4 mr-2" />
              单人排位
            </h4>
            <div v-if="summonerInfo.soloRankTier" class="space-y-2">
              <div class="flex items-center space-x-3">
                <!-- <img
                  :src="getRankIconUrl(summonerInfo.soloRankTier)"
                  :alt="formatRankTier(summonerInfo.soloRankTier)"
                  class="h-12 w-12"
                /> -->
                <div class="flex items-center space-x-3">
                  <div
                    :class="[
                      'px-3 py-1.5 rounded-lg text-sm font-bold',
                      getRankColor(summonerInfo.soloRankTier)
                    ]"
                  >
                    {{ formatRankTier(summonerInfo.soloRankTier) }}
                    {{ summonerInfo.soloRankDivision }}
                  </div>
                  <span class="text-sm text-muted-foreground"
                    >{{ summonerInfo.soloRankLP }} LP</span
                  >
                </div>
              </div>
              <div class="flex items-center space-x-4 text-sm">
                <span class="text-green-600 dark:text-green-400 font-medium"
                  >{{ summonerInfo.soloRankWins }}胜</span
                >
                <span class="text-red-600 dark:text-red-400 font-medium"
                  >{{ summonerInfo.soloRankLosses }}负</span
                >
                <span class="text-muted-foreground">
                  胜率 {{ getRankWinRate(summonerInfo.soloRankWins, summonerInfo.soloRankLosses) }}%
                </span>
              </div>
            </div>
            <div v-else class="text-sm text-muted-foreground">
              <div class="flex items-center">
                <Shield class="h-4 w-4 mr-2" />
                <span>未定级</span>
              </div>
            </div>
          </div>

          <!-- 灵活排位 -->
          <div class="space-y-3">
            <h4 class="font-medium text-foreground flex items-center">
              <Users class="h-4 w-4 mr-2" />
              灵活排位
            </h4>
            <div v-if="summonerInfo.flexRankTier" class="space-y-2">
              <div class="flex items-center space-x-3">
                <!-- <img
                  :src="getRankIconUrl(summonerInfo.flexRankTier)"
                  :alt="formatRankTier(summonerInfo.flexRankTier)"
                  class="h-12 w-12"
                /> -->
                <div class="flex items-center space-x-3">
                  <div
                    :class="[
                      'px-3 py-1.5 rounded-lg text-sm font-bold',
                      getRankColor(summonerInfo.flexRankTier)
                    ]"
                  >
                    {{ formatRankTier(summonerInfo.flexRankTier) }}
                    {{ summonerInfo.flexRankDivision }}
                  </div>
                  <span class="text-sm text-muted-foreground"
                    >{{ summonerInfo.flexRankLP }} LP</span
                  >
                </div>
              </div>
              <div class="flex items-center space-x-4 text-sm">
                <span class="text-green-600 dark:text-green-400 font-medium"
                  >{{ summonerInfo.flexRankWins }}胜</span
                >
                <span class="text-red-600 dark:text-red-400 font-medium"
                  >{{ summonerInfo.flexRankLosses }}负</span
                >
                <span class="text-muted-foreground">
                  胜率 {{ getRankWinRate(summonerInfo.flexRankWins, summonerInfo.flexRankLosses) }}%
                </span>
              </div>
            </div>
            <div v-else class="text-sm text-muted-foreground">
              <div class="flex items-center">
                <Shield class="h-4 w-4 mr-2" />
                <span>未定级</span>
              </div>
            </div>
          </div>
        </div>

        <!-- 天赋信息 -->
        <!-- <div v-if="summonerInfo.currentPerkPage" class="mt-6">
          <h3 class="text-lg font-semibold mb-4 flex items-center">
            <Sparkles class="h-5 w-5 mr-2 text-purple-500" />
            当前天赋
          </h3>
          <div class="space-y-4">
            <div class="flex items-center space-x-4">
              <div class="flex items-center space-x-2">
                <img
                  :src="getPerkIconUrl(summonerInfo.primaryStyleId)"
                  :alt="getPerkName(summonerInfo.primaryStyleId)"
                  class="h-8 w-8"
                />
                <span class="font-medium">{{ getPerkStyleName(summonerInfo.primaryStyleId) }}</span>
              </div>
              <div class="flex items-center space-x-2">
                <img
                  :src="getPerkIconUrl(summonerInfo.subStyleId)"
                  :alt="getPerkName(summonerInfo.subStyleId)"
                  class="h-8 w-8"
                />
                <span class="font-medium">{{ getPerkStyleName(summonerInfo.subStyleId) }}</span>
              </div>
            </div>
            <div class="flex items-center space-x-2">
              <template v-for="perkId in summonerInfo.selectedPerkIds" :key="perkId">
                <img
                  :src="getPerkIconUrl(perkId)"
                  :alt="getPerkName(perkId)"
                  class="h-6 w-6"
                  :title="getPerkName(perkId)"
                />
              </template>
            </div>
            <p class="text-sm text-muted-foreground">{{ summonerInfo.currentPerkPage }}</p>
          </div>
        </div> -->

        <!-- 游戏状态和历史最高 -->
        <div class="flex items-center justify-between mt-6 pt-4 border-t border-border">
          <div v-if="summonerInfo.gameStatus" class="flex items-center space-x-2">
            <div class="h-2 w-2 rounded-full bg-green-500 animate-pulse"></div>
            <span
              :class="[
                'px-3 py-1 rounded-full text-sm font-medium',
                getGameStatusColor(summonerInfo.gameStatus)
              ]"
            >
              {{ formatGameStatus(summonerInfo.gameStatus) }}
            </span>
          </div>

          <div v-if="summonerInfo.highestRankThisSeason" class="text-sm text-muted-foreground">
            赛季最高: {{ formatRankTier(summonerInfo.highestRankThisSeason) }}
          </div>
        </div>
      </div>
    </Card>

    <!-- 顶部统计卡片 -->
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
      <!-- 游戏连接状态 -->
      <Card
        :class="[
          'relative p-6  transition-all duration-300',
          connectionStatus === 'connected'
            ? 'border-l-green-500 bg-green-50/50 dark:bg-green-950/20'
            : connectionStatus === 'connecting'
              ? 'border-l-yellow-500 bg-yellow-50/50 dark:bg-yellow-950/20'
              : 'border-l-red-500 bg-red-50/50 dark:bg-red-950/20'
        ]"
      >
        <div class="flex items-center justify-between">
          <div>
            <p class="text-sm font-medium text-muted-foreground">游戏连接</p>
            <h2
              :class="[
                'text-2xl font-bold',
                connectionStatus === 'connected'
                  ? 'text-green-600 dark:text-green-400'
                  : connectionStatus === 'connecting'
                    ? 'text-yellow-600 dark:text-yellow-400'
                    : 'text-red-600 dark:text-red-400'
              ]"
            >
              {{
                connectionStatus === 'connected'
                  ? '已连接'
                  : connectionStatus === 'connecting'
                    ? '连接中'
                    : '离线'
              }}
            </h2>
            <p class="text-xs text-muted-foreground mt-1">
              {{
                connectionStatus === 'connected'
                  ? '客户端已就绪'
                  : connectionStatus === 'connecting'
                    ? '正在连接...'
                    : '等待连接至League客户端'
              }}
            </p>
          </div>
          <div class="absolute top-4 right-4">
            <div
              :class="[
                'h-2 w-2 rounded-full',
                connectionStatus === 'connected'
                  ? 'bg-green-500'
                  : connectionStatus === 'connecting'
                    ? 'bg-yellow-500 animate-pulse'
                    : 'bg-red-500 animate-pulse'
              ]"
            ></div>
          </div>
        </div>
        <div class="mt-4">
          <Button
            v-if="!isConnected && !isConnecting"
            size="sm"
            variant="outline"
            class="text-xs"
            @click="attemptConnection"
          >
            <RefreshCw class="h-3 w-3 mr-1" />
            重新连接
          </Button>
          <div
            v-else-if="isConnecting"
            class="flex items-center text-sm text-yellow-600 dark:text-yellow-400"
          >
            <Loader2 class="h-3 w-3 mr-1 animate-spin" />
            连接中...
          </div>
          <div v-else class="text-sm text-green-600 dark:text-green-400">
            <Wifi class="h-3 w-3 inline mr-1" />
            连接正常
          </div>
        </div>
      </Card>

      <!-- 今日对局 -->
      <Card class="relative p-6">
        <div class="flex items-center justify-between">
          <div>
            <p class="text-sm font-medium text-muted-foreground">今日对局</p>
            <h2 class="text-2xl font-bold">{{ todayMatches.total }}</h2>
            <p class="text-xs text-muted-foreground mt-1">胜率 {{ winRate }}%</p>
          </div>
          <div class="absolute top-4 right-4">
            <TrendingUp class="h-4 w-4 text-muted-foreground" />
          </div>
        </div>
        <div class="mt-4 flex items-center space-x-4 text-sm">
          <div class="flex items-center">
            <div class="h-2 w-2 rounded-full bg-green-500 mr-1"></div>
            <span class="text-green-600 dark:text-green-400">{{ todayMatches.wins }}胜</span>
          </div>
          <div class="flex items-center">
            <div class="h-2 w-2 rounded-full bg-red-500 mr-1"></div>
            <span class="text-red-600 dark:text-red-400">{{ todayMatches.losses }}负</span>
          </div>
        </div>
      </Card>

      <!-- 自动功能 -->
      <Card class="relative p-6">
        <div class="flex items-center justify-between">
          <div>
            <p class="text-sm font-medium text-muted-foreground">自动功能</p>
            <h2 class="text-2xl font-bold">{{ enabledFunctionsCount }}</h2>
            <p class="text-xs text-muted-foreground mt-1">功能运行中</p>
          </div>
          <div class="absolute top-4 right-4">
            <Settings class="h-4 w-4 text-muted-foreground" />
          </div>
        </div>
        <div class="mt-4 text-sm">
          <div class="flex items-center space-x-1">
            <div
              :class="[
                'h-2 w-2 rounded-full',
                enabledFunctionsCount > 0 ? 'bg-green-500' : 'bg-gray-400'
              ]"
            ></div>
            <span
              :class="[
                enabledFunctionsCount > 0
                  ? 'text-green-600 dark:text-green-400'
                  : 'text-muted-foreground'
              ]"
            >
              {{ enabledFunctionsCount > 0 ? '自动化已启用' : '所有功能已停用' }}
            </span>
          </div>
        </div>
      </Card>

      <!-- 活跃时长 -->
      <Card class="relative p-6">
        <div class="flex items-center justify-between">
          <div>
            <p class="text-sm font-medium text-muted-foreground">活跃时长</p>
            <h2 class="text-2xl font-bold">{{ sessionDuration }}</h2>
            <p class="text-xs text-muted-foreground mt-1">本次会话</p>
          </div>
          <div class="absolute top-4 right-4">
            <Clock class="h-4 w-4 text-muted-foreground" />
          </div>
        </div>
        <div class="mt-4 text-sm text-muted-foreground">
          <div class="flex items-center">
            <Play class="h-3 w-3 mr-1" />
            <span>{{ formatTime(new Date()) }}</span>
          </div>
        </div>
      </Card>
    </div>

    <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
      <!-- 快捷功能 -->
      <Card class="p-6">
        <div class="space-y-4">
          <div class="flex items-center justify-between">
            <div>
              <h3 class="text-lg font-semibold">快捷功能</h3>
              <p class="text-sm text-muted-foreground">常用功能的快速访问</p>
            </div>
            <div class="flex space-x-2">
              <Button variant="outline" size="sm" class="text-xs" @click="debugLoginInfo">
                <Settings class="h-3 w-3 mr-1" />
                调试
              </Button>
              <Button variant="outline" size="sm" class="text-xs" @click="simulateMatch">
                <Play class="h-3 w-3 mr-1" />
                测试对局
              </Button>
            </div>
          </div>

          <div class="space-y-4">
            <!-- 自动接受对局 -->
            <div
              class="flex items-center justify-between p-4 rounded-lg border bg-card hover:bg-accent/50 transition-colors"
            >
              <div class="flex items-center space-x-3">
                <div class="p-2 rounded-lg bg-blue-500/10">
                  <Zap class="h-5 w-5 text-blue-500" />
                </div>
                <div>
                  <p class="font-medium">自动接受对局</p>
                  <p class="text-sm text-muted-foreground">自动接受匹配成功后的对局</p>
                </div>
              </div>
              <Switch
                :checked="autoFunctions.acceptMatch"
                @update:checked="() => toggleAutoFunction('acceptMatch')"
              />
            </div>

            <!-- 自动选择英雄 -->
            <div
              class="flex items-center justify-between p-4 rounded-lg border bg-card hover:bg-accent/50 transition-colors"
            >
              <div class="flex items-center space-x-3">
                <div class="p-2 rounded-lg bg-green-500/10">
                  <User class="h-5 w-5 text-green-500" />
                </div>
                <div>
                  <p class="font-medium">自动选择英雄</p>
                  <p class="text-sm text-muted-foreground">预设英雄自动选择</p>
                </div>
              </div>
              <Switch
                :checked="autoFunctions.selectChampion"
                @update:checked="() => toggleAutoFunction('selectChampion')"
              />
            </div>

            <!-- 自动符文配置 -->
            <div
              class="flex items-center justify-between p-4 rounded-lg border bg-card hover:bg-accent/50 transition-colors"
            >
              <div class="flex items-center space-x-3">
                <div class="p-2 rounded-lg bg-purple-500/10">
                  <Bookmark class="h-5 w-5 text-purple-500" />
                </div>
                <div>
                  <p class="font-medium">自动符文配置</p>
                  <p class="text-sm text-muted-foreground">根据英雄自动配置最优符文</p>
                </div>
              </div>
              <Switch
                :checked="autoFunctions.runeConfig"
                @update:checked="() => toggleAutoFunction('runeConfig')"
              />
            </div>

            <!-- 自动禁用英雄 -->
            <div
              class="flex items-center justify-between p-4 rounded-lg border bg-card hover:bg-accent/50 transition-colors"
            >
              <div class="flex items-center space-x-3">
                <div class="p-2 rounded-lg bg-red-500/10">
                  <Shield class="h-5 w-5 text-red-500" />
                </div>
                <div>
                  <p class="font-medium">自动禁用英雄</p>
                  <p class="text-sm text-muted-foreground">智能禁用敌方强势英雄</p>
                </div>
              </div>
              <Switch
                :checked="autoFunctions.banChampion"
                @update:checked="() => toggleAutoFunction('banChampion')"
              />
            </div>
          </div>
        </div>
      </Card>

      <!-- 最近活动 -->
      <Card class="p-6">
        <div class="space-y-4">
          <div class="flex items-center justify-between">
            <div>
              <h3 class="text-lg font-semibold">最近活动</h3>
              <p class="text-sm text-muted-foreground">最新的活动记录</p>
            </div>
            <div class="flex items-center space-x-2">
              <div class="h-2 w-2 rounded-full bg-green-500 animate-pulse"></div>
              <span class="text-xs text-muted-foreground">实时监控中</span>
            </div>
          </div>

          <div class="space-y-3 max-h-64 overflow-y-auto">
            <div
              v-for="activity in activities.slice(0, 8)"
              :key="activity.id"
              class="flex items-start space-x-3 p-2 rounded-lg hover:bg-muted/30 transition-colors"
            >
              <div
                :class="[
                  'h-2 w-2 rounded-full mt-2 flex-shrink-0',
                  activity.type === 'success'
                    ? 'bg-green-500'
                    : activity.type === 'info'
                      ? 'bg-blue-500'
                      : activity.type === 'warning'
                        ? 'bg-yellow-500'
                        : 'bg-red-500'
                ]"
              ></div>
              <div class="flex-1 min-w-0">
                <p class="text-sm font-medium text-foreground leading-tight">
                  {{ activity.message }}
                </p>
                <p class="text-xs text-muted-foreground">
                  {{ formatRelativeTime(activity.timestamp) }}
                </p>
              </div>
            </div>

            <div v-if="activities.length === 0" class="text-center py-8">
              <Clock class="h-8 w-8 text-muted-foreground mx-auto mb-2" />
              <p class="text-sm text-muted-foreground">暂无活动记录</p>
            </div>
          </div>
        </div>
      </Card>
    </div>

    <!-- 调试信息 -->
    <Card v-if="showDebugInfo && debugInfo" class="p-6">
      <div class="space-y-4">
        <div class="flex items-center justify-between">
          <div>
            <h3 class="text-lg font-semibold">API调试信息</h3>
            <p class="text-sm text-muted-foreground">LCU API响应数据</p>
          </div>
          <Button variant="outline" size="sm" class="text-xs" @click="showDebugInfo = false">
            关闭
          </Button>
        </div>

        <div class="space-y-4">
          <div v-for="(value, key) in debugInfo" :key="key" class="space-y-2">
            <h4 class="font-medium text-sm">{{ key }}</h4>
            <pre class="bg-muted p-4 rounded-lg text-xs overflow-x-auto">{{ value }}</pre>
          </div>
        </div>
      </div>
    </Card>
    <!-- 游戏统计 -->
    <Card class="p-6">
      <div class="space-y-6">
        <div class="flex items-center justify-between">
          <div>
            <h3 class="text-lg font-semibold flex items-center">
              <BarChart class="h-5 w-5 mr-2 text-blue-500" />
              游戏统计
            </h3>
            <p class="text-sm text-muted-foreground">近期游戏数据概览</p>
          </div>
          <Button
            :disabled="!isConnected || matchHistoryLoading"
            variant="outline"
            size="sm"
            @click="fetchMatchHistory"
          >
            <RefreshCw :class="['h-4 w-4 mr-2', { 'animate-spin': matchHistoryLoading }]" />
            {{ matchHistoryLoading ? '加载中...' : '刷新数据' }}
          </Button>
        </div>

        <!-- 加载状态 -->
        <div v-if="matchHistoryLoading" class="flex items-center justify-center py-16">
          <div class="text-center">
            <Loader2 class="h-12 w-12 animate-spin text-blue-500 mx-auto mb-4" />
            <p class="text-lg font-medium text-muted-foreground">正在分析对局数据...</p>
            <p class="text-sm text-muted-foreground">请稍候，这可能需要几秒钟</p>
          </div>
        </div>

        <!-- 未连接状态 -->
        <div v-else-if="!isConnected" class="flex items-center justify-center py-16">
          <div class="text-center">
            <Wifi class="h-12 w-12 text-muted-foreground mx-auto mb-4" />
            <p class="text-lg font-medium text-muted-foreground">需要连接到League客户端</p>
            <p class="text-sm text-muted-foreground">连接后即可查看详细的游戏统计</p>
          </div>
        </div>

        <!-- 无数据状态 -->
        <div v-else-if="!matchStatistics" class="flex items-center justify-center py-16">
          <div class="text-center">
            <BarChart class="h-12 w-12 text-muted-foreground mx-auto mb-4" />
            <p class="text-lg font-medium text-muted-foreground">暂无统计数据</p>
            <p class="text-sm text-muted-foreground">点击"刷新数据"获取最新的游戏统计</p>
          </div>
        </div>

        <!-- 统计数据展示 -->
        <div v-else class="space-y-6">
          <!-- 总体数据概览 -->
          <div class="grid grid-cols-2 md:grid-cols-4 gap-4">
            <div class="text-center p-4 rounded-lg bg-muted/30">
              <Trophy class="h-8 w-8 text-yellow-500 mx-auto mb-2" />
              <p class="text-2xl font-bold text-foreground">{{ matchStatistics.total_games }}</p>
              <p class="text-sm text-muted-foreground">总对局</p>
            </div>
            <div class="text-center p-4 rounded-lg bg-green-500/10">
              <Award class="h-8 w-8 text-green-500 mx-auto mb-2" />
              <p class="text-2xl font-bold text-green-600 dark:text-green-400">
                {{ matchStatistics.wins }}
              </p>
              <p class="text-sm text-muted-foreground">胜场</p>
            </div>
            <div class="text-center p-4 rounded-lg bg-red-500/10">
              <Target class="h-8 w-8 text-red-500 mx-auto mb-2" />
              <p class="text-2xl font-bold text-red-600 dark:text-red-400">
                {{ matchStatistics.losses }}
              </p>
              <p class="text-sm text-muted-foreground">负场</p>
            </div>
            <div class="text-center p-4 rounded-lg bg-blue-500/10">
              <TrendingUp class="h-8 w-8 text-blue-500 mx-auto mb-2" />
              <p class="text-2xl font-bold text-blue-600 dark:text-blue-400">
                {{ matchStatistics.win_rate.toFixed(1) }}%
              </p>
              <p class="text-sm text-muted-foreground">胜率</p>
            </div>
          </div>

          <!-- KDA统计 -->
          <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
            <div class="space-y-4">
              <h4 class="font-semibold flex items-center">
                <Swords class="h-4 w-4 mr-2 text-red-500" />
                KDA统计
              </h4>
              <div class="grid grid-cols-3 gap-4">
                <div class="text-center p-3 rounded-lg border">
                  <p class="text-lg font-bold text-foreground">
                    {{ matchStatistics.avg_kills.toFixed(1) }}
                  </p>
                  <p class="text-xs text-muted-foreground">平均击杀</p>
                </div>
                <div class="text-center p-3 rounded-lg border">
                  <p class="text-lg font-bold text-foreground">
                    {{ matchStatistics.avg_deaths.toFixed(1) }}
                  </p>
                  <p class="text-xs text-muted-foreground">平均死亡</p>
                </div>
                <div class="text-center p-3 rounded-lg border">
                  <p class="text-lg font-bold text-foreground">
                    {{ matchStatistics.avg_assists.toFixed(1) }}
                  </p>
                  <p class="text-xs text-muted-foreground">平均助攻</p>
                </div>
              </div>
              <div class="text-center p-3 rounded-lg bg-purple-500/10">
                <p class="text-xl font-bold text-purple-600 dark:text-purple-400">
                  {{ matchStatistics.avg_kda.toFixed(2) }}
                </p>
                <p class="text-sm text-muted-foreground">平均KDA</p>
              </div>
            </div>

            <!-- 常用英雄 -->
            <div class="space-y-4">
              <h4 class="font-semibold flex items-center">
                <Star class="h-4 w-4 mr-2 text-yellow-500" />
                常用英雄
              </h4>
              <div class="space-y-2">
                <div
                  v-for="champion in matchStatistics.favorite_champions.slice(0, 5)"
                  :key="champion.champion_name"
                  class="flex items-center justify-between p-2 rounded-lg border"
                >
                  <div class="flex items-center space-x-2">
                    <div
                      class="h-8 w-8 rounded-full bg-blue-500/20 flex items-center justify-center"
                    >
                      <span class="text-xs font-bold">{{ champion.champion_name.charAt(0) }}</span>
                    </div>
                    <div>
                      <p class="font-medium text-sm">{{ champion.champion_name }}</p>
                      <p class="text-xs text-muted-foreground">{{ champion.games_played }}场</p>
                    </div>
                  </div>
                  <div class="text-right">
                    <p
                      class="text-sm font-bold"
                      :class="[
                        champion.win_rate >= 60
                          ? 'text-green-600 dark:text-green-400'
                          : champion.win_rate >= 50
                            ? 'text-yellow-600 dark:text-yellow-400'
                            : 'text-red-600 dark:text-red-400'
                      ]"
                    >
                      {{ champion.win_rate.toFixed(0) }}%
                    </p>
                    <p class="text-xs text-muted-foreground">{{ champion.wins }}胜</p>
                  </div>
                </div>
              </div>
            </div>
          </div>

          <!-- 最近对局 -->
          <div class="space-y-4" v-if="matchStatistics.recent_performance.length > 0">
            <h4 class="font-semibold flex items-center">
              <Calendar class="h-4 w-4 mr-2 text-blue-500" />
              最近对局
            </h4>
            <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-3">
              <div
                v-for="game in matchStatistics.recent_performance.slice(0, 6)"
                :key="game.game_creation"
                :class="[
                  game.win
                    ? 'bg-green-500/10 border-green-500/30 hover:border-green-500/50'
                    : 'bg-red-500/10 border-red-500/30 hover:border-red-500/50'
                ]"
                class="p-3 rounded-lg border cursor-pointer transition-all duration-200 hover:shadow-md hover:scale-[1.02]"
                @click="openGameDetail(game)"
              >
                <div class="flex items-center justify-between mb-2">
                  <span class="text-sm font-medium">{{ game.champion_name }}</span>
                  <Badge :variant="game.win ? 'default' : 'destructive'" class="text-xs">
                    {{ game.win ? '胜利' : '失败' }}
                  </Badge>
                </div>
                <div class="flex items-center justify-between text-sm">
                  <span class="text-muted-foreground">{{ formatGameMode(game.game_mode) }}</span>
                  <span class="font-mono"
                    >{{ game.kills }}/{{ game.deaths }}/{{ game.assists }}</span
                  >
                </div>
                <div class="text-xs text-muted-foreground mt-1">
                  {{ formatGameTime(game.game_duration) }}
                </div>
              </div>
            </div>
          </div>
          <div v-else class="text-center text-muted-foreground py-8">
            <div class="text-3xl mb-2">🎮</div>
            <p>暂无对局记录</p>
          </div>
        </div>
      </div>
    </Card>

    <!-- 游戏详细信息Dialog -->
    <Dialog v-model:open="dialogOpen">
      <DialogContent class="!max-w-[80vw] w-[80vw]">
        <DialogHeader>
          <DialogTitle>游戏详细信息</DialogTitle>
          <DialogDescription v-if="selectedGame">
            {{ selectedGame.champion_name }} -
            {{ formatGameMode(selectedGame.game_mode as string) }} -
            {{ formatRelativeTime(selectedGame.game_creation as number) }}
          </DialogDescription>
        </DialogHeader>

        <ScrollArea class="max-h-[60vh]">
          <!-- 加载状态 -->
          <div v-if="gameDetailLoading" class="flex items-center justify-center py-12">
            <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-primary"></div>
            <span class="ml-3 text-muted-foreground">正在加载游戏详细信息...</span>
          </div>

          <!-- 详细信息内容 -->
          <div v-else-if="gameDetailData" class="space-y-6">
            <!-- 基本游戏信息 -->
            <Card>
              <div class="p-4">
                <h4 class="font-semibold mb-3">基本信息</h4>
                <div class="grid grid-cols-2 md:grid-cols-4 gap-4 text-sm">
                  <div>
                    <span class="text-muted-foreground">游戏ID:</span>
                    <p class="font-mono">{{ gameDetailData.gameId }}</p>
                  </div>
                  <div>
                    <span class="text-muted-foreground">游戏时长:</span>
                    <p>{{ formatDuration(gameDetailData.gameDuration || 0) }}</p>
                  </div>
                  <div>
                    <span class="text-muted-foreground">地图:</span>
                    <p>{{ getMapName(gameDetailData.mapId) }}</p>
                  </div>
                  <div>
                    <span class="text-muted-foreground">游戏模式:</span>
                    <p>{{ formatGameMode(gameDetailData.gameMode || '') }}</p>
                  </div>
                  <div>
                    <span class="text-muted-foreground">队列类型:</span>
                    <p>{{ getQueueName(gameDetailData.queueId || 0) }}</p>
                  </div>
                  <div>
                    <span class="text-muted-foreground">游戏版本:</span>
                    <p class="text-xs font-mono">{{ gameDetailData.gameVersion }}</p>
                  </div>
                </div>
              </div>
            </Card>

            <!-- 队伍信息 -->
            <div class="grid grid-cols-1 gap-6">
              <!-- 蓝队 -->
              <Card class="bg-blue-50/50 dark:bg-blue-950/30">
                <div
                  class="px-4 py-2 flex items-center font-bold text-blue-700 dark:text-blue-200 border-b border-blue-200 dark:border-blue-800"
                >
                  <span class="mr-2">蓝队 ({{ getTeamResult('100') }})</span>
                  <span class="ml-auto text-xs font-normal flex items-center">
                    击杀: {{ gameDetailData?.blueTeamStats?.kills || 0 }} | 经济:
                    {{ formatNumber(gameDetailData?.blueTeamStats?.gold_earned || 0) }} | 伤害:
                    {{
                      formatNumber(
                        gameDetailData?.blueTeamStats?.total_damage_dealt_to_champions || 0
                      )
                    }}
                    | 视野: {{ gameDetailData?.blueTeamStats?.vision_score || 0 }} | BAN:
                    <span
                      v-for="ban in getTeamBans('100', gameDetailData?.teams)"
                      :key="ban.championId"
                      class="inline-block mx-0.5"
                    >
                      <img
                        :src="getChampionIconUrl(ban.championId)"
                        class="h-6 w-6 rounded"
                        :title="getChampionName(ban.championId)"
                      />
                    </span>
                  </span>
                </div>
                <Table>
                  <TableHeader>
                    <TableRow>
                      <TableHead
                        v-for="column in columns"
                        :key="column.key"
                        :class="column.class"
                        >{{ column.label }}</TableHead
                      >
                    </TableRow>
                  </TableHeader>
                  <TableBody>
                    <TableRow
                      v-for="participant in getTeamParticipants('100', gameDetailData)"
                      :key="participant.participantId"
                    >
                      <TableCell class="flex items-center space-x-2">
                        <img
                          :src="
                            getProfileIconUrl(
                              getPlayerProfileIcon(participant.participantId, gameDetailData)
                            )
                          "
                          class="h-8 w-8 rounded-full"
                        />
                        <span class="font-medium truncate">{{
                          getPlayerDisplayName(participant.participantId, gameDetailData)
                        }}</span>
                        <img
                          v-if="participant.rankTier"
                          :src="getRankIconUrl(participant.rankTier)"
                          class="h-6 w-6"
                        />
                      </TableCell>
                      <TableCell class="relative">
                        <div class="flex items-center gap-1">
                          <div class="relative">
                            <img
                              :src="getChampionIconUrl(participant.championId)"
                              class="h-8 w-8"
                              :title="participant.championName"
                            />
                            <span
                              class="absolute -bottom-1 -right-1 bg-gray-900/75 text-white text-[10px] min-w-[16px] h-4 flex items-center justify-center rounded"
                            >
                              {{ participant.stats?.champLevel || '?' }}
                            </span>
                          </div>
                          <span class="text-sm font-medium">
                            {{ participant.championName }}
                          </span>
                        </div>
                      </TableCell>
                      <TableCell>
                        <div class="flex items-center justify-center gap-1 w-full">
                          <img
                            v-for="i in itemSlots"
                            :key="i"
                            :src="getItemIconUrl(participant.stats?.[`item${i}`])"
                            class="h-6 w-6 rounded bg-gray-100 dark:bg-gray-800"
                            :style="{
                              opacity: participant.stats?.[`item${i}`] ? 1 : 0.3
                            }"
                            :alt="
                              participant.stats?.[`item${i}`]
                                ? `装备 ${participant.stats[`item${i}`]}`
                                : '空装备槽'
                            "
                          />
                        </div>
                      </TableCell>
                      <TableCell class="text-center">
                        {{ participant.stats?.kills }}/{{ participant.stats?.deaths }}/{{
                          participant.stats?.assists
                        }}
                      </TableCell>
                      <TableCell class="text-center text-yellow-700 dark:text-yellow-300">
                        {{ formatNumber(participant.stats?.goldEarned || 0) }}
                      </TableCell>
                      <TableCell class="text-center text-blue-700 dark:text-blue-300">
                        {{ formatNumber(participant.stats?.totalDamageDealtToChampions || 0) }}
                      </TableCell>
                      <TableCell
                        class="text-center font-bold text-lg text-indigo-700 dark:text-indigo-300"
                      >
                        {{ participant.score || '-' }}
                      </TableCell>
                    </TableRow>
                  </TableBody>
                </Table>
              </Card>

              <!-- 红队 -->
              <Card class="bg-red-50/50 dark:bg-red-950/30">
                <div
                  class="px-4 py-2 flex items-center font-bold text-red-700 dark:text-red-200 border-b border-red-200 dark:border-red-800"
                >
                  <span class="mr-2">红队 ({{ getTeamResult('200') }})</span>
                  <span class="ml-auto text-xs font-normal flex items-center">
                    击杀: {{ gameDetailData?.redTeamStats?.kills || 0 }} | 经济:
                    {{ formatNumber(gameDetailData?.redTeamStats?.gold_earned || 0) }} | 伤害:
                    {{
                      formatNumber(
                        gameDetailData?.redTeamStats?.total_damage_dealt_to_champions || 0
                      )
                    }}
                    | 视野: {{ gameDetailData?.redTeamStats?.vision_score || 0 }} | BAN:
                    <span
                      v-for="ban in getTeamBans('200', gameDetailData?.teams)"
                      :key="ban.championId"
                      class="inline-block mx-0.5"
                    >
                      <img
                        :src="getChampionIconUrl(ban.championId)"
                        class="h-6 w-6 rounded"
                        :title="getChampionName(ban.championId)"
                      />
                    </span>
                  </span>
                </div>
                <Table>
                  <TableHeader>
                    <TableRow>
                      <TableHead
                        v-for="column in columns"
                        :key="column.key"
                        :class="column.class"
                        >{{ column.label }}</TableHead
                      >
                    </TableRow>
                  </TableHeader>
                  <TableBody>
                    <TableRow
                      v-for="participant in getTeamParticipants('200', gameDetailData)"
                      :key="participant.participantId"
                    >
                      <TableCell class="flex items-center space-x-2">
                        <img
                          :src="
                            getProfileIconUrl(
                              getPlayerProfileIcon(participant.participantId, gameDetailData)
                            )
                          "
                          class="h-8 w-8 rounded-full"
                        />
                        <span class="font-medium truncate">{{
                          getPlayerDisplayName(participant.participantId, gameDetailData)
                        }}</span>
                        <img
                          v-if="participant.rankTier"
                          :src="getRankIconUrl(participant.rankTier)"
                          class="h-6 w-6"
                        />
                      </TableCell>
                      <TableCell class="relative">
                        <div class="flex items-center gap-1">
                          <div class="relative">
                            <img
                              :src="getChampionIconUrl(participant.championId)"
                              class="h-8 w-8"
                              :title="participant.championName"
                            />
                            <span
                              class="absolute -bottom-1 -right-1 bg-gray-900/75 text-white text-[10px] min-w-[16px] h-4 flex items-center justify-center rounded"
                            >
                              {{ participant.stats?.champLevel || '?' }}
                            </span>
                          </div>
                          <span class="text-sm font-medium">
                            {{ participant.championName }}
                          </span>
                        </div>
                      </TableCell>
                      <TableCell>
                        <div class="flex items-center justify-center gap-1 w-full">
                          <img
                            v-for="i in itemSlots"
                            :key="i"
                            :src="getItemIconUrl(participant.stats?.[`item${i}`])"
                            class="h-6 w-6 rounded bg-gray-100 dark:bg-gray-800"
                            :style="{
                              opacity: participant.stats?.[`item${i}`] ? 1 : 0.3
                            }"
                            :alt="
                              participant.stats?.[`item${i}`]
                                ? `装备 ${participant.stats[`item${i}`]}`
                                : '空装备槽'
                            "
                          />
                        </div>
                      </TableCell>
                      <TableCell class="text-center">
                        {{ participant.stats?.kills }}/{{ participant.stats?.deaths }}/{{
                          participant.stats?.assists
                        }}
                      </TableCell>
                      <TableCell class="text-center text-yellow-700 dark:text-yellow-300">
                        {{ formatNumber(participant.stats?.goldEarned || 0) }}
                      </TableCell>
                      <TableCell class="text-center text-blue-700 dark:text-blue-300">
                        {{ formatNumber(participant.stats?.totalDamageDealtToChampions || 0) }}
                      </TableCell>
                      <TableCell
                        class="text-center font-bold text-lg text-indigo-700 dark:text-indigo-300"
                      >
                        {{ participant.score || '-' }}
                      </TableCell>
                    </TableRow>
                  </TableBody>
                </Table>
              </Card>
            </div>

            <!-- 单项最佳 -->
            <div class="flex gap-4 mt-4">
              <Card class="flex-1 p-4 text-center">
                <img
                  :src="getChampionIconUrl(gameDetailData.bestPlayerChampionId as number)"
                  class="h-10 w-10 mx-auto"
                />
                <div class="font-bold text-lg mt-2">{{ gameDetailData.maxDamage }}</div>
                <div class="text-xs text-muted-foreground">最高英雄伤害</div>
              </Card>
              <Card class="flex-1 p-4 text-center">
                <img
                  :src="getChampionIconUrl(gameDetailData.maxTankChampionId as number)"
                  class="h-10 w-10 mx-auto"
                />
                <div class="font-bold text-lg mt-2">{{ gameDetailData.maxTank }}</div>
                <div class="text-xs text-muted-foreground">最高承受伤害</div>
              </Card>
              <Card class="flex-1 p-4 text-center">
                <img
                  :src="getChampionIconUrl(gameDetailData.maxStreakChampionId as number)"
                  class="h-10 w-10 mx-auto"
                />
                <div class="font-bold text-lg mt-2">{{ gameDetailData.maxStreak }}</div>
                <div class="text-xs text-muted-foreground">最多连杀</div>
              </Card>
            </div>
          </div>
        </ScrollArea>
      </DialogContent>
    </Dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch, computed } from 'vue'
import { storeToRefs } from 'pinia'
import { invoke } from '@tauri-apps/api/core'
import { Card } from '@/components/ui/card'
import { Switch } from '@/components/ui/switch'
import { Button } from '@/components/ui/button'
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle
} from '@/components/ui/dialog'
import { Badge } from '@/components/ui/badge'
import { ScrollArea } from '@/components/ui/scroll-area'
import {
  User,
  Users,
  Zap,
  Settings,
  Clock,
  Bookmark,
  BarChart,
  TrendingUp,
  RefreshCw,
  Loader2,
  Wifi,
  Play,
  Shield,
  Trophy,
  Target,
  Swords,
  Star,
  Calendar,
  Award,
  Sparkles
} from 'lucide-vue-next'
import { useGameStore } from '@/stores/gameStore'
import { useGameMonitor } from '@/composables/useGameMonitor'
import {
  Table,
  TableBody,
  TableCaption,
  TableCell,
  TableHead,
  TableHeader,
  TableRow
} from '@/components/ui/table'

// 使用store和监控
const gameStore = useGameStore()
const { attemptConnection } = useGameMonitor()

// 格式化相对时间
const formatRelativeTime = (timestamp: number) => {
  const date = new Date(timestamp)
  const now = new Date()
  const diff = now.getTime() - date.getTime()
  const hours = Math.floor(diff / (1000 * 60 * 60))

  if (hours < 1) {
    return '刚刚'
  } else if (hours < 24) {
    return `${hours}小时前`
  } else {
    const days = Math.floor(hours / 24)
    return `${days}天前`
  }
}
// 从store中解构响应式状态
const {
  isConnected,
  isConnecting,
  summonerInfo,
  todayMatches,
  activities,
  autoFunctions,
  connectionStatus,
  winRate,
  enabledFunctionsCount,
  sessionDuration,
  matchStatistics,
  matchHistoryLoading
} = storeToRefs(gameStore)

// 组件挂载时获取游戏版本和对局历史
onMounted(() => {
  fetchGameVersion()

  // 如果已连接，自动获取对局历史
  if (isConnected.value) {
    fetchMatchHistory()
  }
})

// 监听召唤师信息变化，重置头像状态
watch(summonerInfo, () => {
  imageLoadError.value = false
  imageLoading.value = true
})

// 监听连接状态变化，自动获取对局历史
watch(isConnected, newValue => {
  if (newValue && !matchStatistics.value) {
    fetchMatchHistory()
  }
})

// 方法
const { toggleAutoFunction, simulateMatchResult, fetchMatchHistory } = gameStore

// 格式化时间
const formatTime = (date: Date) => {
  return date.toLocaleTimeString('zh-CN', {
    hour: '2-digit',
    minute: '2-digit'
  })
}

// 调试状态
const debugInfo = ref<Record<string, unknown> | null>(null)
const showDebugInfo = ref(false)

// 头像相关状态
const imageLoadError = ref(false)
const imageLoading = ref(true)
const gameVersion = ref('14.23.1') // 默认版本，将从API获取最新版本

// 游戏详细信息相关状态
const gameDetailData = ref<GameDetailData | null>(null)
const gameDetailLoading = ref(false)
const dialogOpen = ref(false)
const selectedGame = ref<RecentGame | null>(null)

// 添加符文相关的状态
const perksInfo = ref<PerkInfo[]>([])

// 获取符文信息
const fetchPerksInfo = async () => {
  console.log('🔄 开始获取符文信息...')
  try {
    perksInfo.value = await invoke<PerkInfo[]>('get_perks_info')
    console.log('✅ 成功获取符文信息:', perksInfo.value.length, '个符文')
    console.log('📋 符文列表:', perksInfo.value)
  } catch (error) {
    console.error('❌ 获取符文信息失败:', error)
  }
}

// 在组件挂载时获取符文信息
onMounted(() => {
  fetchPerksInfo()
})

// 获取游戏版本
const fetchGameVersion = async (): Promise<void> => {
  try {
    const version = await invoke<string>('get_game_version')
    gameVersion.value = version
    console.log('获取到游戏版本:', version)
  } catch (error) {
    console.warn('获取游戏版本失败，使用默认版本:', error)
  }
}

// 调试登录信息
const debugLoginInfo = async (): Promise<void> => {
  try {
    gameStore.addActivity('info', '开始调试API信息...')
    const result = await invoke('debug_login_info')
    debugInfo.value = result as any
    showDebugInfo.value = true
    console.log('调试信息:', result)
    gameStore.addActivity('success', '调试信息获取成功，请查看控制台')
  } catch (error) {
    console.error('调试失败:', error)
    gameStore.addActivity('error', `调试失败: ${error}`)
  }
}

// 格式化排位等级
const formatRankTier = (tier: string): string => {
  const tierMap: Record<string, string> = {
    IRON: '坚韧黑铁',
    BRONZE: '英勇青铜',
    SILVER: '不屈白银',
    GOLD: '荣耀黄金',
    PLATINUM: '华贵铂金',
    EMERALD: '流光翡翠',
    DIAMOND: '璀璨钻石',
    MASTER: '超凡大师',
    GRANDMASTER: '傲世宗师',
    CHALLENGER: '最强王者'
  }
  return tierMap[tier] || tier
}

// 获取排位颜色
const getRankColor = (tier: string): string => {
  const colorMap: Record<string, string> = {
    IRON: 'bg-zinc-500/20 text-zinc-600 dark:text-zinc-400',
    BRONZE: 'bg-orange-500/20 text-orange-600 dark:text-orange-400',
    SILVER: 'bg-slate-500/20 text-slate-600 dark:text-slate-400',
    GOLD: 'bg-yellow-500/20 text-yellow-600 dark:text-yellow-400',
    PLATINUM: 'bg-cyan-500/20 text-cyan-600 dark:text-cyan-400',
    EMERALD: 'bg-emerald-500/20 text-emerald-600 dark:text-emerald-400',
    DIAMOND: 'bg-blue-500/20 text-blue-600 dark:text-blue-400',
    MASTER: 'bg-purple-500/20 text-purple-600 dark:text-purple-400',
    GRANDMASTER: 'bg-red-500/20 text-red-600 dark:text-red-400',
    CHALLENGER: 'bg-yellow-500/20 text-yellow-600 dark:text-yellow-400'
  }
  return colorMap[tier] || 'bg-gray-500/20 text-gray-600 dark:text-gray-400'
}

// 计算胜率
const getRankWinRate = (wins?: number, losses?: number): number => {
  if (!wins && !losses) return 0
  const totalGames = (wins || 0) + (losses || 0)
  if (totalGames === 0) return 0
  return Math.round(((wins || 0) / totalGames) * 100)
}

// 格式化游戏状态
const formatGameStatus = (status: string): string => {
  const statusMap: Record<string, string> = {
    hosting_RANKED_SOLO_5x5: '排位单双',
    hosting_NORMAL: '匹配模式',
    hosting_ARAM: '大乱斗',
    inGame: '游戏中',
    outOfGame: '客户端',
    away: '离开',
    online: '在线'
  }
  return statusMap[status] || status
}

// 获取游戏状态颜色
const getGameStatusColor = (status: string): string => {
  if (status.includes('hosting') || status === 'inGame') {
    return 'bg-green-500/20 text-green-600 dark:text-green-400'
  }
  if (status === 'away') {
    return 'bg-yellow-500/20 text-yellow-600 dark:text-yellow-400'
  }
  return 'bg-blue-500/20 text-blue-600 dark:text-blue-400'
}

// 格式化可用性状态
const formatAvailability = (availability: string): string => {
  const availabilityMap: Record<string, string> = {
    chat: '可聊天',
    away: '离开',
    dnd: '勿扰',
    online: '在线',
    mobile: '手机在线',
    offline: '离线'
  }
  return availabilityMap[availability] || availability
}

// 获取挑战水晶图标
const getChallengeIcon = (level: string): string => {
  const iconMap: Record<string, string> = {
    IRON: '🥉',
    BRONZE: '🥉',
    SILVER: '🥈',
    GOLD: '🥇',
    PLATINUM: '💎',
    DIAMOND: '💎',
    MASTER: '👑',
    GRANDMASTER: '👑',
    CHALLENGER: '⭐'
  }
  return iconMap[level] || '🏆'
}

// 格式化游戏时长
const formatGameTime = (seconds: number): string => {
  const minutes = Math.floor(seconds / 60)
  const remainingSeconds = seconds % 60
  return `${minutes}:${remainingSeconds.toString().padStart(2, '0')}`
}

// 获取头像URL
const getProfileIconUrl = (iconId: number): string => {
  if (!iconId) return ''
  return `https://raw.communitydragon.org/latest/plugins/rcp-be-lol-game-data/global/default/v1/profile-icons/${iconId}.jpg`
}

// 处理图片加载错误
const handleImageError = (event: Event): void => {
  console.warn('头像加载失败:', event)
  imageLoadError.value = true
  imageLoading.value = false
}

// 处理图片加载成功
const handleImageLoad = (): void => {
  imageLoadError.value = false
  imageLoading.value = false
}

// 模拟对局（用于测试）
const simulateMatch = (): void => {
  const won = Math.random() > 0.5
  simulateMatchResult(won)
}

// 打开游戏详细信息
const openGameDetail = async (game: RecentGame): Promise<void> => {
  selectedGame.value = game
  dialogOpen.value = true
  gameDetailLoading.value = true
  gameDetailData.value = null

  try {
    const result = await invoke<GameDetailData>('get_game_detail', {
      gameId: game.game_id
    })
    gameDetailData.value = result
  } catch (err) {
    console.error('获取游戏详细信息失败:', err)
    gameStore.addActivity('error', `获取游戏详细信息失败: ${err}`)
  } finally {
    gameDetailLoading.value = false
  }
}

// 格式化游戏模式
const formatGameMode = (mode: string): string => {
  const modeMap: Record<string, string> = {
    CLASSIC: '经典模式',
    ARAM: '大乱斗',
    URF: '无限火力',
    TUTORIAL: '教程',
    ONEFORALL: '克隆大作战',
    ARSR: '极地大乱斗',
    PRACTICETOOL: '训练工具',
    NEXUSBLITZ: '极地大乱斗'
  }
  return modeMap[mode] || mode
}

// 格式化游戏时长
const formatDuration = (seconds: number) => {
  const minutes = Math.floor(seconds / 60)
  const remainingSeconds = seconds % 60
  return `${minutes}:${remainingSeconds.toString().padStart(2, '0')}`
}

// 获取队列名称
const getQueueName = (queueId: number) => {
  const queueMap: Record<number, string> = {
    400: '5v5 征召模式',
    420: '5v5 排位赛',
    430: '5v5 匹配模式',
    440: '5v5 灵活排位',
    450: '5v5 大乱斗',
    900: '4v4 无限火力',
    1020: '克隆大作战',
    1200: '极地大乱斗'
  }
  return queueMap[queueId] || `队列 ${queueId}`
}

// 获取队伍结果
const getTeamResult = (teamId: string): string => {
  if (!gameDetailData.value?.teams) return ''
  const team = gameDetailData.value.teams.find((t: any) => t.teamId.toString() === teamId)
  if (!team) return ''
  return team.win === 'Win' ? '胜方' : '败方'
}

// 获取队伍参与者
const getTeamParticipants = (teamId: string, gameDetail: GameDetailData): Participant[] => {
  if (!gameDetail?.participants) return []
  return gameDetail.participants.filter(p => p.teamId.toString() === teamId)
}

// 获取英雄名称 (暂时使用英雄ID，后续可以添加英雄名称映射)
const getChampionName = (championId: number): string => {
  const championMap: Record<number, string> = {
    1: '安妮',
    2: '奥拉夫',
    3: '加里奥',
    4: '崔丝塔娜',
    5: '瑞兹',
    6: '乌迪尔',
    7: '乐芙兰',
    8: '弗拉基米尔',
    9: '费德提克',
    10: '凯尔',
    11: '易',
    12: '阿利斯塔',
    13: '瑞兹',
    14: '赛恩',
    15: '希维尔',
    16: '索拉卡',
    17: '提莫',
    18: '崔丝塔娜',
    19: '沃里克',
    20: '努努和威朗普',
    21: '厄运小姐',
    22: '阿什',
    23: '泰达米尔',
    24: '贾克斯',
    25: '莫甘娜',
    26: '泽拉斯',
    27: '辛吉德',
    28: '伊芙琳',
    29: '图奇',
    30: '卡尔萨斯',
    31: '科加斯',
    32: '阿木木',
    33: '拉莫斯',
    34: '安妮',
    35: '费德提克',
    36: '努努和威朗普',
    37: '索拉卡',
    38: '凯尔',
    39: '易',
    40: '贾克斯',
    41: '莫甘娜',
    42: '科加斯',
    43: '卡尔玛',
    44: '塔里克',
    45: '维迦',
    48: '崔丝塔娜',
    50: '斯维因',
    51: '凯特琳',
    53: '布隆',
    54: '马尔扎哈',
    55: '卡西奥佩娅',
    56: '诺提勒斯',
    57: '莫菲特',
    58: '雷克顿',
    59: '卡萨丁',
    60: '卡兹克',
    61: '奥莉安娜',
    62: '蒙多医生',
    63: '布兰德',
    64: '李青',
    67: '薇恩',
    68: '兰博',
    69: '卡萨丁',
    72: '斯卡纳',
    74: '海克斯科技',
    75: '纳瑟斯',
    76: '奈德丽',
    77: '乌迪尔',
    78: '波比',
    79: '古拉加斯',
    80: '潘森',
    81: '伊泽瑞尔',
    82: '莫德凯撒',
    83: '约里克',
    84: '阿卡丽',
    85: '凯南',
    86: '盖伦',
    89: '蕾欧娜',
    90: '马尔扎哈',
    91: '塔隆',
    92: '瑞兹',
    96: '科加斯',
    98: '慎',
    99: '拉克丝',
    101: '泽拉斯',
    102: '希维尔',
    103: '阿狸',
    104: '格雷福斯',
    105: '费德提克',
    106: '沃利贝尔',
    107: '雷克顿',
    110: '韦鲁斯',
    111: '诺提勒斯',
    112: '维克托',
    113: '瑟庄妮',
    114: '费德提克',
    115: '希维尔',
    117: '璐璐',
    119: '德莱文',
    120: '赫卡里姆',
    121: '卡萨丁',
    122: '德莱厄斯',
    126: '杰斯',
    127: '丽桑卓',
    131: '戴安娜',
    133: '奎因',
    134: '辛德拉',
    136: '奥瑞利安·索尔',
    141: '凯隐',
    142: '佐伊',
    143: '赛恩',
    145: '卡莉丝塔',
    147: '瑟庄妮',
    150: '纳尔',
    154: '扎克',
    157: '亚索',
    161: '维鲁斯',
    163: '塔莉垭',
    164: '卡米尔',
    166: '阿克尚',
    200: '贝尔维斯',
    201: '布隆',
    202: '金克丝',
    203: '金克丝',
    221: '泽丽',
    222: '金克丝',
    223: '塔姆',
    234: '维戈',
    235: '塞娜',
    236: '卢锡安',
    238: '劫',
    240: '克烈',
    245: '艾克',
    246: '奇亚娜',
    254: '薇恩',
    266: '阿托克斯',
    267: '娜美',
    268: '阿兹尔',
    350: '悠米',
    360: '萨米拉',
    412: '布隆',
    420: '伊莉丝',
    421: '雷克塞',
    427: '艾翁',
    429: '卡莉丝塔',
    432: '巴德',
    497: '蕾欧娜',
    498: '纳尔',
    516: '奥恩',
    517: '赛娜',
    518: '妮蔻',
    523: '厄斐琉斯',
    526: '奥恩',
    555: '派克',
    711: '薇古丝',
    777: '永恩',
    875: '瑟提',
    876: '莉莉娅',
    887: '格温',
    888: '泽丽',
    895: '尼菈',
    897: '奎桑提',
    901: '魔腾',
    902: '米利欧',
    950: '纳亚菲利',
    955: '米利欧',
    958: '奎桑提',
    959: '纳亚菲利'
  }
  return championMap[championId] || `英雄${championId}`
}

// 获取装备图标URL
const getItemIconUrl = (itemId: unknown): string => {
  if (!itemId || typeof itemId !== 'number' || itemId === 0) return ''
  return `https://ddragon.leagueoflegends.com/cdn/${gameVersion.value}/img/item/${itemId}.png`
}

// 获取玩家头像
const getPlayerProfileIcon = (participantId: number, gameDetail: GameDetailData): number => {
  const identity = gameDetail.participantIdentities?.find(id => id.participantId === participantId)
  return identity?.player?.profileIcon || 0
}

// 获取队伍禁用英雄
const getTeamBans = (teamId: string, teams: Team[]): TeamBan[] => {
  if (!teams) return []
  const team = teams.find(t => t.teamId.toString() === teamId)
  return team?.bans || []
}

// 获取玩家显示名称
const getPlayerDisplayName = (participantId: number, gameDetail: GameDetailData): string => {
  const identity = gameDetail.participantIdentities?.find(id => id.participantId === participantId)
  console.log('participantId', participantId)
  console.log('gameDetail', gameDetail)
  if (!identity?.player) return '未知玩家'

  const { gameName, tagLine, summonerName } = identity.player
  if (gameName && tagLine) {
    return `${gameName}#${tagLine}`
  }
  return summonerName || '未知玩家'
}

// 获取地图名称
const getMapName = (mapId: number): string => {
  const mapNames: Record<number, string> = {
    11: '召唤师峡谷',
    12: '嚎哭深渊',
    21: '纽克萨斯对战',
    22: '训练模式'
  }
  return mapNames[mapId] || `地图${mapId}`
}

// 格式化数字
const formatNumber = (num: number): string => {
  return num?.toLocaleString() || '0'
}

// 获取段位图标URL
const getRankIconUrl = (tier: string): string => {
  if (!tier) return ''
  const tierLower = tier.toLowerCase()
  return `https://raw.communitydragon.org/latest/plugins/rcp-fe-lol-leagues/global/default/images/gold.png`
}

// 获取符文系统图标URL
const getPerkStyleIconUrl = (styleId: number | undefined): string => {
  if (!styleId) {
    console.log('⚠️ 符文系统ID为空')
    return ''
  }
  const url = `https://raw.communitydragon.org/latest/plugins/rcp-be-lol-game-data/global/default/v1/perk-images/styles/icon/7200_${styleId}.png`
  console.log(`🔍 符文系统图标URL: ${url}`)
  return url
}

// 获取符文系统名称
const getPerkStyleName = (styleId: number | undefined): string => {
  if (!styleId) {
    console.log('⚠️ 符文系统ID为空')
    return '未知符文系统'
  }
  const styleMap: Record<number, string> = {
    8000: '精密',
    8100: '支配',
    8200: '巫术',
    8300: '启迪',
    8400: '坚决'
  }
  const name = styleMap[styleId] || `符文系统 ${styleId}`
  console.log(`📝 符文系统名称: ${name}-${styleId}`)
  return name
}

// 获取符文图标URL
const getPerkIconUrl = (perkId: number | undefined): string => {
  if (!perkId) {
    console.log('⚠️ 符文ID为空')
    return ''
  }
  const perk = perksInfo.value.find((p: any) => p.id === perkId)
  console.log('perk', perk)
  if (!perk) {
    console.log(`⚠️ 未找到ID为 ${perkId} 的符文`)
    return ''
  }
  const url = `https://raw.communitydragon.org/latest/game${perk.iconPath}`
  console.log(`🔍 符文图标URL: ${url}`)
  return url
}

// 获取符文名称
const getPerkName = (perkId: number | undefined): string => {
  if (!perkId) {
    console.log('⚠️ 符文ID为空')
    return '未知符文'
  }
  const perk = perksInfo.value.find(p => p.id === perkId)
  if (!perk) {
    console.log(`⚠️ 未找到ID为 ${perkId} 的符文`)
    return `符文 ${perkId}`
  }
  console.log(`📝 符文名称: ${perk.name}`)
  return perk.name
}

// 获取英雄图标URL
const getChampionIconUrl = (championId: number): string => {
  if (!championId) return ''
  return `https://raw.communitydragon.org/latest/plugins/rcp-be-lol-game-data/global/default/v1/champion-icons/${championId}.png`
}

// 格式化挑战点数
const formatChallengePoints = (points: string): string => {
  const num = parseInt(points)
  if (num >= 1000) {
    return `${(num / 1000).toFixed(1)}k`
  }
  return points
}

// 表格列定义
const columns = ref<Column[]>([
  {
    key: 'summoner',
    label: '召唤师',
    class: 'w-[200px]'
  },
  {
    key: 'champion',
    label: '英雄/等级',
    class: 'w-[120px]'
  },
  {
    key: 'items',
    label: '装备',
    class: 'w-[250px] text-center'
  },
  {
    key: 'kda',
    label: 'KDA',
    class: 'w-[100px] text-center'
  },
  {
    key: 'gold',
    label: '经济',
    class: 'w-[100px] text-center'
  },
  {
    key: 'damage',
    label: '伤害',
    class: 'w-[100px] text-center'
  },
  {
    key: 'score',
    label: '评分',
    class: 'w-[80px] text-center'
  }
])

// 装备槽位
const itemSlots = [0, 1, 2, 3, 4, 5, 6]
</script>
