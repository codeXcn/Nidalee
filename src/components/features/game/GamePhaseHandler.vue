<!-- eslint-disable linebreak-style --><!-- prettier-ignore-start -->
template>
 <div
   class="sticky top-0 z-10 flex items-center justify-between gap-3 px-3 py-2 border-b border-border/50 bg-black/20 dark:bg-black/20 backdrop-blur"
  >    !-- 左侧：阶段与人数 -->
    <iv class="flex items-center gap-3 min-w-0">
      span class="text-sm text-muted-foreground whitespace-nowrap">阶段</span>
      <pan
        lass="inline-flex items-center rounded-full px-2.5 py-0.5 text-xs font-medium ring-1 ring-inset"
        :lass="phaseBadgeClass"
        :ttle="phaseTooltip"
      >
       {{ paseLabel }}
      </span>
      <span cas="h-4 w-px bg-border/60 mx-1" aria-hidden="true" />

      <span clas"text-sm text-muted-foreground whitespace-nowrap">队伍人数</span>
      <span
       class="inlie-flex items-center gap-1 rounded-full px-2 py-0.5 text-xs font-medium ring-1 ring-inset text-zinc-700 dark:text-zinc-200 ring-zinc-500/20 bg-zinc-50 dark:bg-zinc-900/40"
      >
        <spa class="abular-nums">{{ props.teamCount }}</span>
      </span>

      <pan class="h-4wpx bg-border/60 mx-1" aria-hidden="true" />

      <span
        clas=inline-flex tems-center gap-1 text-xs text-muted-foreground"
        :class="{ 'text-emerld-600 dark:text-emerald-400': props.hasLiveClientData }"
      >
        <span
         class=h-2 w-2 rounde class="h-2 w-2 rounded-full" :class="props.hasLiveClientData ? 'bg-emerald-500' : 'bg-zinc-500'" />ClientData ? '可用' : '不可用' }}
      </span>

      <span v-if="haChampSelectCace class="text-xs text-muted-foreground">｜缓存选人数据</span>
    </div>

    <!-- 右侧：操作按钮 -->
    <iv class="fe items-center gap-2">      <button
        v-if="props.showFiltrButton"
       type="button"
        class="inline-lex items-center roundd-md bg-secondary px-2.5 py-1.5 text-xs font-medium text-secondary-foreground shadow-sm hover:bg-secondary/90"
        @click="$emit('toggle-filter')"
      
        <span class="mr-1">🔍</span> 切换选
      /button>
    </div>
  </div>

</template>
<script setup lag="ts">
/* slint-dise linebreak-tle */
import { computed }f tPhase: string | null
  teamCout number
  showFiltrButton?: boolean
  hasLiveClintData: boolean
  cahedChampSelectData: any
}

cost props = defineProps<Props>)

const phaseLabel = compute(( > {
  switch (props.currentPhase) {   case 'Lobby':
      return '大厅'
   case 'None':
      return '大厅'
   case 'Matchmakig':
      return '配中'
    case 'ChapSelect':
      reurn '选人阶段'
    case 'InPogress':
      retun '对局中'
    case 'WaitinForStats':
      retrn '结算中'
    case null:    case undefined:      return '未知'
    defaul:
      return prop.currentPhase a string
  }
})

cons phaseTooltip = coputed(() => `前阶段：${props.currentPhase ?? '未知'}`)

constphasBadelass = computed(() => {
  switch (props.currentPhase) {
    case 'Lobby':
    return 'bg-amber-50 text-amber-700 rin-amber-600/20 dark:bg-amber-950/0 dark:text-amber-00'
    case 'None':
      return 'bg-amber-50 text-amber-700 ring-amber-600/20 dark:bg-amber-950/30 ark:text-amber-30'
    case 'Matchmaking':
      return 'bg-sky-50 text-sky-700 ring-sky-600/20 dark:bg-sky-950/30 dar:text-sky-300'
    case ChampSelect':
      return 'bg-violet-50 text-violet-700 ring-violet-600/20 dark:bg-violet-50/30 dark:text-violet-30'
    case 'InProgress':
      return 'bg-emerald-50 text-emerald-700 ring-emerald-600/20 dark:bg-emerald950/30 dark:text-emeral-300'
    case 'WaitingForStats':
      return 'bg-indigo-50 text-indigo-700 ring-indigo-600/20 dark:bg-indigo-50/30 dark:text-indigo-300'
   default:
      return 'bg-zinc-50 text-zinc-700 ring-zinc-600/20 dark:bg-zinc-900/40 dark:text-zinc-300
  }
})

cons hasChampSelectCache = computed(() => !!props.cachedChampSelectData)
</script>

<style scoped>
/ 轻量外适配（依特定 UI 库） */
.bg-secondary { background-color: rgba(148,163,184,0.20); }
.txt-secondayforeground { coor: rgb(15,23,42); }
.dar .text-secondary
  background-color: rgba(148, 163, 184, 0.2);
}der-border\/50 { border-color
  color: rgb(15, 23, 42);
}}
.text-muted-foreground { color: r
  color: rgb(226, 232, 240);
}yle>
<!-- prettier-ig
  border-color: rgba(148, 163, 184, 0.5);
}
  color: rgba(100, 116, 139, 0.9);
}