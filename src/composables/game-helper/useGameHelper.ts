import { invoke } from '@tauri-apps/api/core'
import { toast } from 'vue-sonner'
import { useActivityLogger } from '@/composables/utils/useActivityLogger'

export function useGameHelper() {
  const activityLogger = useActivityLogger()
  const updatingNote = ref(false)
  const updatingRank = ref(false)

  /**
   * 设置召唤师生涯背景皮肤
   * @param skinId 皮肤ID
   * @returns Promise<void>
   */
  async function setSummonerBackgroundSkin(skinId: number, skinName?: string): Promise<void> {
    try {
      await invoke('set_summoner_background_skin', { skinId })
      if (skinName) {
        activityLogger.logSettings.setCareerBackground(skinName)
      }
      toast.success(`皮肤${skinName ? `"${skinName}"` : ''}已设置为生涯背景`, {
        description: '生涯背景设置成功完成',
        duration: 3000
      })
    } catch (error) {
      toast.error('设置生涯背景失败', {
        description: String(error),
        duration: 5000
      })
    }
  }

  /**
   * 通用设置召唤师聊天资料（签名/段位信息）
   */
  async function setSummonerChatProfile({
    statusMessage,
    queue,
    tier,
    division
  }: {
    statusMessage?: string
    queue?: string
    tier?: string
    division?: string
  }): Promise<void> {
    // 判断是签名还是段位修改，分别 loading
    const isNote = !!statusMessage && !queue && !tier && !division
    const isRank = !!queue && !!tier && !!division
    if (isNote) updatingNote.value = true
    if (isRank) updatingRank.value = true
    try {
      await invoke('set_summoner_chat_profile', {
        statusMessage,
        queue,
        tier,
        division
      })
      if (isNote) {
        toast.success('签名修改成功', {
          description: '召唤师签名已更新',
          duration: 3000
        })
        activityLogger.logSettings.setCareerBackground('修改签名')
      } else if (isRank) {
        toast.success('段位信息修改成功', {
          description: '聊天段位已更新',
          duration: 3000
        })
        activityLogger.logSettings.setCareerBackground('修改段位')
      } else {
        toast.success('资料修改成功', { duration: 3000 })
      }
    } catch (error) {
      if (isNote) {
        toast.error('签名修改失败', {
          description: String(error),
          duration: 5000
        })
      } else if (isRank) {
        toast.error('段位信息修改失败', {
          description: String(error),
          duration: 5000
        })
      } else {
        toast.error('资料修改失败', { description: String(error), duration: 5000 })
      }
    } finally {
      if (isNote) updatingNote.value = false
      if (isRank) updatingRank.value = false
    }
  }

  return { setSummonerBackgroundSkin, setSummonerChatProfile, updatingNote, updatingRank }
}
