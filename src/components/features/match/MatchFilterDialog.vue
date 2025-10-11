<template>
  <Dialog v-model:open="open">
    <DialogContent class="sm:max-w-md">
      <DialogHeader>
        <DialogTitle>战绩过滤</DialogTitle>
        <DialogDescription> 选择要显示的比赛类型 </DialogDescription>
      </DialogHeader>
      <div class="space-y-4">
        <div class="space-y-2">
          <Label>比赛类型</Label>
          <div class="grid grid-cols-2 gap-2">
            <div v-for="queueType in availableQueueTypes" :key="queueType.id" class="flex items-center space-x-2">
              <Checkbox
                :id="String(queueType.id)"
                :checked="selectedQueueTypes.includes(queueType.id)"
                @update:checked="toggleQueueType(queueType.id)"
              />
              <Label :for="String(queueType.id)" class="text-sm">{{ queueType.name }}</Label>
            </div>
          </div>
        </div>
      </div>
      <DialogFooter>
        <Button variant="outline" @click="reset">重置</Button>
        <Button @click="apply">应用</Button>
      </DialogFooter>
    </DialogContent>
  </Dialog>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue'
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle
} from '@/components/ui/dialog'
import { Button } from '@/components/ui/button'
import { Checkbox } from '@/components/ui/checkbox'
import { Label } from '@/components/ui/label'
import { useSearchMatches } from '@/composables/game/useSearchMatches'

interface Props {
  open: boolean
}

const props = defineProps<Props>()

const emit = defineEmits<{
  'update:open': [value: boolean]
}>()

const { selectedQueueTypes, setFilterTypes } = useSearchMatches()

// 定义可用的队列类型
const availableQueueTypes = ref([
  { id: 420, name: '单双排位' },
  { id: 440, name: '灵活组排' },
  { id: 450, name: '大乱斗' },
  { id: 490, name: '快速游戏' },
  { id: 400, name: '匹配模式' },
  { id: 700, name: '冲突模式' },
  { id: 1700, name: '斗魂竞技场' },
  { id: 1400, name: '终极技能书' },
  { id: 900, name: '无限火力' },
  { id: 1020, name: '一血模式' }
])

const open = computed({
  get: () => props.open,
  set: (value) => emit('update:open', value)
})

const toggleQueueType = (queueTypeId: number) => {
  const index = selectedQueueTypes.value.indexOf(queueTypeId)
  if (index > -1) {
    selectedQueueTypes.value.splice(index, 1)
  } else {
    selectedQueueTypes.value.push(queueTypeId)
  }
}

const reset = () => {
  selectedQueueTypes.value = []
}

const apply = () => {
  setFilterTypes(selectedQueueTypes.value)
  open.value = false
}
</script>
