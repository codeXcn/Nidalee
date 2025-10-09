import { toast } from 'vue-sonner'
import { ref } from 'vue'

// 全局更新状态，供界面展示
export const updateAvailable = ref(false)
export const updateVersion = ref<string | null>(null)
export const isUpdating = ref(false)
export const updateProgress = ref(0) // 0-100

// 运行时懒加载，避免在 Web 场景报错
async function getUpdater() {
  try {
    const mod = await import('@tauri-apps/plugin-updater')
    const proc = await import('@tauri-apps/plugin-process')
    return { updater: mod, process: proc }
  } catch (e) {
    console.warn('[Updater] 插件不可用，可能未在 Tauri 环境中运行', e)
    return null
  }
}

export async function checkAppUpdate(options?: { silent?: boolean; target?: string }) {
  const RELEASES_URL = 'https://github.com/codeXcn/Nidalee/releases/latest'
  const loaded = await getUpdater()
  if (!loaded) return
  const { updater, process } = loaded
  try {
    const update = await updater.check({ target: options?.target })
    if (!update) {
      updateAvailable.value = false
      updateVersion.value = null
      if (!options?.silent) toast.info('当前已是最新版本')
      return
    }

    // 有可用更新时，记录状态以用于 UI 展示
    updateAvailable.value = true
    updateVersion.value = update.version ?? null

    const ok = await new Promise<boolean>((resolve) => {
      toast(`发现新版本 ${update.version}`, {
        description: update.body ?? '',
        action: {
          label: '立即更新',
          onClick: () => resolve(true)
        },
        cancel: {
          label: '稍后再说',
          onClick: () => resolve(false)
        },
        duration: 10000
      })
    })

    if (!ok) return

    let total = 0
    await update.downloadAndInstall((evt: any) => {
      switch (evt.event) {
        case 'Started':
          total = evt.data.contentLength
          toast.info(`开始下载更新（${Math.round(total / 1024 / 1024)} MB）`)
          break
        case 'Progress':
          // 可根据需要汇总进度：evt.data.chunkLength
          break
        case 'Finished':
          toast.success('更新下载完成，准备安装')
          break
      }
    })

    // 安装完成后重启
    await process.relaunch()
  } catch (err) {
    console.error('[Updater] 检查/安装更新失败', err)
    toast.error('更新失败，请稍后重试', {
      action: {
        label: '前往手动下载',
        onClick: () => {
          try {
            window.open(RELEASES_URL, '_blank')
          } catch {}
        }
      },
      duration: 8000
    })
  }
}

// 立即开始下载并安装更新（用于侧边栏点击触发）
export async function startUpdateNow(options?: { target?: string }) {
  const RELEASES_URL = 'https://github.com/codeXcn/Nidalee/releases/latest'
  const loaded = await getUpdater()
  if (!loaded) return
  const { updater, process } = loaded
  try {
    const update = await updater.check({ target: options?.target })
    if (!update) {
      updateAvailable.value = false
      updateVersion.value = null
      toast.info('当前已是最新版本')
      return
    }

    updateAvailable.value = true
    updateVersion.value = update.version ?? null
    isUpdating.value = true
    updateProgress.value = 0

    let total = 0
    let received = 0
    await update.downloadAndInstall((evt: any) => {
      switch (evt.event) {
        case 'Started':
          total = evt.data.contentLength || 0
          received = 0
          toast.info(`开始下载更新（${total ? Math.round(total / 1024 / 1024) : '?'} MB）`)
          break
        case 'Progress':
          received += evt.data.chunkLength || 0
          if (total > 0) updateProgress.value = Math.min(100, Math.round((received / total) * 100))
          break
        case 'Finished':
          updateProgress.value = 100
          toast.success('更新下载完成，准备安装')
          break
      }
    })

    await process.relaunch()
  } catch (err) {
    console.error('[Updater] 立即更新失败', err)
    toast.error('更新失败，请稍后重试', {
      action: {
        label: '前往手动下载',
        onClick: () => {
          try {
            window.open(RELEASES_URL, '_blank')
          } catch {}
        }
      },
      duration: 8000
    })
  } finally {
    isUpdating.value = false
  }
}

// 标准组合式函数接口：返回全局的更新状态与方法
export function useAppUpdater() {
  return {
    updateAvailable,
    updateVersion,
    isUpdating,
    updateProgress,
    checkAppUpdate,
    startUpdateNow
  }
}
