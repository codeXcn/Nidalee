<template>
  <div class="settings-panel space-y-6">
    <!-- 设置标签页 -->
    <div class="border-b border-border">
      <nav class="flex space-x-8">
        <button
          v-for="tab in tabs"
          :key="tab.id"
          :class="[
            'py-3 px-2 border-b-2 font-medium text-sm transition-all duration-200 relative',
            activeTab === tab.id
              ? 'border-primary text-primary shadow-primary/20 shadow-sm'
              : 'border-transparent text-muted-foreground hover:text-foreground hover:border-primary/30'
          ]"
          @click="activeTab = tab.id"
        >
          {{ tab.name }}
          <div
            v-if="activeTab === tab.id"
            class="absolute inset-x-0 -bottom-0.5 h-0.5 bg-primary rounded-full"
          />
        </button>
      </nav>
    </div>

    <!-- 通用设置 -->
    <div v-if="activeTab === 'general'" class="space-y-6">
      <div class="space-y-4">
        <h3 class="text-lg font-medium">通用设置</h3>

        <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
          <div class="space-y-4">
            <div class="space-y-2">
              <label class="text-sm font-medium">应用语言</label>
              <select
                v-model="configStore.config.general.language"
                class="w-full px-3 py-2 border rounded-md"
              >
                <option value="zh-CN">简体中文</option>
                <option value="en-US">English</option>
                <option value="ja-JP">日本語</option>
                <option value="ko-KR">한국어</option>
              </select>
            </div>

            <div class="flex items-center justify-between">
              <div>
                <p class="font-medium">开机自启动</p>
                <p class="text-sm text-muted-foreground">系统启动时自动运行</p>
              </div>
              <Switch v-model:checked="configStore.config.general.autoStart" />
            </div>

            <div class="flex items-center justify-between">
              <div>
                <p class="font-medium">最小化到托盘</p>
                <p class="text-sm text-muted-foreground">关闭窗口时最小化到系统托盘</p>
              </div>
              <Switch v-model:checked="configStore.config.general.minimizeToTray" />
            </div>
          </div>

          <div class="space-y-4">
            <div class="flex items-center justify-between">
              <div>
                <p class="font-medium">自动检查更新</p>
                <p class="text-sm text-muted-foreground">启动时检查新版本</p>
              </div>
              <Switch v-model:checked="configStore.config.general.autoUpdate" />
            </div>

            <div class="flex items-center justify-between">
              <div>
                <p class="font-medium">匿名数据收集</p>
                <p class="text-sm text-muted-foreground">帮助改进应用体验</p>
              </div>
              <Switch v-model:checked="configStore.config.general.analytics" />
            </div>

            <div class="flex items-center justify-between">
              <div>
                <p class="font-medium">硬件加速</p>
                <p class="text-sm text-muted-foreground">使用GPU加速渲染</p>
              </div>
              <Switch v-model:checked="configStore.config.general.hardwareAcceleration" />
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- 连接设置 -->
    <div v-if="activeTab === 'connection'" class="space-y-6">
      <div class="space-y-4">
        <h3 class="text-lg font-medium">连接设置</h3>

        <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
          <div class="space-y-4">
            <div class="flex items-center justify-between">
              <div>
                <p class="font-medium">自动重连</p>
                <p class="text-sm text-muted-foreground">连接断开时自动重连</p>
              </div>
              <Switch v-model:checked="configStore.config.connection.autoReconnect" />
            </div>

            <div class="space-y-2">
              <label class="text-sm font-medium">连接超时时间</label>
              <div class="flex items-center space-x-3">
                <input
                  v-model.number="configStore.config.connection.timeout"
                  type="range"
                  min="5"
                  max="60"
                  class="flex-1"
                />
                <span class="text-sm min-w-0">{{ configStore.config.connection.timeout }}秒</span>
              </div>
            </div>

            <div class="space-y-2">
              <label class="text-sm font-medium">重连间隔</label>
              <div class="flex items-center space-x-3">
                <input
                  v-model.number="configStore.config.connection.retryInterval"
                  type="range"
                  min="1"
                  max="30"
                  class="flex-1"
                />
                <span class="text-sm min-w-0"
                  >{{ configStore.config.connection.retryInterval }}秒</span
                >
              </div>
            </div>
          </div>

          <div class="space-y-4">
            <div class="space-y-2">
              <label class="text-sm font-medium">最大重连次数</label>
              <div class="flex items-center space-x-3">
                <input
                  v-model.number="configStore.config.connection.maxRetries"
                  type="range"
                  min="1"
                  max="10"
                  class="flex-1"
                />
                <span class="text-sm min-w-0"
                  >{{ configStore.config.connection.maxRetries }}次</span
                >
              </div>
            </div>

            <div class="flex items-center justify-between">
              <div>
                <p class="font-medium">连接状态通知</p>
                <p class="text-sm text-muted-foreground">显示连接状态变化</p>
              </div>
              <Switch v-model:checked="configStore.config.connection.notifications" />
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- 通知设置 -->
    <div v-if="activeTab === 'notifications'" class="space-y-6">
      <div class="space-y-4">
        <h3 class="text-lg font-medium">通知设置</h3>

        <div class="space-y-4">
          <div class="flex items-center justify-between">
            <div>
              <p class="font-medium">启用桌面通知</p>
              <p class="text-sm text-muted-foreground">显示系统桌面通知</p>
            </div>
            <Switch v-model:checked="configStore.config.notifications.desktop" />
          </div>

          <div class="flex items-center justify-between">
            <div>
              <p class="font-medium">声音提醒</p>
              <p class="text-sm text-muted-foreground">播放提示音</p>
            </div>
            <Switch v-model:checked="configStore.config.notifications.sound" />
          </div>

          <div class="flex items-center justify-between">
            <div>
              <p class="font-medium">对局开始通知</p>
              <p class="text-sm text-muted-foreground">进入游戏时提醒</p>
            </div>
            <Switch v-model:checked="configStore.config.notifications.gameStart" />
          </div>

          <div class="flex items-center justify-between">
            <div>
              <p class="font-medium">自动操作通知</p>
              <p class="text-sm text-muted-foreground">自动操作执行时通知</p>
            </div>
            <Switch v-model:checked="configStore.config.notifications.autoOperations" />
          </div>

          <div class="flex items-center justify-between">
            <div>
              <p class="font-medium">错误通知</p>
              <p class="text-sm text-muted-foreground">发生错误时通知</p>
            </div>
            <Switch v-model:checked="configStore.config.notifications.errors" />
          </div>
        </div>
      </div>
    </div>

    <!-- 缓存与存储设置 -->
    <div v-if="activeTab === 'cache'" class="space-y-6">
      <div class="space-y-4">
        <h3 class="text-lg font-medium">缓存与存储</h3>

        <!-- 缓存控制 -->
        <div class="space-y-4">
          <h4 class="font-medium">缓存控制</h4>

          <div class="space-y-3">
            <div class="flex items-center justify-between">
              <div>
                <p class="font-medium">启用配置缓存</p>
                <p class="text-sm text-muted-foreground">保存用户配置到本地存储</p>
              </div>
              <Switch v-model:checked="configStore.config.app.enableConfigCache" />
            </div>

            <div class="flex items-center justify-between">
              <div>
                <p class="font-medium">启用主题缓存</p>
                <p class="text-sm text-muted-foreground">保存主题设置到本地存储</p>
              </div>
              <Switch v-model:checked="configStore.config.app.enableThemeCache" />
            </div>

            <div class="flex items-center justify-between">
              <div>
                <p class="font-medium">启用游戏数据缓存</p>
                <p class="text-sm text-muted-foreground">保存游戏状态和统计数据</p>
              </div>
              <Switch v-model:checked="configStore.config.app.enableGameDataCache" />
            </div>
          </div>
        </div>

        <!-- 缓存信息 -->
        <div class="space-y-4">
          <h4 class="font-medium">缓存信息</h4>

          <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
            <div
              class="p-4 border rounded-lg transition-all duration-200 hover:shadow-md hover:border-primary/30 hover:scale-[1.02]"
            >
              <div class="flex items-center justify-between mb-2">
                <span class="font-medium">配置缓存</span>
                <div
                  :class="[
                    'size-3 rounded-full transition-all duration-200',
                    cacheStatus.configCacheExists
                      ? 'bg-green-500 shadow-lg shadow-green-500/30 animate-pulse'
                      : 'bg-gray-400 shadow-md shadow-gray-400/20'
                  ]"
                />
              </div>
              <p class="text-xs text-muted-foreground">{{ formatCacheSize('userConfig') }}</p>
              <Button
                variant="outline"
                size="sm"
                class="w-full mt-2"
                :disabled="!cacheStatus.configCacheExists"
                @click="clearSpecificCache('config')"
              >
                清除
              </Button>
            </div>

            <div
              class="p-4 border rounded-lg transition-all duration-200 hover:shadow-md hover:border-primary/30 hover:scale-[1.02]"
            >
              <div class="flex items-center justify-between mb-2">
                <span class="font-medium">主题缓存</span>
                <div
                  :class="[
                    'size-3 rounded-full transition-all duration-200',
                    cacheStatus.themeCacheExists
                      ? 'bg-green-500 shadow-green-500/40 shadow-md animate-pulse'
                      : 'bg-gray-400 shadow-gray-400/20 shadow-sm'
                  ]"
                />
              </div>
              <p class="text-xs text-muted-foreground">{{ formatCacheSize('theme') }}</p>
              <Button
                variant="outline"
                size="sm"
                class="w-full mt-2"
                :disabled="!cacheStatus.themeCacheExists"
                @click="clearSpecificCache('theme')"
              >
                清除
              </Button>
            </div>

            <div
              class="p-4 border rounded-lg transition-all duration-200 hover:shadow-md hover:border-primary/30 hover:scale-[1.02]"
            >
              <div class="flex items-center justify-between mb-2">
                <span class="font-medium">游戏数据</span>
                <div
                  :class="[
                    'size-3 rounded-full transition-all duration-200',
                    cacheStatus.gameDataCacheExists
                      ? 'bg-green-500 shadow-green-500/40 shadow-md animate-pulse'
                      : 'bg-gray-400 shadow-gray-400/20 shadow-sm'
                  ]"
                />
              </div>
              <p class="text-xs text-muted-foreground">{{ formatCacheSize('gameStore') }}</p>
              <Button
                variant="outline"
                size="sm"
                class="w-full mt-2"
                :disabled="!cacheStatus.gameDataCacheExists"
                @click="clearSpecificCache('gameData')"
              >
                清除
              </Button>
            </div>
          </div>

          <div class="flex justify-between items-center pt-4 border-t">
            <div>
              <p class="font-medium">总缓存大小: {{ totalCacheSize }}</p>
              <p class="text-sm text-muted-foreground">包含所有应用数据</p>
            </div>
            <Button variant="destructive" @click="clearAllCaches">
              <Trash2 class="size-4 mr-2" />
              清除所有缓存
            </Button>
          </div>
        </div>
      </div>
    </div>

    <!-- 高级设置 -->
    <div v-if="activeTab === 'advanced'" class="space-y-6">
      <div class="space-y-4">
        <h3 class="text-lg font-medium">高级设置</h3>

        <div class="space-y-4">
          <div class="flex items-center justify-between">
            <div>
              <p class="font-medium">开发者模式</p>
              <p class="text-sm text-muted-foreground">启用调试功能</p>
            </div>
            <Switch v-model:checked="configStore.config.advanced.developerMode" />
          </div>

          <div class="flex items-center justify-between">
            <div>
              <p class="font-medium">详细日志</p>
              <p class="text-sm text-muted-foreground">记录详细的操作日志</p>
            </div>
            <Switch v-model:checked="configStore.config.advanced.verboseLogging" />
          </div>

          <div class="space-y-2">
            <label class="text-sm font-medium">日志保留天数</label>
            <div class="flex items-center space-x-3">
              <input
                v-model.number="configStore.config.advanced.logRetentionDays"
                type="range"
                min="1"
                max="30"
                class="flex-1"
              />
              <span class="text-sm min-w-0"
                >{{ configStore.config.advanced.logRetentionDays }}天</span
              >
            </div>
          </div>

          <div class="space-y-2">
            <label class="text-sm font-medium">API请求频率限制</label>
            <div class="flex items-center space-x-3">
              <input
                v-model.number="configStore.config.advanced.apiRateLimit"
                type="range"
                min="10"
                max="100"
                class="flex-1"
              />
              <span class="text-sm min-w-0"
                >{{ configStore.config.advanced.apiRateLimit }}次/分钟</span
              >
            </div>
          </div>
        </div>

        <!-- 数据管理 -->
        <div class="space-y-4 pt-6 border-t">
          <h4 class="font-medium">数据管理</h4>
          <div class="flex flex-wrap gap-3">
            <Button variant="outline" size="sm" @click="exportSettings">
              <Download class="size-4 mr-2" />
              导出设置
            </Button>
            <Button variant="outline" size="sm" @click="importSettings">
              <Upload class="size-4 mr-2" />
              导入设置
            </Button>
            <Button variant="outline" size="sm" @click="resetToDefaults">
              <RotateCcw class="size-4 mr-2" />
              重置为默认
            </Button>
            <Button variant="destructive" size="sm" @click="resetAllData">
              <Trash2 class="size-4 mr-2" />
              重置所有数据
            </Button>
          </div>
        </div>
      </div>
    </div>

    <!-- 关于页面 -->
    <div v-if="activeTab === 'about'" class="space-y-6">
      <div class="space-y-4">
        <h3 class="text-lg font-medium">关于 Nidalee</h3>

        <div class="space-y-6">
          <div class="flex items-center space-x-4">
            <img src="/src/assets/logo.svg" alt="Nidalee Logo" class="size-16" />
            <div>
              <h4 class="text-xl font-bold">Nidalee</h4>
              <p class="text-muted-foreground">英雄联盟游戏助手</p>
              <p class="text-sm text-muted-foreground">版本 {{ appInfo.version }}</p>
            </div>
          </div>

          <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
            <div class="space-y-3">
              <h5 class="font-medium">应用信息</h5>
              <div class="space-y-2 text-sm">
                <div class="flex justify-between">
                  <span>版本:</span>
                  <span>{{ appInfo.version }}</span>
                </div>
                <div class="flex justify-between">
                  <span>构建日期:</span>
                  <span>{{ appInfo.buildDate }}</span>
                </div>
                <div class="flex justify-between">
                  <span>作者:</span>
                  <span>{{ appInfo.author }}</span>
                </div>
                <div class="flex justify-between">
                  <span>许可证:</span>
                  <span>{{ appInfo.license }}</span>
                </div>
              </div>
            </div>

            <div class="space-y-3">
              <h5 class="font-medium">系统信息</h5>
              <div class="space-y-2 text-sm">
                <div class="flex justify-between">
                  <span>操作系统:</span>
                  <span>{{ systemInfo.os }}</span>
                </div>
                <div class="flex justify-between">
                  <span>Node.js:</span>
                  <span>{{ systemInfo.node }}</span>
                </div>
                <div class="flex justify-between">
                  <span>Chrome:</span>
                  <span>{{ systemInfo.chrome }}</span>
                </div>
                <div class="flex justify-between">
                  <span>Tauri:</span>
                  <span>{{ systemInfo.tauri }}</span>
                </div>
              </div>
            </div>
          </div>

          <div class="space-y-3">
            <h5 class="font-medium">开源项目</h5>
            <p class="text-sm text-muted-foreground">
              本项目基于MIT许可证开源，欢迎贡献代码和反馈问题。
            </p>
            <div class="flex space-x-3">
              <Button variant="outline" size="sm">
                <Github class="w-4 h-4 mr-2" />
                GitHub
              </Button>
              <Button variant="outline" size="sm">
                <Globe class="w-4 h-4 mr-2" />
                官网
              </Button>
              <Button variant="outline" size="sm">
                <MessageCircle class="w-4 h-4 mr-2" />
                反馈
              </Button>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- 操作按钮 -->
    <div class="flex justify-end space-x-3 pt-6 border-t">
      <Button variant="outline" @click="cancelSettings"> 取消 </Button>
      <Button @click="saveSettings"> 保存设置 </Button>
    </div>

    <!-- 重置确认对话框 -->
    <Dialog v-model:open="showResetDialog">
      <DialogContent>
        <DialogHeader>
          <DialogTitle>确认重置</DialogTitle>
          <DialogDescription>
            {{ resetDialogMessage }}
          </DialogDescription>
        </DialogHeader>
        <DialogFooter>
          <Button variant="outline" @click="showResetDialog = false"> 取消 </Button>
          <Button variant="destructive" @click="confirmReset"> 确认重置 </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { Button } from '@/components/ui/button'
