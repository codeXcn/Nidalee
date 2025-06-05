<template>
  <div class="space-y-6">
    <div class="space-y-2">
      <h3 class="flex items-center gap-2 text-lg font-semibold text-gray-900 dark:text-gray-100">
        <Gamepad2 class="w-5 h-5" />
        游戏设置
      </h3>
      <p class="text-sm text-gray-600 dark:text-gray-400">
        配置英雄联盟游戏路径以确保Nidalee能够正确连接到游戏客户端
      </p>
    </div>

    <div class="space-y-6">
      <!-- 当前路径显示 -->
      <div class="space-y-3">
        <label class="block text-sm font-medium text-gray-700 dark:text-gray-300"
          >当前游戏路径</label
        >
        <div class="flex gap-2">
          <input
            v-model="gameStore.gamePath"
            type="text"
            class="flex-1 px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md bg-gray-50 dark:bg-gray-700 text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-blue-500 focus:border-transparent"
            placeholder="未设置游戏路径 - 将使用自动搜索"
            @input="handlePathInput"
          />
          <div class="flex gap-2">
            <button
              @click="openFileDialog"
              class="px-3 py-2 rounded-md font-medium text-sm transition-colors flex items-center gap-2 disabled:opacity-50 disabled:cursor-not-allowed bg-blue-600 hover:bg-blue-700 text-white"
              :disabled="selectingPath"
            >
              <FolderOpen v-if="!selectingPath" class="w-4 h-4" />
              <LoadingIcon v-else class="w-4 h-4 animate-spin" />
              {{ selectingPath ? '选择中...' : '浏览' }}
            </button>
            <!-- 隐藏的文件输入 -->
            <input
              ref="fileInput"
              type="file"
              webkitdirectory
              style="display: none"
              @change="handleFileSelect"
            />
            <button
              v-if="gameStore.gamePath"
              @click="clearPath"
              class="px-3 py-2 rounded-md font-medium text-sm transition-colors flex items-center gap-2 disabled:opacity-50 disabled:cursor-not-allowed bg-gray-200 hover:bg-gray-300 text-gray-700 dark:bg-gray-600 dark:hover:bg-gray-700 dark:text-gray-200"
            >
              <X class="w-4 h-4" />
              清除
            </button>
          </div>
        </div>
        <p class="text-xs text-gray-500 dark:text-gray-400">
          请选择英雄联盟的安装目录（例如：D:\WeGameApps\英雄联盟）或手动输入路径
        </p>
      </div>

      <!-- 连接状态 -->
      <div class="p-4 bg-gray-50 dark:bg-gray-800 rounded-lg">
        <div class="space-y-4">
          <div class="flex items-center gap-3">
            <div
              class="w-3 h-3 rounded-full transition-colors"
              :class="{
                'bg-green-500': gameStore.isConnected,
                'bg-yellow-500 animate-pulse': gameStore.gameStatus.connecting,
                'bg-red-500': !gameStore.isConnected && !gameStore.gameStatus.connecting
              }"
            />
            <span class="font-medium text-gray-900 dark:text-gray-100">
              {{ getConnectionStatusText() }}
            </span>
          </div>

          <div class="flex gap-2 flex-wrap">
            <button
              @click="testConnection"
              class="px-3 py-2 rounded-md font-medium text-sm transition-colors flex items-center gap-2 disabled:opacity-50 disabled:cursor-not-allowed bg-blue-600 hover:bg-blue-700 text-white"
              :disabled="gameStore.gameStatus.connecting"
            >
              <Wifi v-if="!gameStore.gameStatus.connecting" class="w-4 h-4" />
              <LoadingIcon v-else class="w-4 h-4 animate-spin" />
              {{ gameStore.gameStatus.connecting ? '连接中...' : '测试连接' }}
            </button>

            <button
              v-if="gameStore.isConnected"
              @click="gameStore.disconnectFromGame"
              class="px-3 py-2 rounded-md font-medium text-sm transition-colors flex items-center gap-2 disabled:opacity-50 disabled:cursor-not-allowed bg-gray-200 hover:bg-gray-300 text-gray-700 dark:bg-gray-600 dark:hover:bg-gray-700 dark:text-gray-200"
            >
              <WifiOff class="w-4 h-4" />
              断开连接
            </button>

            <button
              @click="gameStore.clearLcuCache"
              class="px-3 py-2 rounded-md font-medium text-sm transition-colors flex items-center gap-2 disabled:opacity-50 disabled:cursor-not-allowed bg-gray-200 hover:bg-gray-300 text-gray-700 dark:bg-gray-600 dark:hover:bg-gray-700 dark:text-gray-200"
              title="清除LCU缓存"
            >
              <RotateCcw class="w-4 h-4" />
              清除缓存
            </button>
          </div>
        </div>
      </div>

      <!-- LCU 信息显示 -->
      <div v-if="gameStore.lcuConfig" class="p-4 bg-blue-50 dark:bg-blue-900/20 rounded-lg">
        <h4 class="text-sm font-semibold text-gray-900 dark:text-gray-100 mb-3">LCU 连接信息</h4>
        <div class="space-y-2">
          <div class="flex justify-between">
            <span class="text-sm text-gray-600 dark:text-gray-400">端口:</span>
            <span class="text-sm font-mono text-gray-900 dark:text-gray-100">{{
              gameStore.lcuConfig.port
            }}</span>
          </div>
          <div class="flex justify-between">
            <span class="text-sm text-gray-600 dark:text-gray-400">令牌长度:</span>
            <span class="text-sm font-mono text-gray-900 dark:text-gray-100"
              >{{ gameStore.lcuConfig.token.length }} 字符</span
            >
          </div>
        </div>
      </div>

      <!-- 搜索说明 -->
      <div class="p-4 bg-yellow-50 dark:bg-yellow-900/20 rounded-lg">
        <h4 class="text-sm font-semibold text-gray-900 dark:text-gray-100 mb-3">自动搜索说明</h4>
        <div class="space-y-3 text-sm">
          <p>如果未设置游戏路径，Nidalee 将自动搜索以下位置：</p>
          <ul class="list-disc list-inside space-y-1 text-gray-600 dark:text-gray-400 ml-4">
            <li>C:\WeGameApps\英雄联盟</li>
            <li>D:\WeGameApps\英雄联盟</li>
            <li>E:\WeGameApps\英雄联盟</li>
            <li>F:\WeGameApps\英雄联盟</li>
          </ul>
          <p class="text-xs text-gray-500 dark:text-gray-400 italic">
            推荐手动设置游戏路径以获得更快的连接速度
          </p>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { Gamepad2, FolderOpen, X, Wifi, WifiOff, RotateCcw } from 'lucide-vue-next'
