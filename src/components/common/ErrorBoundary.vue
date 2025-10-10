<template>
  <div v-if="hasError" class="min-h-screen flex items-center justify-center bg-background">
    <div class="max-w-md w-full mx-auto p-6">
      <div class="text-center space-y-4">
        <!-- 错误图标 -->
        <div class="mx-auto w-16 h-16 bg-destructive/10 rounded-full flex items-center justify-center">
          <svg class="w-8 h-8 text-destructive" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.964-.833-2.732 0L3.732 16.5c-.77.833.192 2.5 1.732 2.5z" />
          </svg>
        </div>

        <!-- 错误标题 -->
        <h1 class="text-2xl font-bold text-foreground">出现错误</h1>

        <!-- 错误信息 -->
        <div class="text-muted-foreground space-y-2">
          <p>{{ errorMessage }}</p>
          <p class="text-sm">错误代码: {{ errorCode }}</p>
        </div>

        <!-- 错误详情（可展开） -->
        <div class="space-y-2">
          <button
            @click="showDetails = !showDetails"
            class="text-sm text-primary hover:underline"
          >
            {{ showDetails ? '隐藏详情' : '显示详情' }}
          </button>

          <div v-if="showDetails" class="text-left">
            <details class="bg-muted/50 rounded-lg p-4">
              <summary class="cursor-pointer font-medium mb-2">错误堆栈</summary>
              <pre class="text-xs text-muted-foreground overflow-auto max-h-32">{{ errorStack }}</pre>
            </details>
          </div>
        </div>

        <!-- 操作按钮 -->
        <div class="flex gap-3 justify-center">
          <Button @click="retry" :disabled="isRetrying">
            <svg v-if="isRetrying" class="w-4 h-4 mr-2 animate-spin" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
            </svg>
            {{ isRetrying ? '重试中...' : '重试' }}
          </Button>

          <Button variant="outline" @click="reportError">
            报告问题
          </Button>

          <Button variant="ghost" @click="reset">
            重置应用
          </Button>
        </div>

        <!-- 提示信息 -->
        <div class="text-xs text-muted-foreground space-y-1">
          <p>如果问题持续存在，请尝试：</p>
          <ul class="list-disc list-inside space-y-1">
            <li>重启英雄联盟客户端</li>
            <li>检查网络连接</li>
            <li>更新应用版本</li>
          </ul>
        </div>
      </div>
    </div>
  </div>

  <!-- 正常内容 -->
  <slot v-else />
</template>

<script setup lang="ts">
import { ref, computed, onErrorCaptured, provide } from 'vue'
import { Button } from '@/components/ui/button'
import { useErrorHandler } from '@/composables/utils/useErrorHandler'

interface Props {
  fallback?: string
  onError?: (error: Error, errorInfo: any) => void
}

const props = withDefaults(defineProps<Props>(), {
  fallback: '应用出现错误，请重试'
})

const emit = defineEmits<{
  'error': [error: Error, errorInfo: any]
  'retry': []
  'reset': []
}>()

// 错误状态
const hasError = ref(false)
const errorMessage = ref('')
const errorCode = ref('')
const errorStack = ref('')
const showDetails = ref(false)
const isRetrying = ref(false)

// 错误处理
const errorHandler = useErrorHandler()

// 计算属性
const errorInfo = computed(() => ({
  message: errorMessage.value,
  code: errorCode.value,
  stack: errorStack.value,
  timestamp: new Date().toISOString()
}))

// 错误捕获
onErrorCaptured((error: Error, instance, info: string) => {
  console.error('[ErrorBoundary] 捕获到错误:', error)

  hasError.value = true
  errorMessage.value = error.message || props.fallback
  errorCode.value = error.name || 'UNKNOWN_ERROR'
  errorStack.value = error.stack || ''

  // 记录错误
  errorHandler.recordError(error, 'ErrorBoundary', true)

  // 发送错误事件
  emit('error', error, { instance, info })

  // 调用外部错误处理
  if (props.onError) {
    props.onError(error, { instance, info })
  }

  return false // 阻止错误继续传播
})

// 重试
const retry = async () => {
  isRetrying.value = true

  try {
    // 等待一段时间后重试
    await new Promise(resolve => setTimeout(resolve, 1000))

    // 清除错误状态
    hasError.value = false
    errorMessage.value = ''
    errorCode.value = ''
    errorStack.value = ''
    showDetails.value = false

    // 发送重试事件
    emit('retry')

    console.log('[ErrorBoundary] 重试成功')
  } catch (error) {
    console.error('[ErrorBoundary] 重试失败:', error)
  } finally {
    isRetrying.value = false
  }
}

// 报告错误
const reportError = () => {
  const reportData = {
    error: errorInfo.value,
    userAgent: navigator.userAgent,
    url: window.location.href,
    timestamp: new Date().toISOString()
  }

  console.log('[ErrorBoundary] 错误报告:', reportData)

  // 这里可以发送到错误收集服务
  // 例如 Sentry、LogRocket 等

  // 临时显示复制链接
  if (navigator.clipboard) {
    navigator.clipboard.writeText(JSON.stringify(reportData, null, 2))
    console.log('[ErrorBoundary] 错误信息已复制到剪贴板')
  }
}

// 重置应用
const reset = () => {
  // 清除所有状态
  hasError.value = false
  errorMessage.value = ''
  errorCode.value = ''
  errorStack.value = ''
  showDetails.value = false

  // 清除错误记录
  errorHandler.clearErrors()

  // 发送重置事件
  emit('reset')

  // 重新加载页面
  window.location.reload()
}

// 提供错误处理函数给子组件
provide('errorHandler', {
  handleError: (error: Error, context?: string) => {
    errorHandler.handleError(error, context)
  },
  clearErrors: () => {
    errorHandler.clearErrors()
  }
})
</script>