import { Switch } from '@/components/ui/switch'
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle
} from '@/components/ui/dialog'
import { Download, Upload, RotateCcw, Trash2, Github, Globe, MessageCircle } from 'lucide-vue-next'

// 导入 stores
import { useUserConfigStore } from '@/stores/useUserConfigStore'
import { useThemeStore } from '@/stores/useThemeStore'
import { useGameStore } from '@/stores/useGameStore'
import { CacheManager } from '@/utils/cacheManager'

// 使用 stores
const configStore = useUserConfigStore()
const themeStore = useThemeStore()
const gameStore = useGameStore()

const activeTab = ref('general')
const showResetDialog = ref(false)
const resetDialogMessage = ref('')
const resetAction = ref<(() => void) | null>(null)

const tabs = [
  { id: 'general', name: '通用' },
  { id: 'connection', name: '连接' },
  { id: 'notifications', name: '通知' },
  { id: 'cache', name: '缓存' },
  { id: 'advanced', name: '高级' },
  { id: 'about', name: '关于' }
]

const appInfo = ref({
  version: '1.0.0',
  buildDate: '2024-01-20',
  author: 'Nidalee Team',
  license: 'MIT'
})

const systemInfo = ref({
  os: 'Windows 11',
  node: 'v18.20.8',
  chrome: '120.0.0.0',
  tauri: '1.5.0'
})

