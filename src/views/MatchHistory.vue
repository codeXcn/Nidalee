<template>
  <div class="p-6 space-y-6">
    <!-- 页面标题 -->
    <div class="flex items-center justify-between">
      <h1 class="text-2xl font-bold text-gray-900 dark:text-white">
        {{ searchedSummoner || '我的战绩' }}
      </h1>
      <button
        @click="refreshData"
        :disabled="loading"
        class="px-4 py-2 bg-blue-500 hover:bg-blue-600 text-white rounded-lg transition-colors disabled:opacity-50"
      >
        <svg
          v-if="loading"
          class="animate-spin -ml-1 mr-3 h-5 w-5 text-white inline"
          xmlns="http://www.w3.org/2000/svg"
          fill="none"
          viewBox="0 0 24 24"
        >
          <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
          <path
            class="opacity-75"
            fill="currentColor"
            d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
          ></path>
        </svg>
        {{ loading ? '加载中...' : '刷新数据' }}
      </button>
    </div>

    <!-- 搜索框 -->
    <div class="bg-white dark:bg-gray-800 rounded-lg shadow p-6">
      <h2 class="text-lg font-semibold text-gray-900 dark:text-white mb-4">战绩查询</h2>
      <div class="flex gap-4">
        <div class="flex-1">
          <input
            v-model="searchInput"
            @keyup.enter="searchSummoner"
            type="text"
            placeholder="输入召唤师名字..."
            class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 bg-white dark:bg-gray-700 text-gray-900 dark:text-white"
          />
        </div>
        <button
          @click="searchSummoner"
          :disabled="!searchInput.trim() || loading"
          class="px-6 py-2 bg-green-500 hover:bg-green-600 text-white rounded-lg transition-colors disabled:opacity-50"
        >
          搜索
        </button>
        <button
          @click="showMyData"
          :disabled="loading"
          class="px-6 py-2 bg-purple-500 hover:bg-purple-600 text-white rounded-lg transition-colors disabled:opacity-50"
        >
          我的数据
        </button>
        <button
          @click="debugMatchHistory"
          :disabled="loading"
          class="px-4 py-2 bg-yellow-500 hover:bg-yellow-600 text-white rounded-lg transition-colors disabled:opacity-50"
        >
          调试
        </button>
      </div>

      <!-- 搜索历史 -->
      <div v-if="searchHistory.length > 0" class="mt-4">
        <h3 class="text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">最近搜索</h3>
        <div class="flex flex-wrap gap-2">
          <button
            v-for="(name, index) in searchHistory"
            :key="index"
            @click="quickSearch(name)"
            class="px-3 py-1 bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-300 rounded-full text-sm hover:bg-gray-200 dark:hover:bg-gray-600 transition-colors"
          >
            {{ name }}
          </button>
        </div>
      </div>
    </div>

    <!-- 错误信息 -->
    <div v-if="error" class="bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-lg p-4">
      <div class="flex">
        <svg class="h-5 w-5 text-red-400" viewBox="0 0 20 20" fill="currentColor">
          <path
            fill-rule="evenodd"
            d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z"
            clip-rule="evenodd"
          />
        </svg>
        <div class="ml-3">
          <p class="text-sm text-red-800 dark:text-red-200">{{ error }}</p>
        </div>
      </div>
    </div>

    <!-- 战绩统计 -->
    <div v-if="statistics" class="space-y-6">
      <!-- 总体数据 -->
      <div class="grid grid-cols-1 md:grid-cols-4 gap-4">
        <div class="bg-gradient-to-r from-blue-500 to-blue-600 rounded-lg p-6 text-white">
          <div class="flex items-center">
            <div class="flex-1">
              <p class="text-blue-100 text-sm font-medium">总对局</p>
              <p class="text-2xl font-bold">{{ statistics.total_games }}</p>
            </div>
            <div class="p-3 bg-blue-400 bg-opacity-30 rounded-full">
              <svg class="h-6 w-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  stroke-width="2"
                  d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z"
                />
              </svg>
            </div>
          </div>
        </div>

        <div class="bg-gradient-to-r from-green-500 to-green-600 rounded-lg p-6 text-white">
          <div class="flex items-center">
            <div class="flex-1">
              <p class="text-green-100 text-sm font-medium">胜场</p>
              <p class="text-2xl font-bold">{{ statistics.wins }}</p>
            </div>
            <div class="p-3 bg-green-400 bg-opacity-30 rounded-full">
              <svg class="h-6 w-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
              </svg>
            </div>
          </div>
        </div>

        <div class="bg-gradient-to-r from-red-500 to-red-600 rounded-lg p-6 text-white">
          <div class="flex items-center">
            <div class="flex-1">
              <p class="text-red-100 text-sm font-medium">负场</p>
              <p class="text-2xl font-bold">{{ statistics.losses }}</p>
            </div>
            <div class="p-3 bg-red-400 bg-opacity-30 rounded-full">
              <svg class="h-6 w-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
              </svg>
            </div>
          </div>
        </div>

        <div class="bg-gradient-to-r from-purple-500 to-purple-600 rounded-lg p-6 text-white">
          <div class="flex items-center">
            <div class="flex-1">
              <p class="text-purple-100 text-sm font-medium">胜率</p>
              <p class="text-2xl font-bold">{{ statistics.win_rate.toFixed(1) }}%</p>
            </div>
            <div class="p-3 bg-purple-400 bg-opacity-30 rounded-full">
              <svg class="h-6 w-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  stroke-width="2"
                  d="M13 7h8m0 0v8m0-8l-8 8-4-4-6 6"
                />
              </svg>
            </div>
          </div>
        </div>
      </div>

      <!-- KDA统计 -->
      <div class="bg-white dark:bg-gray-800 rounded-lg shadow p-6">
        <h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-4">KDA统计</h3>
        <div class="grid grid-cols-2 md:grid-cols-4 gap-4">
          <div class="text-center">
            <p class="text-2xl font-bold text-blue-600 dark:text-blue-400">
              {{ statistics.avg_kills.toFixed(1) }}
            </p>
            <p class="text-sm text-gray-600 dark:text-gray-400">平均击杀</p>
          </div>
          <div class="text-center">
            <p class="text-2xl font-bold text-red-600 dark:text-red-400">
              {{ statistics.avg_deaths.toFixed(1) }}
            </p>
            <p class="text-sm text-gray-600 dark:text-gray-400">平均死亡</p>
          </div>
          <div class="text-center">
            <p class="text-2xl font-bold text-green-600 dark:text-green-400">
              {{ statistics.avg_assists.toFixed(1) }}
            </p>
            <p class="text-sm text-gray-600 dark:text-gray-400">平均助攻</p>
          </div>
          <div class="text-center">
            <p class="text-2xl font-bold text-purple-600 dark:text-purple-400">
              {{ statistics.avg_kda.toFixed(2) }}
            </p>
            <p class="text-sm text-gray-600 dark:text-gray-400">平均KDA</p>
          </div>
        </div>
      </div>

      <!-- 常用英雄 -->
      <div class="bg-white dark:bg-gray-800 rounded-lg shadow p-6">
        <h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-4">常用英雄</h3>
        <div class="space-y-3">
          <div
            v-for="champion in statistics.favorite_champions"
            :key="champion.champion_id"
            class="flex items-center justify-between p-3 bg-gray-50 dark:bg-gray-700 rounded-lg"
          >
            <div class="flex items-center">
              <div
                class="w-10 h-10 bg-gradient-to-r from-blue-500 to-purple-600 rounded-full flex items-center justify-center text-white font-bold"
              >
                {{ getChampionName(champion.champion_id).charAt(0) }}
              </div>
              <div class="ml-3">
                <p class="font-medium text-gray-900 dark:text-white">
                  {{ getChampionName(champion.champion_id) }}
                </p>
                <p class="text-sm text-gray-600 dark:text-gray-400">{{ champion.games_played }} 场游戏</p>
              </div>
            </div>
            <div class="text-right">
              <p
                class="font-medium"
                :class="
                  champion.win_rate >= 50 ? 'text-green-600 dark:text-green-400' : 'text-red-600 dark:text-red-400'
                "
              >
                {{ champion.win_rate.toFixed(1) }}%
              </p>
              <p class="text-sm text-gray-600 dark:text-gray-400">
                {{ champion.wins }}胜 {{ champion.games_played - champion.wins }}负
              </p>
            </div>
          </div>
        </div>
      </div>

      <!-- 最近对局 -->
      <Card>
        <CardHeader>
          <CardTitle>最近对局</CardTitle>
          <CardDescription>点击查看详细信息</CardDescription>
        </CardHeader>
        <CardContent>
          <div class="space-y-3">
            <!-- 战绩卡片列表 -->
            <Card
              v-for="game in statistics.recent_performance"
              :key="game.game_creation"
              @click="openGameDetail(game)"
              class="cursor-pointer transition-all duration-200 hover:shadow-md hover:scale-[1.02]"
              :class="game.win ? 'border-green-200 hover:border-green-300' : 'border-red-200 hover:border-red-300'"
            >
              <CardContent class="p-4">
                <!-- 主要信息行 -->
                <div class="flex items-center justify-between mb-3">
                  <div class="flex items-center space-x-3">
                    <Avatar class="h-12 w-12">
                      <AvatarFallback class="bg-gradient-to-br from-blue-500 to-purple-600 text-white font-bold">
                        {{ getChampionName(game.champion_id).charAt(0) }}
                      </AvatarFallback>
                    </Avatar>
                    <div>
                      <h4 class="font-semibold">{{ getChampionName(game.champion_id) }}</h4>
                      <p class="text-sm text-muted-foreground">
                        {{ formatGameMode(game.game_mode) }}
                      </p>
                    </div>
                  </div>

                  <!-- 胜负标识 -->
                  <div class="flex items-center space-x-3">
                    <Badge :variant="game.win ? 'default' : 'destructive'" class="text-sm font-bold">
                      {{ game.win ? '胜利' : '失败' }}
                    </Badge>
                    <svg class="w-5 h-5 text-muted-foreground" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"></path>
                    </svg>
                  </div>
                </div>

                <!-- KDA和时间信息 -->
                <div class="flex items-center justify-between text-sm">
                  <div class="flex items-center space-x-4">
                    <!-- KDA -->
                    <div class="flex items-center space-x-1">
                      <span class="font-bold text-blue-600">{{ game.kills }}</span>
                      <span class="text-muted-foreground">/</span>
                      <span class="font-bold text-red-600">{{ game.deaths }}</span>
                      <span class="text-muted-foreground">/</span>
                      <span class="font-bold text-green-600">{{ game.assists }}</span>
                    </div>

                    <!-- KDA比率 -->
                    <Badge variant="secondary" class="font-mono">
                      {{ calculateKDA(game.kills, game.deaths, game.assists) }}
                    </Badge>

                    <!-- 游戏时长 -->
                    <div class="text-muted-foreground flex items-center">
                      <svg class="w-4 h-4 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path
                          stroke-linecap="round"
                          stroke-linejoin="round"
                          stroke-width="2"
                          d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"
                        ></path>
                      </svg>
                      {{ formatDuration(game.game_duration) }}
                    </div>
                  </div>

                  <!-- 时间戳 -->
                  <div class="text-muted-foreground">
                    {{ formatGameTime(game.game_creation) }}
                  </div>
                </div>
              </CardContent>
            </Card>
          </div>

          <!-- 游戏详细信息Dialog -->
          <Dialog v-model:open="dialogOpen">
            <DialogContent class="max-w-7xl max-h-[90vh]">
              <DialogHeader>
                <DialogTitle class="flex items-center gap-3">
                  <div
                    class="h-8 w-8 rounded-full bg-gradient-to-br from-blue-500 to-purple-600 flex items-center justify-center text-white font-bold"
                  >
                    {{ selectedGame?.champion_id ? getChampionName(selectedGame.champion_id).charAt(0) : 'G' }}
                  </div>
                  游戏详细信息
                </DialogTitle>
                <DialogDescription v-if="selectedGame">
                  {{ getChampionName(selectedGame?.champion_id) }} - {{ formatGameMode(selectedGame.game_mode) }} -
                  {{ formatRelativeTime(selectedGame.game_creation) }}
                </DialogDescription>
              </DialogHeader>

              <ScrollArea class="max-h-[75vh]">
                <!-- 加载状态 -->
                <div v-if="gameDetailLoading" class="flex items-center justify-center py-16">
                  <div class="text-center">
                    <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-primary mx-auto mb-4"></div>
                    <p class="text-lg font-medium">正在加载游戏详细信息...</p>
                    <p class="text-sm text-muted-foreground">这可能需要几秒钟</p>
                  </div>
                </div>

                <!-- 详细信息内容 -->
                <div v-else-if="gameDetailData" class="space-y-6">
                  <!-- 游戏概览卡片 -->
                  <div class="grid grid-cols-1 lg:grid-cols-3 gap-4">
                    <!-- 基本信息 -->
                    <Card class="lg:col-span-2">
                      <CardHeader>
                        <CardTitle class="flex items-center gap-2">
                          <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path
                              stroke-linecap="round"
                              stroke-linejoin="round"
                              stroke-width="2"
                              d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
                            ></path>
                          </svg>
                          游戏信息
                        </CardTitle>
                      </CardHeader>
                      <CardContent>
                        <div class="grid grid-cols-2 md:grid-cols-3 gap-6">
                          <div class="space-y-1">
                            <p class="text-sm text-muted-foreground">游戏ID</p>
                            <p class="font-mono text-sm">{{ gameDetailData.gameId }}</p>
                          </div>
                          <div class="space-y-1">
                            <p class="text-sm text-muted-foreground">游戏时长</p>
                            <p class="font-semibold">
                              {{ formatDuration((gameDetailData.gameDuration as number) || 0) }}
                            </p>
                          </div>
                          <div class="space-y-1">
                            <p class="text-sm text-muted-foreground">地图</p>
                            <p class="font-semibold">
                              {{ getMapName(gameDetailData.mapId as number) }}
                            </p>
                          </div>
                          <div class="space-y-1">
                            <p class="text-sm text-muted-foreground">游戏模式</p>
                            <Badge variant="secondary">{{
                              formatGameMode((gameDetailData.gameMode as string) || '')
                            }}</Badge>
                          </div>
                          <div class="space-y-1">
                            <p class="text-sm text-muted-foreground">队列类型</p>
                            <Badge variant="outline">{{ getQueueName((gameDetailData.queueId as number) || 0) }}</Badge>
                          </div>
                          <div class="space-y-1">
                            <p class="text-sm text-muted-foreground">游戏版本</p>
                            <p class="text-sm font-mono">{{ gameDetailData.gameVersion }}</p>
                          </div>
                        </div>
                      </CardContent>
                    </Card>

                    <!-- 队伍概览 -->
                    <Card>
                      <CardHeader>
                        <CardTitle class="flex items-center gap-2">
                          <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path
                              stroke-linecap="round"
                              stroke-linejoin="round"
                              stroke-width="2"
                              d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z"
                            ></path>
                          </svg>
                          对战结果
                        </CardTitle>
                      </CardHeader>
                      <CardContent>
                        <div class="space-y-4">
                          <div class="flex items-center justify-between p-3 rounded-lg bg-blue-50 dark:bg-blue-950/20">
                            <div class="flex items-center gap-2">
                              <div class="w-4 h-4 bg-blue-500 rounded"></div>
                              <span class="font-medium">蓝色方</span>
                            </div>
                            <Badge
                              :variant="getTeamResult('100', gameDetailData.teams) === '胜利' ? 'default' : 'secondary'"
                            >
                              {{ getTeamResult('100', gameDetailData.teams) }}
                            </Badge>
                          </div>
                          <div class="flex items-center justify-between p-3 rounded-lg bg-red-50 dark:bg-red-950/20">
                            <div class="flex items-center gap-2">
                              <div class="w-4 h-4 bg-red-500 rounded"></div>
                              <span class="font-medium">红色方</span>
                            </div>
                            <Badge
                              :variant="getTeamResult('200', gameDetailData.teams) === '胜利' ? 'default' : 'secondary'"
                            >
                              {{ getTeamResult('200', gameDetailData.teams) }}
                            </Badge>
                          </div>
                        </div>
                      </CardContent>
                    </Card>
                  </div>

                  <!-- 队伍数据对比 -->
                  <Card>
                    <CardHeader>
                      <CardTitle class="flex items-center gap-2">
                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                          <path
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            stroke-width="2"
                            d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z"
                          ></path>
                        </svg>
                        队伍数据对比
                      </CardTitle>
                    </CardHeader>
                    <CardContent>
                      <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
                        <div class="text-center space-y-2">
                          <p class="text-sm text-muted-foreground">推塔数</p>
                          <div class="flex items-center justify-center gap-4">
                            <span class="text-lg font-bold text-blue-600">{{
                              getTeamStat('100', gameDetailData.teams, 'towerKills')
                            }}</span>
                            <span class="text-muted-foreground">VS</span>
                            <span class="text-lg font-bold text-red-600">{{
                              getTeamStat('200', gameDetailData.teams, 'towerKills')
                            }}</span>
                          </div>
                        </div>
                        <div class="text-center space-y-2">
                          <p class="text-sm text-muted-foreground">小龙击杀</p>
                          <div class="flex items-center justify-center gap-4">
                            <span class="text-lg font-bold text-blue-600">{{
                              getTeamStat('100', gameDetailData.teams, 'dragonKills')
                            }}</span>
                            <span class="text-muted-foreground">VS</span>
                            <span class="text-lg font-bold text-red-600">{{
                              getTeamStat('200', gameDetailData.teams, 'dragonKills')
                            }}</span>
                          </div>
                        </div>
                        <div class="text-center space-y-2">
                          <p class="text-sm text-muted-foreground">大龙击杀</p>
                          <div class="flex items-center justify-center gap-4">
                            <span class="text-lg font-bold text-blue-600">{{
                              getTeamStat('100', gameDetailData.teams, 'baronKills')
                            }}</span>
                            <span class="text-muted-foreground">VS</span>
                            <span class="text-lg font-bold text-red-600">{{
                              getTeamStat('200', gameDetailData.teams, 'baronKills')
                            }}</span>
                          </div>
                        </div>
                      </div>
                    </CardContent>
                  </Card>

                  <!-- 队伍详细信息 -->
                  <Tabs default-value="blue-team" class="w-full">
                    <TabsList class="grid w-full grid-cols-3">
                      <TabsTrigger value="blue-team" class="text-blue-600">
                        🔵 蓝色方 ({{ getTeamResult('100', gameDetailData.teams) }})
                      </TabsTrigger>
                      <TabsTrigger value="red-team" class="text-red-600">
                        🔴 红色方 ({{ getTeamResult('200', gameDetailData.teams) }})
                      </TabsTrigger>
                      <TabsTrigger value="bans"> 🚫 禁用英雄 </TabsTrigger>
                    </TabsList>

                    <!-- 蓝色方 -->
                    <TabsContent value="blue-team" class="mt-6">
                      <div class="space-y-4">
                        <div
                          v-for="participant in getTeamParticipants('100', gameDetailData)"
                          :key="participant.participantId"
                          class="p-4 rounded-lg border bg-gradient-to-r from-blue-50/50 to-transparent dark:from-blue-950/20"
                        >
                          <div class="grid grid-cols-1 lg:grid-cols-12 gap-4 items-center">
                            <!-- 玩家基本信息 -->
                            <div class="lg:col-span-3 flex items-center gap-3">
                              <Avatar class="h-12 w-12">
                                <AvatarFallback
                                  class="bg-gradient-to-br from-blue-500 to-purple-600 text-white font-bold"
                                >
                                  {{ getChampionName(participant.championId).charAt(0) }}
                                </AvatarFallback>
                              </Avatar>
                              <div>
                                <p class="font-semibold">
                                  {{ getChampionName(participant.championId) }}
                                </p>
                                <p class="text-sm text-muted-foreground">
                                  {{ getPlayerName(participant.participantId, gameDetailData) }}
                                </p>
                                <p class="text-xs text-muted-foreground">
                                  {{ getPlayerPosition(participant) }}
                                </p>
                              </div>
                            </div>

                            <!-- KDA和等级 -->
                            <div class="lg:col-span-2 text-center">
                              <p class="font-mono text-lg font-bold">
                                <span class="text-green-600">{{ participant.stats?.kills || 0 }}</span>
                                <span class="text-muted-foreground">/</span>
                                <span class="text-red-600">{{ participant.stats?.deaths || 0 }}</span>
                                <span class="text-muted-foreground">/</span>
                                <span class="text-blue-600">{{ participant.stats?.assists || 0 }}</span>
                              </p>
                              <Badge variant="outline" class="text-xs mt-1">
                                {{
                                  calculateKDA(
                                    participant.stats?.kills || 0,
                                    participant.stats?.deaths || 0,
                                    participant.stats?.assists || 0
                                  )
                                }}
                                KDA
                              </Badge>
                              <p class="text-xs text-muted-foreground mt-1">
                                等级 {{ participant.stats?.champLevel || 1 }}
                              </p>
                            </div>

                            <!-- 伤害数据 -->
                            <div class="lg:col-span-2 text-center">
                              <p class="text-sm font-semibold">
                                {{ formatNumber(participant.stats?.totalDamageDealtToChampions || 0) }}
                              </p>
                              <p class="text-xs text-muted-foreground">对英雄伤害</p>
                              <p class="text-xs text-green-600 mt-1">视野: {{ participant.stats?.visionScore || 0 }}</p>
                            </div>

                            <!-- 补刀和金币 -->
                            <div class="lg:col-span-2 text-center">
                              <p class="text-sm font-semibold">
                                {{
                                  (participant.stats?.totalMinionsKilled || 0) +
                                  (participant.stats?.neutralMinionsKilled || 0)
                                }}
                              </p>
                              <p class="text-xs text-muted-foreground">补刀</p>
                              <p class="text-xs text-yellow-600 mt-1">
                                {{ formatNumber(participant.stats?.goldEarned || 0) }}金币
                              </p>
                            </div>

                            <!-- 装备 -->
                            <div class="lg:col-span-3">
                              <div class="grid grid-cols-6 gap-1">
                                <div
                                  v-for="i in 6"
                                  :key="i"
                                  class="aspect-square bg-muted rounded border flex items-center justify-center text-xs"
                                >
                                  {{ getItemDisplay(participant.stats?.[`item${i - 1}`]) }}
                                </div>
                              </div>
                              <div class="flex gap-1 mt-2 justify-center">
                                <Badge v-if="participant.stats?.doubleKills" variant="secondary" class="text-xs">
                                  双杀x{{ participant.stats.doubleKills }}
                                </Badge>
                                <Badge v-if="participant.stats?.tripleKills" variant="secondary" class="text-xs">
                                  三杀x{{ participant.stats.tripleKills }}
                                </Badge>
                                <Badge v-if="participant.stats?.quadraKills" variant="secondary" class="text-xs">
                                  四杀x{{ participant.stats.quadraKills }}
                                </Badge>
                                <Badge v-if="participant.stats?.pentaKills" variant="destructive" class="text-xs">
                                  五杀x{{ participant.stats.pentaKills }}
                                </Badge>
                              </div>
                            </div>
                          </div>
                        </div>
                      </div>
                    </TabsContent>

                    <!-- 红色方 -->
                    <TabsContent value="red-team" class="mt-6">
                      <div class="space-y-4">
                        <div
                          v-for="participant in getTeamParticipants('200', gameDetailData)"
                          :key="participant.participantId"
                          class="p-4 rounded-lg border bg-gradient-to-r from-red-50/50 to-transparent dark:from-red-950/20"
                        >
                          <div class="grid grid-cols-1 lg:grid-cols-12 gap-4 items-center">
                            <!-- 玩家基本信息 -->
                            <div class="lg:col-span-3 flex items-center gap-3">
                              <Avatar class="h-12 w-12">
                                <AvatarFallback class="bg-gradient-to-br from-red-500 to-pink-600 text-white font-bold">
                                  {{ getChampionName(participant.championId).charAt(0) }}
                                </AvatarFallback>
                              </Avatar>
                              <div>
                                <p class="font-semibold">
                                  {{ getChampionName(participant.championId) }}
                                </p>
                                <p class="text-sm text-muted-foreground">
                                  {{ getPlayerName(participant.participantId, gameDetailData) }}
                                </p>
                                <p class="text-xs text-muted-foreground">
                                  {{ getPlayerPosition(participant) }}
                                </p>
                              </div>
                            </div>

                            <!-- KDA和等级 -->
                            <div class="lg:col-span-2 text-center">
                              <p class="font-mono text-lg font-bold">
                                <span class="text-green-600">{{ participant.stats?.kills || 0 }}</span>
                                <span class="text-muted-foreground">/</span>
                                <span class="text-red-600">{{ participant.stats?.deaths || 0 }}</span>
                                <span class="text-muted-foreground">/</span>
                                <span class="text-blue-600">{{ participant.stats?.assists || 0 }}</span>
                              </p>
                              <Badge variant="outline" class="text-xs mt-1">
                                {{
                                  calculateKDA(
                                    participant.stats?.kills || 0,
                                    participant.stats?.deaths || 0,
                                    participant.stats?.assists || 0
                                  )
                                }}
                                KDA
                              </Badge>
                              <p class="text-xs text-muted-foreground mt-1">
                                等级 {{ participant.stats?.champLevel || 1 }}
                              </p>
                            </div>

                            <!-- 伤害数据 -->
                            <div class="lg:col-span-2 text-center">
                              <p class="text-sm font-semibold">
                                {{ formatNumber(participant.stats?.totalDamageDealtToChampions || 0) }}
                              </p>
                              <p class="text-xs text-muted-foreground">对英雄伤害</p>
                              <p class="text-xs text-green-600 mt-1">视野: {{ participant.stats?.visionScore || 0 }}</p>
                            </div>

                            <!-- 补刀和金币 -->
                            <div class="lg:col-span-2 text-center">
                              <p class="text-sm font-semibold">
                                {{
                                  (participant.stats?.totalMinionsKilled || 0) +
                                  (participant.stats?.neutralMinionsKilled || 0)
                                }}
                              </p>
                              <p class="text-xs text-muted-foreground">补刀</p>
                              <p class="text-xs text-yellow-600 mt-1">
                                {{ formatNumber(participant.stats?.goldEarned || 0) }}金币
                              </p>
                            </div>

                            <!-- 装备 -->
                            <div class="lg:col-span-3">
                              <div class="grid grid-cols-6 gap-1">
                                <div
                                  v-for="i in 6"
                                  :key="i"
                                  class="aspect-square bg-muted rounded border flex items-center justify-center text-xs"
                                >
                                  {{ getItemDisplay(participant.stats?.[`item${i - 1}`]) }}
                                </div>
                              </div>
                              <div class="flex gap-1 mt-2 justify-center">
                                <Badge v-if="participant.stats?.doubleKills" variant="secondary" class="text-xs">
                                  双杀x{{ participant.stats.doubleKills }}
                                </Badge>
                                <Badge v-if="participant.stats?.tripleKills" variant="secondary" class="text-xs">
                                  三杀x{{ participant.stats.tripleKills }}
                                </Badge>
                                <Badge v-if="participant.stats?.quadraKills" variant="secondary" class="text-xs">
                                  四杀x{{ participant.stats.quadraKills }}
                                </Badge>
                                <Badge v-if="participant.stats?.pentaKills" variant="destructive" class="text-xs">
                                  五杀x{{ participant.stats.pentaKills }}
                                </Badge>
                              </div>
                            </div>
                          </div>
                        </div>
                      </div>
                    </TabsContent>

                    <!-- 禁用英雄 -->
                    <TabsContent value="bans" class="mt-6">
                      <Card>
                        <CardHeader>
                          <CardTitle>禁用英雄</CardTitle>
                        </CardHeader>
                        <CardContent>
                          <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                            <!-- 蓝色方禁用 -->
                            <div>
                              <h4 class="font-semibold text-blue-600 mb-3 flex items-center gap-2">
                                <div class="w-4 h-4 bg-blue-500 rounded"></div>
                                蓝色方禁用
                              </h4>
                              <div class="grid grid-cols-5 gap-2">
                                <div
                                  v-for="ban in getTeamBans('100', gameDetailData.teams)"
                                  :key="ban.pickTurn"
                                  class="aspect-square bg-blue-50 dark:bg-blue-950/20 rounded border flex items-center justify-center"
                                >
                                  <span class="text-xs font-semibold">{{
                                    getChampionName(ban.championId).slice(-2)
                                  }}</span>
                                </div>
                              </div>
                            </div>

                            <!-- 红色方禁用 -->
                            <div>
                              <h4 class="font-semibold text-red-600 mb-3 flex items-center gap-2">
                                <div class="w-4 h-4 bg-red-500 rounded"></div>
                                红色方禁用
                              </h4>
                              <div class="grid grid-cols-5 gap-2">
                                <div
                                  v-for="ban in getTeamBans('200', gameDetailData.teams)"
                                  :key="ban.pickTurn"
                                  class="aspect-square bg-red-50 dark:bg-red-950/20 rounded border flex items-center justify-center"
                                >
                                  <span class="text-xs font-semibold">{{
                                    getChampionName(ban.championId).slice(-2)
                                  }}</span>
                                </div>
                              </div>
                            </div>
                          </div>
                        </CardContent>
                      </Card>
                    </TabsContent>
                  </Tabs>
                </div>
              </ScrollArea>
            </DialogContent>
          </Dialog>
        </CardContent>
      </Card>
    </div>

    <!-- 空状态 -->
    <div v-else-if="!loading && !error" class="text-center py-12">
      <svg class="mx-auto h-12 w-12 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path
          stroke-linecap="round"
          stroke-linejoin="round"
          stroke-width="2"
          d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z"
        />
      </svg>
      <h3 class="mt-2 text-sm font-medium text-gray-900 dark:text-white">暂无战绩数据</h3>
      <p class="mt-1 text-sm text-gray-500 dark:text-gray-400">请先连接到League of Legends客户端</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ScrollArea } from '@/components/ui/scroll-area'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import { getChampionName } from '@/lib'
