<template>
  <div class="space-y-6">
    <div class="flex items-center justify-between">
      <div>
        <h1 class="text-3xl font-bold text-foreground">游戏助手</h1>
        <p class="text-muted-foreground">自动化游戏操作和实时数据分析</p>
      </div>
      <div class="flex items-center space-x-2">
        <Button variant="outline" size="sm">
          <RefreshCw class="h-4 w-4 mr-2" />
          刷新数据
        </Button>
        <Button size="sm">
          <Power class="h-4 w-4 mr-2" />
          全部启用
        </Button>
      </div>
    </div>

    <!-- 游戏状态 -->
    <Card class="p-6">
      <div class="space-y-4">
        <div class="flex items-center space-x-2">
          <Zap class="h-5 w-5 text-blue-500" />
          <h3 class="text-lg font-semibold">游戏状态</h3>
        </div>

        <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
          <!-- 客户端连接 -->
          <div class="space-y-3">
            <div class="flex items-center space-x-2">
              <div class="h-2 w-2 rounded-full bg-green-500"></div>
              <p class="text-sm font-medium">客户端连接</p>
            </div>
            <p class="text-lg font-bold text-green-500">已连接</p>
          </div>

          <!-- 游戏状态 -->
          <div class="space-y-3">
            <div class="flex items-center space-x-2">
              <div class="h-2 w-2 rounded-full bg-gray-500"></div>
              <p class="text-sm font-medium">游戏状态</p>
            </div>
            <p class="text-lg font-bold text-muted-foreground">大厅</p>
          </div>

          <!-- 召唤师 -->
          <div class="space-y-3">
            <div class="flex items-center space-x-2">
              <div class="h-2 w-2 rounded-full bg-green-500"></div>
              <p class="text-sm font-medium">召唤师</p>
            </div>
            <p class="text-lg font-bold text-green-500">TestSummoner</p>
          </div>
        </div>
      </div>
    </Card>

    <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
      <!-- 基础功能 -->
      <Card class="p-6">
        <div class="space-y-4">
          <div>
            <h3 class="text-lg font-semibold">基础功能</h3>
            <p class="text-sm text-muted-foreground">基本的自动化操作设置</p>
          </div>

          <div class="space-y-4">
            <!-- 自动接受对局 -->
            <div class="flex items-center justify-between p-4 rounded-lg border">
              <div>
                <p class="font-medium">自动接受对局</p>
                <p class="text-sm text-muted-foreground">自动接受找到的对局</p>
                <div class="flex items-center space-x-2 mt-2">
                  <label class="text-xs text-muted-foreground">延迟时间:</label>
                  <input
                    v-model="acceptDelay"
                    type="number"
                    min="1"
                    max="30"
                    class="w-16 px-2 py-1 text-xs border border-input bg-background rounded focus:outline-none focus:ring-2 focus:ring-primary focus:ring-offset-2"
                  />
                  <span class="text-xs text-muted-foreground">秒</span>
                </div>
              </div>
              <Switch v-model:checked="autoAcceptMatch" />
            </div>

            <!-- 自动选择英雄 -->
            <div class="flex items-center justify-between p-4 rounded-lg border">
              <div>
                <p class="font-medium">自动选择英雄</p>
                <p class="text-sm text-muted-foreground">预设英雄自动选择</p>
              </div>
              <Switch v-model:checked="autoSelectChampion" />
            </div>

            <!-- 自动禁用英雄 -->
            <div class="flex items-center justify-between p-4 rounded-lg border">
              <div>
                <p class="font-medium">自动禁用英雄</p>
                <p class="text-sm text-muted-foreground">自动禁用预设英雄</p>
                <div class="mt-2">
                  <select
                    class="w-full px-3 py-1 text-sm border border-input bg-background rounded focus:outline-none focus:ring-2 focus:ring-primary focus:ring-offset-2"
                  >
                    <option value="亚索">亚索</option>
                    <option value="劫">劫</option>
                    <option value="德莱文">德莱文</option>
                  </select>
                </div>
              </div>
              <Switch v-model:checked="autoBanChampion" />
            </div>
          </div>
        </div>
      </Card>

      <!-- 高级功能 -->
      <Card class="p-6">
        <div class="space-y-4">
          <div>
            <h3 class="text-lg font-semibold">高级功能</h3>
            <p class="text-sm text-muted-foreground">进阶的辅助功能</p>
          </div>

          <div class="space-y-4">
            <!-- 自动符文配置 -->
            <div class="flex items-center justify-between p-4 rounded-lg border">
              <div>
                <p class="font-medium">自动符文配置</p>
                <p class="text-sm text-muted-foreground">根据英雄自动配置最优符文</p>
                <div class="mt-2">
                  <select
                    class="w-full px-3 py-1 text-sm border border-input bg-background rounded focus:outline-none focus:ring-2 focus:ring-primary focus:ring-offset-2"
                  >
                    <option value="OPGG">OPGG</option>
                    <option value="UGG">U.GG</option>
                    <option value="LOLALYTICS">LOLALYTICS</option>
                  </select>
                </div>
              </div>
              <Switch v-model:checked="autoRuneConfig" />
            </div>

            <!-- 对局信息查询 -->
            <div class="flex items-center justify-between p-4 rounded-lg border">
              <div>
                <p class="font-medium">对局信息查询</p>
                <p class="text-sm text-muted-foreground">查询当前对局信息</p>
              </div>
              <Switch v-model:checked="matchInfoQuery" />
            </div>

            <!-- 战术建议 -->
            <div class="flex items-center justify-between p-4 rounded-lg border">
              <div>
                <p class="font-medium">战术建议</p>
                <p class="text-sm text-muted-foreground">基于数据的战术建议</p>
              </div>
              <Switch v-model:checked="tacticalAdvice" />
            </div>
          </div>
        </div>
      </Card>
    </div>

    <!-- 功能状态 -->
    <Card class="p-6">
      <div class="space-y-4">
        <div>
          <h3 class="text-lg font-semibold">功能状态</h3>
          <p class="text-sm text-muted-foreground">当前启用的自动化功能状态</p>
        </div>

        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
          <div class="p-4 rounded-lg border border-border/40 bg-background/30 backdrop-blur-sm">
            <div class="flex items-center space-x-2 mb-2">
              <div class="h-2 w-2 rounded-full bg-green-500 shadow-sm"></div>
              <p class="text-sm font-medium text-foreground">自动接受</p>
            </div>
            <p class="text-xs text-foreground/70">运行中</p>
          </div>

          <div class="p-4 rounded-lg border border-border/40 bg-background/30 backdrop-blur-sm">
            <div class="flex items-center space-x-2 mb-2">
              <div class="h-2 w-2 rounded-full bg-blue-500 shadow-sm"></div>
              <p class="text-sm font-medium text-foreground">自动选择</p>
            </div>
            <p class="text-xs text-foreground/70">已禁用</p>
          </div>

          <div class="p-4 rounded-lg border border-border/40 bg-background/30 backdrop-blur-sm">
            <div class="flex items-center space-x-2 mb-2">
              <div class="h-2 w-2 rounded-full bg-green-500 shadow-sm"></div>
              <p class="text-sm font-medium text-foreground">自动符文</p>
            </div>
            <p class="text-xs text-foreground/70">常规符文</p>
          </div>

          <div class="p-4 rounded-lg border border-border/40 bg-background/30 backdrop-blur-sm">
            <div class="flex items-center space-x-2 mb-2">
              <div class="h-2 w-2 rounded-full bg-green-500 shadow-sm"></div>
              <p class="text-sm font-medium text-foreground">对局分析</p>
            </div>
            <p class="text-xs text-foreground/70">运行中</p>
          </div>
        </div>
      </div>
    </Card>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { Card } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { Switch } from '@/components/ui/switch'
import { RefreshCw, Power, Zap } from 'lucide-vue-next'

// 状态管理
const autoAcceptMatch = ref(true)
const autoSelectChampion = ref(false)
const autoBanChampion = ref(false)
const autoRuneConfig = ref(true)
const matchInfoQuery = ref(true)
const tacticalAdvice = ref(false)
const acceptDelay = ref(1)
</script>
