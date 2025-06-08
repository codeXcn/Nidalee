<template>
  <div class="space-y-6">
    <div class="flex items-center justify-between">
      <div>
        <h1 class="text-3xl font-bold text-foreground">自动功能</h1>
        <p class="text-muted-foreground">配置和管理所有自动化功能</p>
      </div>
      <div class="flex items-center space-x-2">
        <Button variant="outline" size="sm">
          <Power class="h-4 w-4 mr-2" />
          全部禁用
        </Button>
        <Button size="sm">
          <PowerOff class="h-4 w-4 mr-2" />
          全部启用
        </Button>
        <div class="text-sm text-muted-foreground ml-4">总计启用: 4/8</div>
      </div>
    </div>

    <!-- 功能卡片网格 -->
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
      <!-- 自动接受对局 -->
      <Card class="p-6">
        <div class="space-y-4">
          <div class="flex items-center justify-between">
            <div class="flex items-center space-x-3">
              <div class="p-2 rounded-lg bg-blue-500/10">
                <Zap class="h-5 w-5 text-blue-500" />
              </div>
              <div>
                <h3 class="font-semibold">自动接受对局</h3>
                <p class="text-sm text-muted-foreground">自动接受匹配成功后的对局</p>
              </div>
            </div>
            <Switch v-model:checked="autoAcceptMatch" />
          </div>

          <div class="space-y-2">
            <div class="flex items-center justify-between text-sm">
              <label>延迟时间</label>
              <div class="flex items-center space-x-2">
                <SimpleSlider
                  v-model:model-value="acceptDelay"
                  :min="1"
                  :max="30"
                  :step="1"
                  :default-value="[1]"
                  class="w-16"
                />
                <span class="text-xs text-muted-foreground w-8">{{ acceptDelay[0] }}s</span>
              </div>
            </div>
          </div>
        </div>
      </Card>

      <!-- 自动选择英雄 -->
      <Card class="p-6">
        <div class="space-y-4">
          <div class="flex items-center justify-between">
            <div class="flex items-center space-x-3">
              <div class="p-2 rounded-lg bg-green-500/10">
                <User class="h-5 w-5 text-green-500" />
              </div>
              <div>
                <h3 class="font-semibold">自动选择英雄</h3>
                <p class="text-sm text-muted-foreground">预设英雄自动选择</p>
              </div>
            </div>
            <Switch v-model:checked="autoSelectChampion" />
          </div>
        </div>
      </Card>

      <!-- 自动禁用英雄 -->
      <Card class="p-6">
        <div class="space-y-4">
          <div class="flex items-center justify-between">
            <div class="flex items-center space-x-3">
              <div class="p-2 rounded-lg bg-red-500/10">
                <Shield class="h-5 w-5 text-red-500" />
              </div>
              <div>
                <h3 class="font-semibold">自动禁用英雄</h3>
                <p class="text-sm text-muted-foreground">自动禁用指定英雄</p>
              </div>
            </div>
            <Switch v-model:checked="autoBanChampion" />
          </div>

          <div v-if="autoBanChampion" class="space-y-2">
            <label class="text-sm">禁用英雄</label>
            <select
              class="w-full px-3 py-2 text-sm border border-input bg-background rounded focus:outline-none focus:ring-2 focus:ring-primary focus:ring-offset-2"
            >
              <option value="亚索">亚索</option>
              <option value="劫">劫</option>
              <option value="德莱文">德莱文</option>
            </select>
          </div>
        </div>
      </Card>

      <!-- 自动符文配置 -->
      <Card class="p-6">
        <div class="space-y-4">
          <div class="flex items-center justify-between">
            <div class="flex items-center space-x-3">
              <div class="p-2 rounded-lg bg-purple-500/10">
                <Bookmark class="h-5 w-5 text-purple-500" />
              </div>
              <div>
                <h3 class="font-semibold">自动符文配置</h3>
                <p class="text-sm text-muted-foreground">根据英雄自动配置最优符文</p>
              </div>
            </div>
            <Switch v-model:checked="autoRuneConfig" />
          </div>

          <div v-if="autoRuneConfig" class="space-y-2">
            <label class="text-sm">数据来源</label>
            <select
              v-model="runeSource"
              class="w-full px-3 py-2 text-sm border border-input bg-background rounded focus:outline-none focus:ring-2 focus:ring-primary focus:ring-offset-2"
            >
              <option value="OPGG">OPGG</option>
              <option value="UGG">U.GG</option>
              <option value="LOLALYTICS">LOLALYTICS</option>
            </select>
            <Button size="sm" class="w-full mt-2">
              <Edit class="h-4 w-4 mr-2" />
              立即更新符文
            </Button>
          </div>
        </div>
      </Card>

      <!-- 敌方信息查询 -->
      <Card class="p-6">
        <div class="space-y-4">
          <div class="flex items-center justify-between">
            <div class="flex items-center space-x-3">
              <div class="p-2 rounded-lg bg-orange-500/10">
                <Eye class="h-5 w-5 text-orange-500" />
              </div>
              <div>
                <h3 class="font-semibold">敌方信息查询</h3>
                <p class="text-sm text-muted-foreground">自动查询敌方英雄信息</p>
              </div>
            </div>
            <Switch v-model:checked="enemyInfoQuery" />
          </div>
        </div>
      </Card>

      <!-- 游戏助手 -->
      <Card class="p-6">
        <div class="space-y-4">
          <div class="flex items-center justify-between">
            <div class="flex items-center space-x-3">
              <div class="p-2 rounded-lg bg-cyan-500/10">
                <Target class="h-5 w-5 text-cyan-500" />
              </div>
              <div>
                <h3 class="font-semibold">游戏助手</h3>
                <p class="text-sm text-muted-foreground">检测中，对局未开始</p>
              </div>
            </div>
            <Switch v-model:checked="gameAssistant" />
          </div>

          <div v-if="gameAssistant" class="space-y-2">
            <label class="text-sm">等待刷新频</label>
          </div>
        </div>
      </Card>

      <!-- 自动聊天 -->
      <Card class="p-6">
        <div class="space-y-4">
          <div class="flex items-center justify-between">
            <div class="flex items-center space-x-3">
              <div class="p-2 rounded-lg bg-pink-500/10">
                <MessageSquare class="h-5 w-5 text-pink-500" />
              </div>
              <div>
                <h3 class="font-semibold">自动聊天</h3>
                <p class="text-sm text-muted-foreground">预设游戏内聊天发送</p>
              </div>
            </div>
            <Switch v-model:checked="autoChat" />
          </div>
        </div>
      </Card>

      <!-- 高级配置 -->
      <Card class="p-6">
        <div class="space-y-4">
          <div class="flex items-center justify-between">
            <div class="flex items-center space-x-3">
              <div class="p-2 rounded-lg bg-gray-500/10">
                <Settings class="h-5 w-5 text-gray-500" />
              </div>
              <div>
                <h3 class="font-semibold">高级配置</h3>
                <p class="text-sm text-muted-foreground">更多自定义设置</p>
              </div>
            </div>
            <Switch v-model:checked="advancedConfig" />
          </div>

          <div v-if="advancedConfig" class="text-center py-4">
            <p class="text-sm text-muted-foreground">运行中</p>
          </div>
        </div>
      </Card>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { Card } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { Switch } from '@/components/ui/switch'
import SimpleSlider from '@/components/ui/slider/SimpleSlider.vue'
import {
  Zap,
  Power,
  PowerOff,
  User,
  Shield,
  Bookmark,
  Eye,
  Target,
  MessageSquare,
  Settings,
  Edit
} from 'lucide-vue-next'

// 状态管理
const autoAcceptMatch = ref(true)
const autoSelectChampion = ref(false)
const autoBanChampion = ref(false)
const autoRuneConfig = ref(true)
const enemyInfoQuery = ref(true)
const gameAssistant = ref(false)
const autoChat = ref(false)
const advancedConfig = ref(false)

// 配置选项
const acceptDelay = ref([1])
const runeSource = ref('OPGG')
</script>