import { invoke } from '@tauri-apps/api/core'
interface MatchStatistics {
  total_games: number
  wins: number
  losses: number
  win_rate: number
  avg_kills: number
  avg_deaths: number
  avg_assists: number
  avg_kda: number
  favorite_champions: ChampionStats[]
  recent_performance: RecentGame[]
}

interface ChampionStats {
  champion_id: string
  games_played: number
  wins: number
  win_rate: number
}

interface RecentGame {
  game_id: number
  champion_id: string
  game_mode: string
  win: boolean
  kills: number
  deaths: number
  assists: number
  game_duration: number
  game_creation: number
}

const statistics = ref<MatchStatistics | null>(null)
const loading = ref(false)
const error = ref('')
const searchInput = ref('')
const searchedSummoner = ref('')
const searchHistory = ref<string[]>([])

onMounted(() => {
  console.log(statistics.value)
  loadSearchHistory()
  loadMyMatchHistory()
})

const loadSearchHistory = () => {
  const saved = localStorage.getItem('lcu_search_history')
  if (saved) {
    searchHistory.value = JSON.parse(saved)
  }
}

const saveSearchHistory = (name: string) => {
  if (!searchHistory.value.includes(name)) {
    searchHistory.value.unshift(name)
    searchHistory.value = searchHistory.value.slice(0, 5) // 只保留最近5个
    localStorage.setItem('lcu_search_history', JSON.stringify(searchHistory.value))
  }
}

