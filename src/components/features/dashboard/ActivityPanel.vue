<template>
  <div class="p-6 space-y-6">
    <!-- 活动统计概览 -->
    <div class="grid grid-cols-4 gap-4">
      <div class="bg-card rounded-xl p-4 border">
        <div class="flex items-center justify-between">
          <div>
            <p class="text-sm text-muted-foreground">总活动</p>
            <p class="text-2xl font-bold">{{ activities.length }}</p>
          </div>
          <div class="h-8 w-8 bg-blue-100 rounded-lg flex items-center justify-center">
            <Activity class="h-4 w-4 text-blue-600" />
          </div>
        </div>
      </div>

      <div class="bg-card rounded-xl p-4 border">
        <div class="flex items-center justify-between">
          <div>
            <p class="text-sm text-muted-foreground">成功</p>
            <p class="text-2xl font-bold text-green-600">{{ successCount }}</p>
          </div>
          <div class="h-8 w-8 bg-green-100 rounded-lg flex items-center justify-center">
            <CheckCircle class="h-4 w-4 text-green-600" />
          </div>
        </div>
      </div>

      <div class="bg-card rounded-xl p-4 border">
        <div class="flex items-center justify-between">
          <div>
            <p class="text-sm text-muted-foreground">警告</p>
            <p class="text-2xl font-bold text-yellow-600">{{ warningCount }}</p>
          </div>
          <div class="h-8 w-8 bg-yellow-100 rounded-lg flex items-center justify-center">
            <AlertTriangle class="h-4 w-4 text-yellow-600" />
          </div>
        </div>
      </div>

      <div class="bg-card rounded-xl p-4 border">
        <div class="flex items-center justify-between">
          <div>
            <p class="text-sm text-muted-foreground">错误</p>
            <p class="text-2xl font-bold text-red-600">{{ errorCount }}</p>
          </div>
          <div class="h-8 w-8 bg-red-100 rounded-lg flex items-center justify-center">
            <XCircle class="h-4 w-4 text-red-600" />
          </div>
        </div>
      </div>
    </div>

    <!-- 筛选和操作栏 -->
    <div class="flex items-center justify-between">
      <div class="flex items-center space-x-4">
        <!-- 分类筛选 -->
        <div class="flex items-center space-x-2">
          <label class="text-sm font-medium">分类:</label>
          <Select v-model="selectedCategory" @update:model-value="handleCategoryChange">
            <SelectTrigger class="w-40">
              <SelectValue placeholder="全部分类" />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="all">全部分类</SelectItem>
              <SelectItem v-for="(label, category) in ActivityCategoryLabels" :key="category" :value="category">
                {{ label }}
              </SelectItem>
            </SelectContent>
          </Select>
        </div>

        <!-- 类型筛选 -->
        <div class="flex items-center space-x-2">
          <label class="text-sm font-medium">类型:</label>
          <Select v-model="selectedType" @update:model-value="handleTypeChange">
            <SelectTrigger class="w-32">
              <SelectValue placeholder="全部类型" />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="all">全部类型</SelectItem>
              <SelectItem value="success">成功</SelectItem>
              <SelectItem value="info">信息</SelectItem>
              <SelectItem value="warning">警告</SelectItem>
              <SelectItem value="error">错误</SelectItem>
            </SelectContent>
          </Select>
        </div>
      </div>

      <div class="flex items-center space-x-2">
        <Button variant="outline" size="sm" @click="clearFilters">
          <Filter class="h-4 w-4 mr-2" />
          清除筛选
        </Button>
        <Button variant="outline" size="sm" @click="clearActivities">
          <Trash2 class="h-4 w-4 mr-2" />
          清空记录
        </Button>
      </div>
    </div>

    <!-- 活动列表 -->
    <div class="bg-card rounded-xl border overflow-hidden">
      <div class="p-4 border-b bg-muted/30">
        <h3 class="font-medium">活动记录</h3>
        <p class="text-sm text-muted-foreground">
          {{ filteredActivities.length }} 条记录
          <template v-if="selectedCategory !== 'all' || selectedType !== 'all'"> (已筛选) </template>
        </p>
      </div>

      <ScrollArea class="h-[500px]">
        <div v-if="filteredActivities.length === 0" class="p-8 text-center text-muted-foreground">
          <Activity class="h-12 w-12 mx-auto mb-4 opacity-50" />
          <p>暂无活动记录</p>
        </div>

        <div v-else class="divide-y">
          <div
            v-for="activity in filteredActivities"
            :key="activity.id"
            class="p-4 hover:bg-muted/30 transition-colors"
          >
            <div class="flex items-start space-x-3">
              <!-- 活动类型图标 -->
              <div
                :class="[
                  'flex-shrink-0 w-8 h-8 rounded-lg flex items-center justify-center text-xs font-bold',
                  getTypeStyle(activity.type)
                ]"
              >
                {{ getTypeIcon(activity.type) }}
              </div>

              <div class="flex-1 min-w-0">
                <div class="flex items-center space-x-2 mb-1">
                  <!-- 分类标签 -->
                  <Badge variant="outline" class="text-xs" :class="getCategoryColor(activity.category)">
                    {{ ActivityCategoryLabels[activity.category] }}
                  </Badge>

                  <!-- 时间戳 -->
                  <span class="text-xs text-muted-foreground">
                    {{ formatRelativeTime(activity.timestamp) }}
                  </span>
                </div>

                <!-- 活动消息 -->
                <p class="text-sm">{{ activity.message }}</p>
              </div>
            </div>
          </div>
        </div>
      </ScrollArea>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue'