// 缓存相关计算属性
const cacheStatus = computed(() => configStore.getCacheStatus())

const totalCacheSize = computed(() => {
  const totalBytes = CacheManager.getTotalCacheSize()
  return CacheManager.formatSize(totalBytes)
})

// 格式化缓存大小
function formatCacheSize(cacheType: string): string {
  const data = localStorage.getItem(cacheType)
  if (!data) return '0 B'
  const size = new Blob([data]).size
  return CacheManager.formatSize(size)
}

// 清除特定缓存
function clearSpecificCache(type: 'config' | 'theme' | 'gameData') {
  configStore.clearCache(type)

  gameStore.addNotification({
    message: `${type === 'config' ? '配置' : type === 'theme' ? '主题' : '游戏数据'}缓存已清除`,
    type: 'success'
  })
}

// 清除所有缓存
function clearAllCaches() {
  resetDialogMessage.value = '这将清除所有应用缓存数据，包括配置、主题和游戏数据。此操作不可撤销。'
  resetAction.value = () => {
    configStore.clearAllCache()
    // 重新加载页面以应用更改
    window.location.reload()
  }
  showResetDialog.value = true
}

// 重置为默认设置
function resetToDefaults() {
  resetDialogMessage.value = '这将重置所有设置为默认值，但保留缓存数据。'
  resetAction.value = () => {
    configStore.resetConfig()
    themeStore.resetToDefault()

    gameStore.addNotification({
      message: '设置已重置为默认值',
      type: 'success'
    })
  }
  showResetDialog.value = true
}

