import { invoke } from '@tauri-apps/api/core'

export function useGameHelper() {
  /**
   * 设置召唤师生涯背景皮肤
   * @param skinId 皮肤ID
   * @returns Promise<void>
   */
  async function setSummonerBackgroundSkin(skinId: number): Promise<void> {
    try {
      await invoke('set_summoner_background_skin', { skinId })
      console.log(`成功设置生涯背景皮肤，皮肤ID: ${skinId}`)
    } catch (error) {
      console.error('设置生涯背景皮肤失败:', error)
      throw new Error(`设置生涯背景失败: ${error}`)
    }
  }
  return { setSummonerBackgroundSkin }
}
