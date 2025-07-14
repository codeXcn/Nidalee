<template>
  <Dialog :open="true" @close="onCancel">
    <DialogTitle>发现新版本 {{ version }}</DialogTitle>
    <DialogDescription>
      <div v-if="body" class="mb-2 whitespace-pre-line">{{ body }}</div>
      <div v-if="progress.total > 0">
        <div class="text-xs text-muted-foreground mb-1">下载进度：{{ progress.downloaded }} / {{ progress.total }}</div>
        <progress :value="progress.downloaded" :max="progress.total" class="w-full h-2" />
      </div>
    </DialogDescription>
    <template #footer>
      <DialogFooter>
        <DialogClose asChild>
          <Button @click="onCancel" variant="secondary">取消</Button>
        </DialogClose>
        <Button v-if="!downloading" @click="onConfirm" :disabled="downloading">立即更新</Button>
        <Button v-else disabled>正在下载...</Button>
      </DialogFooter>
    </template>
  </Dialog>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'

const props = defineProps<{
  version: string
  body?: string
  progress?: { downloaded: number; total: number }
  onConfirm: () => void
  onCancel: () => void
}>()

const downloading = ref(false)

const onConfirm = async () => {
  downloading.value = true
  await props.onConfirm()
  downloading.value = false
}
const onCancel = () => {
  if (!downloading.value) props.onCancel()
}

const progress = props.progress || { downloaded: 0, total: 0 }
</script>

<style scoped>
progress {
  accent-color: #6366f1;
  border-radius: 4px;
}
</style>
