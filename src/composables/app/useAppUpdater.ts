import { h, render, ref } from 'vue'
import { check } from '@tauri-apps/plugin-updater'
import { relaunch } from '@tauri-apps/plugin-process'
import AppUpdateDialog from '@/components/AppUpdateDialog.vue'

import type { VNode } from 'vue'
let dialogVNode: VNode | null = null
let dialogContainer: HTMLElement | null = null

export function useAppUpdater() {
  // 进度状态
  const progress = ref({ downloaded: 0, total: 0 })

  // 检查更新
  const fetchVersion = async () => {
    const update = await check()
    if (update) {
      showUpdateDialog(update)
    }
  }

  // 用 h 渲染弹窗
  const showUpdateDialog = (update: { version: string; body?: string }) => {
    if (dialogContainer) return // 防止重复弹窗
    dialogContainer = document.createElement('div')
    document.body.appendChild(dialogContainer)
    progress.value = { downloaded: 0, total: 0 }
    dialogVNode = h(AppUpdateDialog, {
      version: update.version,
      body: update.body,
      progress: progress.value,
      onConfirm: async () => {
        await downloadAndInstall()
        closeDialog()
      },
      onCancel: closeDialog
    })
    render(dialogVNode, dialogContainer)
  }

  const closeDialog = () => {
    if (dialogVNode && dialogContainer) {
      render(null, dialogContainer)
      document.body.removeChild(dialogContainer)
      dialogVNode = null
      dialogContainer = null
    }
  }

  // 下载并安装
  const downloadAndInstall = async () => {
    const update = await check()
    if (update) {
      await update.downloadAndInstall((event) => {
        switch (event.event) {
          case 'Started':
            progress.value.total = event.data.contentLength ?? 0
            break
          case 'Progress':
            progress.value.downloaded += event.data.chunkLength
            break
        }
      })
      await relaunch()
    }
  }

  return { fetchVersion }
}
