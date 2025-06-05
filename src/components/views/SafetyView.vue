<template>
  <div class="safety-view space-y-6">
    <div>
      <h2 class="text-2xl font-bold">安全设置</h2>
      <p class="text-muted-foreground">配置使用安全和风险控制选项</p>
    </div>

    <!-- 安全状态概览 -->
    <Card>
      <CardHeader>
        <CardTitle class="flex items-center">
          <Shield class="w-5 h-5 mr-2" />
          安全状态概览
        </CardTitle>
      </CardHeader>
      <CardContent>
        <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
          <div class="flex items-center space-x-3">
            <div class="w-3 h-3 rounded-full bg-green-500" />
            <div>
              <p class="font-medium">连接安全</p>
              <p class="text-sm text-muted-foreground">使用官方API</p>
            </div>
          </div>
          <div class="flex items-center space-x-3">
            <div class="w-3 h-3 rounded-full bg-green-500" />
            <div>
              <p class="font-medium">数据保护</p>
              <p class="text-sm text-muted-foreground">本地存储</p>
            </div>
          </div>
          <div class="flex items-center space-x-3">
            <div class="w-3 h-3 rounded-full bg-yellow-500" />
            <div>
              <p class="font-medium">风险等级</p>
              <p class="text-sm text-muted-foreground">低风险</p>
            </div>
          </div>
        </div>
      </CardContent>
    </Card>

    <!-- 使用限制 -->
    <Card>
      <CardHeader>
        <CardTitle>使用限制</CardTitle>
        <CardDescription> 设置自动功能的使用限制以降低风险 </CardDescription>
      </CardHeader>
      <CardContent class="space-y-4">
        <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
          <div class="space-y-4">
            <div class="flex items-center justify-between">
              <div>
                <p class="font-medium">启用使用频率限制</p>
                <p class="text-sm text-muted-foreground">限制自动操作频率</p>
              </div>
              <Switch v-model:checked="safetySettings.enableRateLimit" />
            </div>

            <div
              v-if="safetySettings.enableRateLimit"
              class="ml-4 space-y-3 border-l-2 border-muted pl-4"
            >
              <div class="space-y-2">
                <label class="text-sm font-medium">每小时操作上限</label>
                <div class="flex items-center space-x-3">
                  <input
                    v-model.number="safetySettings.hourlyLimit"
                    type="range"
                    min="1"
                    max="100"
                    class="flex-1"
                  />
                  <span class="text-sm min-w-0">{{ safetySettings.hourlyLimit }}次</span>
                </div>
              </div>

              <div class="space-y-2">
                <label class="text-sm font-medium">操作间隔</label>
                <div class="flex items-center space-x-3">
                  <input
                    v-model.number="safetySettings.operationInterval"
                    type="range"
                    min="1"
                    max="30"
                    class="flex-1"
                  />
                  <span class="text-sm min-w-0">{{ safetySettings.operationInterval }}秒</span>
                </div>
              </div>
            </div>
          </div>

          <div class="space-y-4">
            <div class="flex items-center justify-between">
              <div>
                <p class="font-medium">随机延迟</p>
                <p class="text-sm text-muted-foreground">添加随机延迟提高安全性</p>
              </div>
              <Switch v-model:checked="safetySettings.randomDelay" />
            </div>

            <div
              v-if="safetySettings.randomDelay"
              class="ml-4 space-y-3 border-l-2 border-muted pl-4"
            >
              <div class="space-y-2">
                <label class="text-sm font-medium">延迟范围</label>
                <div class="flex items-center space-x-3">
                  <input
                    v-model.number="safetySettings.minDelay"
                    type="number"
                    min="0"
                    max="10"
                    class="w-20 px-2 py-1 text-sm border rounded"
                  />
                  <span class="text-sm">-</span>
                  <input
                    v-model.number="safetySettings.maxDelay"
                    type="number"
                    min="0"
                    max="30"
                    class="w-20 px-2 py-1 text-sm border rounded"
                  />
                  <span class="text-sm text-muted-foreground">秒</span>
                </div>
              </div>
            </div>
          </div>
        </div>
      </CardContent>
    </Card>

    <!-- 检测规避 -->
    <Card>
      <CardHeader>
        <CardTitle>检测规避</CardTitle>
        <CardDescription> 配置反检测措施 </CardDescription>
      </CardHeader>
      <CardContent class="space-y-4">
        <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
          <div class="space-y-4">
            <div class="flex items-center justify-between">
              <div>
                <p class="font-medium">模拟人工操作</p>
                <p class="text-sm text-muted-foreground">添加轻微的随机性</p>
              </div>
              <Switch v-model:checked="safetySettings.humanLike" />
            </div>

            <div class="flex items-center justify-between">
              <div>
                <p class="font-medium">错误率模拟</p>
                <p class="text-sm text-muted-foreground">偶尔故意失败操作</p>
              </div>
              <Switch v-model:checked="safetySettings.errorSimulation" />
            </div>

            <div class="flex items-center justify-between">
              <div>
                <p class="font-medium">响应时间变化</p>
                <p class="text-sm text-muted-foreground">模拟不同的反应速度</p>
              </div>
              <Switch v-model:checked="safetySettings.varyResponseTime" />
            </div>
          </div>

          <div class="space-y-4">
            <div class="flex items-center justify-between">
              <div>
                <p class="font-medium">仅在排位赛使用</p>
                <p class="text-sm text-muted-foreground">降低被发现的风险</p>
              </div>
              <Switch v-model:checked="safetySettings.rankedOnly" />
            </div>

            <div class="flex items-center justify-between">
              <div>
                <p class="font-medium">暂停检测</p>
                <p class="text-sm text-muted-foreground">检测到异常时自动暂停</p>
              </div>
              <Switch v-model:checked="safetySettings.autoPause" />
            </div>

            <div class="flex items-center justify-between">
              <div>
                <p class="font-medium">日志记录</p>
                <p class="text-sm text-muted-foreground">记录操作日志供分析</p>
              </div>
              <Switch v-model:checked="safetySettings.logging" />
            </div>
          </div>
        </div>
      </CardContent>
    </Card>

    <!-- 风险提醒 -->
    <Card>
      <CardHeader>
        <CardTitle class="flex items-center">
          <AlertTriangle class="w-5 h-5 mr-2 text-yellow-500" />
          重要提醒
        </CardTitle>
      </CardHeader>
      <CardContent class="space-y-4">
        <div
          class="bg-yellow-50 dark:bg-yellow-950 border border-yellow-200 dark:border-yellow-800 rounded-lg p-4"
        >
          <div class="flex items-start space-x-3">
            <AlertTriangle class="w-5 h-5 text-yellow-500 mt-0.5" />
            <div class="space-y-2">
              <h4 class="font-medium text-yellow-800 dark:text-yellow-200">使用须知</h4>
              <ul class="text-sm text-yellow-700 dark:text-yellow-300 space-y-1">
                <li>• 本工具仅使用官方API，不修改游戏文件</li>
                <li>• 请合理使用自动功能，避免过度依赖</li>
                <li>• 建议在自定义游戏中先测试功能</li>
                <li>• 如遇问题请及时停用相关功能</li>
              </ul>
            </div>
          </div>
        </div>

        <div
          class="bg-blue-50 dark:bg-blue-950 border border-blue-200 dark:border-blue-800 rounded-lg p-4"
        >
          <div class="flex items-start space-x-3">
            <Info class="w-5 h-5 text-blue-500 mt-0.5" />
            <div class="space-y-2">
              <h4 class="font-medium text-blue-800 dark:text-blue-200">技术说明</h4>
              <p class="text-sm text-blue-700 dark:text-blue-300">
                本工具通过League Client Protocol (LCP)
                与游戏客户端通信，所有操作都在官方支持的范围内进行。
              </p>
            </div>
          </div>
        </div>
      </CardContent>
    </Card>

    <!-- 操作日志 -->
    <Card>
      <CardHeader>
        <CardTitle>操作日志</CardTitle>
        <CardDescription> 最近的自动操作记录 </CardDescription>
      </CardHeader>
      <CardContent>
        <div class="space-y-3">
          <div
            v-for="log in operationLogs"
            :key="log.id"
            class="flex items-center justify-between p-3 border rounded-lg"
          >
            <div class="flex items-center space-x-3">
              <div
                :class="[
                  'w-2 h-2 rounded-full',
                  log.type === 'success'
                    ? 'bg-green-500'
                    : log.type === 'error'
                      ? 'bg-red-500'
                      : 'bg-yellow-500'
                ]"
              />
              <div>
                <p class="font-medium text-sm">{{ log.operation }}</p>
                <p class="text-xs text-muted-foreground">{{ log.details }}</p>
              </div>
            </div>
            <div class="text-right">
              <p class="text-xs text-muted-foreground">{{ log.timestamp }}</p>
            </div>
          </div>
        </div>

        <div class="flex justify-between items-center mt-4 pt-4 border-t">
          <Button variant="outline" size="sm" @click="clearLogs">
            <Trash2 class="w-4 h-4 mr-2" />
            清空日志
          </Button>
          <Button variant="outline" size="sm" @click="exportLogs">
            <Download class="w-4 h-4 mr-2" />
            导出日志
          </Button>
        </div>
      </CardContent>
    </Card>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Switch } from '@/components/ui/switch'