import { Activity, AlertTriangle, CheckCircle, Filter, Trash2, XCircle } from 'lucide-vue-next'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'
import { ScrollArea } from '@/components/ui/scroll-area'
import type { Activity as ActivityType, ActivityCategory, ActivityType as ActivityTypeEnum } from '@/types'
import { ActivityCategoryColors, ActivityCategoryLabels, ActivityTypeStyles } from '@/types'
import { useFormatters } from '@/composables'

const props = defineProps<{
  activities: ActivityType[]
}>()

const emit = defineEmits<{
  clearActivities: []
}>()

// 筛选状态
const selectedCategory = ref<ActivityCategory | 'all'>('all')
const selectedType = ref<ActivityTypeEnum | 'all'>('all')

// 统计计算
const successCount = computed(() => props.activities.filter((a) => a.type === 'success').length)
const warningCount = computed(() => props.activities.filter((a) => a.type === 'warning').length)
const errorCount = computed(() => props.activities.filter((a) => a.type === 'error').length)

// 筛选后的活动
const filteredActivities = computed(() => {
  let filtered = props.activities

  if (selectedCategory.value !== 'all') {
    filtered = filtered.filter((a) => a.category === selectedCategory.value)
  }

  if (selectedType.value !== 'all') {
    filtered = filtered.filter((a) => a.type === selectedType.value)
  }

  return filtered
})

// 获取类型样式
const getTypeStyle = (type: ActivityTypeEnum) => {
  const style = ActivityTypeStyles[type]
  return `${style.color} ${style.bgColor}`
}

// 获取类型图标
const getTypeIcon = (type: ActivityTypeEnum) => {
  return ActivityTypeStyles[type].icon
}

// 获取分类颜色
const getCategoryColor = (category: ActivityCategory) => {
  return ActivityCategoryColors[category]
}

// 处理分类变更
const handleCategoryChange = (category: ActivityCategory | 'all') => {
  selectedCategory.value = category
}

// 处理类型变更
const handleTypeChange = (type: ActivityTypeEnum | 'all') => {
  selectedType.value = type
}

// 清除筛选
const clearFilters = () => {
  selectedCategory.value = 'all'
  selectedType.value = 'all'
}

// 清空活动记录
const clearActivities = () => {
  emit('clearActivities')
}

const { formatRelativeTime } = useFormatters()
</script>