const loadMyMatchHistory = async () => {
  loading.value = true
  error.value = ''
  searchedSummoner.value = ''

  try {
    const result = await invoke<MatchStatistics>('get_match_history')
    statistics.value = result
  } catch (err) {
    error.value = err as string
    statistics.value = null
  } finally {
    loading.value = false
  }
}

const searchSummoner = async () => {
  if (!searchInput.value.trim()) return

  loading.value = true
  error.value = ''

  try {
    const result = await invoke<MatchStatistics>('get_match_history_by_name', {
      summonerName: searchInput.value.trim()
    })
    statistics.value = result
    searchedSummoner.value = searchInput.value.trim()
    saveSearchHistory(searchInput.value.trim())
  } catch (err) {
    error.value = `查询失败: ${err}`
    statistics.value = null
  } finally {
    loading.value = false
  }
}

const quickSearch = (name: string) => {
  searchInput.value = name
  searchSummoner()
}

const showMyData = () => {
  searchInput.value = ''
  loadMyMatchHistory()
}

const refreshData = () => {
  if (searchedSummoner.value) {
    searchSummoner()
  } else {
    loadMyMatchHistory()
  }
}

const debugMatchHistory = async () => {
  loading.value = true
  error.value = ''

  try {
    const result = await invoke('debug_match_history')
    console.log('Debug Match History:', result)
    alert('调试信息已输出到控制台，请打开开发者工具查看')
  } catch (err) {
    console.error('Debug failed:', err)
    error.value = `调试失败: ${err}`
  } finally {
    loading.value = false
  }
}

