<template>
  <div class="space-y-6">
    <div>
      <h1 class="text-3xl font-bold text-foreground">安全设置</h1>
      <p class="text-muted-foreground">配置使用安全和风险控制选项</p>
    </div>

    <!-- 安全状态概览 -->
    <Card class="p-6">
      <div class="space-y-4">
        <div class="flex items-center space-x-2">
          <Shield class="h-5 w-5 text-green-500" />
          <h3 class="text-lg font-semibold">安全状态概览</h3>
        </div>

        <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
          <!-- 连接安全 -->
          <div class="space-y-3">
            <div class="flex items-center space-x-2">
              <div class="h-2 w-2 rounded-full bg-green-500"></div>
              <p class="text-sm font-medium">连接安全</p>
            </div>
            <p class="text-lg font-bold text-green-500">使用官方API</p>
          </div>

          <!-- 数据保护 -->
          <div class="space-y-3">
            <div class="flex items-center space-x-2">
              <div class="h-2 w-2 rounded-full bg-green-500"></div>
              <p class="text-sm font-medium">数据保护</p>
            </div>
            <p class="text-lg font-bold text-green-500">本地存储</p>
          </div>

          <!-- 风险等级 -->
          <div class="space-y-3">
            <div class="flex items-center space-x-2">
              <div class="h-2 w-2 rounded-full bg-yellow-500"></div>
              <p class="text-sm font-medium">风险等级</p>
            </div>
            <p class="text-lg font-bold text-yellow-500">低风险</p>
          </div>
        </div>
      </div>
    </Card>

    <!-- 使用限制 -->
    <Card class="p-6">
      <div class="space-y-6">
        <div>
          <h3 class="text-lg font-semibold">使用限制</h3>
          <p class="text-sm text-muted-foreground">设置自动功能的使用限制和风险控制</p>
        </div>

        <div class="space-y-6">
          <!-- 启用使用限制 -->
          <div class="flex items-center justify-between p-4 rounded-lg border">
            <div>
              <p class="font-medium">启用使用限制</p>
              <p class="text-sm text-muted-foreground">限制自动功能的使用频率</p>
            </div>
            <Switch v-model:checked="enableUsageLimit" />
          </div>

          <!-- 随机延迟 -->
          <div class="flex items-center justify-between p-4 rounded-lg border">
            <div>
              <p class="font-medium">随机延迟</p>
              <p class="text-sm text-muted-foreground">添加随机延迟以提高安全性</p>
            </div>
            <Switch v-model:checked="randomDelay" />
          </div>

          <div v-if="enableUsageLimit" class="space-y-4 pl-4 border-l-2 border-muted">
            <!-- 每小时操作上限 -->
            <div class="space-y-2">
              <div class="flex items-center justify-between">
                <label class="text-sm font-medium">每小时操作上限</label>
                <span class="text-sm text-muted-foreground">{{ hourlyLimit[0] }} 次</span>
              </div>
              <SimpleSlider
                v-model:model-value="hourlyLimit"
                :min="10"
                :max="100"
                :step="5"
                :default-value="[30]"
                class="w-full"
              />
            </div>

            <!-- 操作间隔 -->
            <div class="space-y-2">
              <div class="flex items-center justify-between">
                <label class="text-sm font-medium">操作间隔</label>
                <span class="text-sm text-muted-foreground">{{ operationInterval[0] }} 秒</span>
              </div>
              <SimpleSlider
                v-model:model-value="operationInterval"
                :min="1"
                :max="60"
                :step="1"
                :default-value="[5]"
                class="w-full"
              />
            </div>

            <!-- 延迟时间 -->
            <div class="space-y-2">
              <div class="flex items-center justify-between">
                <label class="text-sm font-medium">延迟时间</label>
                <div class="flex items-center space-x-2">
                  <input
                    v-model="delayMin"
                    type="number"
                    min="1"
                    max="30"
                    class="w-16 px-2 py-1 text-xs border border-input bg-background rounded focus:outline-none focus:ring-2 focus:ring-primary focus:ring-offset-2"
                  />
                  <span class="text-xs text-muted-foreground">-</span>
                  <input
                    v-model="delayMax"
                    type="number"
                    min="1"
                    max="30"
                    class="w-16 px-2 py-1 text-xs border border-input bg-background rounded focus:outline-none focus:ring-2 focus:ring-primary focus:ring-offset-2"
                  />
                  <span class="text-xs text-muted-foreground">秒</span>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </Card>
  </div>
</template>

<script setup lang="ts">
import { Shield } from 'lucide-vue-next'

// 安全设置状态
const enableUsageLimit = ref(true)
const randomDelay = ref(true)
const hourlyLimit = ref([30])
const operationInterval = ref([5])
const delayMin = ref(1)
const delayMax = ref(5)
</script>