// 重置所有数据
function resetAllData() {
  resetDialogMessage.value =
    '这将删除所有应用数据，包括设置、主题、缓存和游戏数据。此操作不可撤销，应用将重启。'
  resetAction.value = () => {
    configStore.clearAllCache()
    configStore.resetConfig()
    themeStore.resetToDefault()
    gameStore.clearNotifications()
    gameStore.clearOperationLogs()

    // 重新加载页面
    setTimeout(() => {
      window.location.reload()
    }, 1000)
  }
  showResetDialog.value = true
}

// 确认重置
function confirmReset() {
  if (resetAction.value) {
    resetAction.value()
  }
  showResetDialog.value = false
  resetAction.value = null
}

// 保存设置
function saveSettings() {
  gameStore.addNotification({
    message: '设置已保存',
    type: 'success'
  })
}

// 取消设置
function cancelSettings() {
  // TODO: 恢复到原始设置
}

// 导出设置
function exportSettings() {
  try {
    const allData = CacheManager.exportAllCache()
    const blob = new Blob([allData], { type: 'application/json' })
    const url = URL.createObjectURL(blob)

    const a = document.createElement('a')
    a.href = url
    a.download = `nidalee-settings-${new Date().toISOString().split('T')[0]}.json`
    document.body.appendChild(a)
    a.click()
    document.body.removeChild(a)
    URL.revokeObjectURL(url)

    gameStore.addNotification({
      message: '设置导出成功',
      type: 'success'
    })
  } catch (error) {
    gameStore.addNotification({
      message: '设置导出失败',
      type: 'error'
    })
  }
}

// 导入设置
function importSettings() {
  const input = document.createElement('input')
  input.type = 'file'
  input.accept = '.json'

  input.onchange = async e => {
    const file = (e.target as HTMLInputElement).files?.[0]
    if (!file) return

    try {
      const text = await file.text()
      const success = CacheManager.importCache(text)

      if (success) {
        gameStore.addNotification({
          message: '设置导入成功，即将重新加载应用',
          type: 'success'
        })

        // 重新加载应用以应用导入的设置
        setTimeout(() => {
          window.location.reload()
        }, 2000)
      } else {
        throw new Error('Invalid settings file')
      }
    } catch (error) {
      gameStore.addNotification({
        message: '设置导入失败，请检查文件格式',
        type: 'error'
      })
    }
  }

  input.click()
}
</script>
