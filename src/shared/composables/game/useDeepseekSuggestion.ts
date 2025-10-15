export function useDeepseekSuggestion() {
  const loading = ref(false)
  const error = ref<string | null>(null)
  const data = ref<string>('')

  /**
   * 获取对局建议
   * @param context 你要传递的上下文（如队伍、英雄、战绩等，建议拼成一段文字）
   */
  async function fetchSuggestion(context: string) {
    loading.value = true
    error.value = null
    data.value = ''
    try {
      const res = await fetch('https://api.deepseek.com/chat/completions', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          Authorization: `Bearer ${process.env.VITE_DEEPSEEK_API_KEY}`
        },
        body: JSON.stringify({
          model: 'deepseek-chat',
          messages: [
            {
              role: 'system',
              content:
                '你是一个英雄联盟数据分析师，请根据用户提供的对局信息，给出详细的对局分析、英雄推荐和战术建议，内容简明扼要，适合直接展示在前端卡片。'
            },
            { role: 'user', content: context }
          ],
          stream: false
        })
      })
      if (!res.ok) throw new Error('API 请求失败')
      const json = await res.json()
      data.value = json.choices?.[0]?.message?.content || ''
    } catch (e: any) {
      error.value = e.message || '请求失败'
    } finally {
      loading.value = false
    }
  }

  return {
    loading,
    error,
    data,
    fetchSuggestion
  }
}