import { Shield, AlertTriangle, Info, Trash2, Download } from 'lucide-vue-next'

const safetySettings = ref({
  enableRateLimit: true,
  hourlyLimit: 30,
  operationInterval: 5,
  randomDelay: true,
  minDelay: 1,
  maxDelay: 5,
  humanLike: true,
  errorSimulation: false,
  varyResponseTime: true,
  rankedOnly: false,
  autoPause: true,
  logging: true
})

const operationLogs = ref([
  {
    id: 1,
    operation: '自动接受对局',
    details: '延迟 2.3秒 后接受',
    type: 'success',
    timestamp: '14:32:15'
  },
  {
    id: 2,
    operation: '自动选择英雄',
    details: '成功选择尼德丽',
    type: 'success',
    timestamp: '14:31:42'
  },
  {
    id: 3,
    operation: '自动符文配置',
    details: '从OP.GG获取符文数据',
    type: 'success',
    timestamp: '14:31:38'
  },
  {
    id: 4,
    operation: '连接检测',
    details: '检测到客户端重启',
    type: 'warning',
    timestamp: '14:25:10'
  },
  {
    id: 5,
    operation: '自动接受对局',
    details: '操作失败：对局已开始',
    type: 'error',
    timestamp: '14:20:33'
  }
])

function clearLogs() {
  operationLogs.value = []
}

function exportLogs() {
  // TODO: 实现导出日志功能
  console.log('导出日志')
}
</script>