// 游戏详细信息相关状态
const gameDetailData = ref<Record<string, unknown> | null>(null)
const gameDetailLoading = ref(false)
const dialogOpen = ref(false)
const selectedGame = ref<RecentGame | null>(null)

// 打开游戏详细信息
const openGameDetail = async (game: RecentGame) => {
  selectedGame.value = game
  dialogOpen.value = true
  gameDetailLoading.value = true
  gameDetailData.value = null

  try {
    const result = await invoke('get_game_detail', {
      gameId: game.game_id
    })
    gameDetailData.value = result as GameDetailData
    console.log('游戏详细信息:', result)
  } catch (err) {
    console.error('获取游戏详细信息失败:', err)
    error.value = `获取游戏详细信息失败: ${err}`
  } finally {
    gameDetailLoading.value = false
  }
}

// 格式化游戏模式
const formatGameMode = (mode: string) => {
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

// 计算KDA比率
const calculateKDA = (kills: number, deaths: number, assists: number) => {
  if (deaths === 0) {
    return (kills + assists).toFixed(1)
  }
  return ((kills + assists) / deaths).toFixed(1)
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
const getTeamResult = (teamId: string, teams: unknown) => {
  if (!teams || !Array.isArray(teams)) return '未知'
  const team = teams.find((t: Record<string, unknown>) => t.teamId?.toString() === teamId)
  return team?.win === 'Win' ? '胜利' : '失败'
}

// 获取队伍参与者
const getTeamParticipants = (teamId: string, gameDetail: Record<string, unknown>) => {
  const participants = gameDetail?.participants
  if (!participants || !Array.isArray(participants)) return []
  return participants.filter((p: Record<string, unknown>) => p.teamId?.toString() === teamId)
}

// 获取玩家名称
const getPlayerName = (participantId: number, gameDetail: Record<string, unknown>) => {
  const identities = gameDetail?.participantIdentities
  if (!identities || !Array.isArray(identities)) return '未知玩家'
  const identity = identities.find((id: Record<string, unknown>) => id.participantId === participantId)
  const player = identity?.player as Record<string, unknown>
  return player?.summonerName?.toString() || '未知玩家'
}

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

const formatGameTime = formatRelativeTime

// 获取地图名称
const getMapName = (mapId: number) => {
  const mapNames: Record<number, string> = {
    11: '召唤师峡谷',
    12: '嚎哭深渊',
    21: '纽克萨斯对战',
    22: '训练模式'
  }
  return mapNames[mapId] || `地图${mapId}`
}

// 获取队伍统计数据
const getTeamStat = (teamId: string, teams: unknown, statName: string) => {
  if (!teams || !Array.isArray(teams)) return 0
  const team = teams.find((t: Record<string, unknown>) => t.teamId?.toString() === teamId)
  return team?.[statName] || 0
}

// 获取队伍禁用英雄
const getTeamBans = (teamId: string, teams: unknown) => {
  if (!teams || !Array.isArray(teams)) return []
  const team = teams.find((t: Record<string, unknown>) => t.teamId?.toString() === teamId)
  return team?.bans || []
}

// 格式化数字
const formatNumber = (num: number) => {
  if (num >= 1000) {
    return `${(num / 1000).toFixed(1)}k`
  }
  return num.toString()
}

// 获取装备显示
const getItemDisplay = (itemId: number) => {
  if (!itemId || itemId === 0) return ''
  return itemId.toString().slice(-2) // 显示后两位数字
}

// 获取玩家位置
const getPlayerPosition = (participant: Record<string, unknown>) => {
  const timeline = participant.timeline as Record<string, unknown>
  const lane = timeline?.lane as string
  const role = timeline?.role as string

  const positionMap: Record<string, string> = {
    TOP: '上单',
    JUNGLE: '打野',
    MIDDLE: '中单',
    BOTTOM: role === 'DUO_CARRY' ? 'ADC' : '辅助',
    MID: '中单'
  }

  return positionMap[lane] || '位置未知'
}
</script>