import { useGameStore } from '@/stores/useGameStore'
import { LoadingIcon } from '@/components/ui/icons'

const gameStore = useGameStore()
const selectingPath = ref(false)
const fileInput = ref<HTMLInputElement>()

// 组件挂载时加载保存的路径
onMounted(() => {
  gameStore.loadGamePath()
})

function openFileDialog() {
  // 触发隐藏的文件输入
  fileInput.value?.click()
}

function handleFileSelect(event: Event) {
  selectingPath.value = true

  const target = event.target as HTMLInputElement
  const files = target.files

  try {
    if (files && files.length > 0) {
      // 获取选择的目录路径
      const firstFile = files[0]
      let dirPath = ''

      if ('webkitRelativePath' in firstFile && firstFile.webkitRelativePath) {
        // 从 webkitRelativePath 获取目录路径
        const relativePath = firstFile.webkitRelativePath
        const pathParts = relativePath.split('/')

        // 取目录名（第一部分）
        if (pathParts.length > 0) {
          dirPath = pathParts[0]
        }
      }

      if (dirPath) {
        gameStore.setGamePath(dirPath)
        gameStore.addNotification({
          message: `游戏路径设置成功：${dirPath}`,
          type: 'success'
        })
      } else {
        gameStore.addNotification({
          message: '无法获取目录路径，请手动输入游戏安装目录',
          type: 'warning'
        })
      }
    }
  } catch (error) {
    console.error('处理文件选择失败:', error)
    gameStore.addNotification({
      message: '文件选择处理失败',
      type: 'error'
    })
  } finally {
    // 重置文件输入
    target.value = ''
    selectingPath.value = false
  }
}

function handlePathInput(event: Event) {
  const target = event.target as HTMLInputElement
  const path = target.value.trim()

  if (path) {
    gameStore.setGamePath(path)
  }
}

function clearPath() {
  gameStore.clearGamePath()
}

async function testConnection() {
  const success = await gameStore.connectToGame()

  if (!success) {
    gameStore.addNotification({
      message: '连接测试失败，请检查游戏路径或确保游戏客户端正在运行',
      type: 'warning'
    })
  }
}

function getConnectionStatusText(): string {
  if (gameStore.gameStatus.connecting) {
    return '正在连接...'
  } else if (gameStore.isConnected) {
    return '已连接'
  } else {
    return '未连接'
  }
}
</script>

<style scoped>
/* 所有样式现在直接在 class 属性中使用 Tailwind 工具类 */
</style>
