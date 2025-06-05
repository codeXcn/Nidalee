<template>
  <DropdownMenu v-model:open="open">
    <DropdownMenuTrigger as-child>
      <Button
        variant="outline"
        role="combobox"
        :aria-expanded="open"
        :class="cn('w-full justify-between', props.class)"
      >
        {{ selectedLabel || placeholder }}
        <ChevronDown class="ml-2 h-4 w-4 shrink-0 opacity-50" />
      </Button>
    </DropdownMenuTrigger>
    <DropdownMenuContent class="w-[--reka-dropdown-menu-trigger-width] p-0">
      <DropdownMenuItem
        v-for="option in options"
        :key="option.value"
        @click="selectOption(option)"
        :class="
          cn('cursor-pointer', modelValue === option.value && 'bg-accent text-accent-foreground')
        "
      >
        <Check
          :class="cn('mr-2 h-4 w-4', modelValue === option.value ? 'opacity-100' : 'opacity-0')"
        />
        {{ option.label }}
      </DropdownMenuItem>
    </DropdownMenuContent>
  </DropdownMenu>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import type { HTMLAttributes } from 'vue'
import { Button } from '@/components/ui/button'
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger
} from '@/components/ui/dropdown-menu'
import { ChevronDown, Check } from 'lucide-vue-next'
import { cn } from '@/lib/utils'

interface SelectOption {
  value: string
  label: string
}

const props = withDefaults(
  defineProps<{
    modelValue?: string
    options: SelectOption[]
    placeholder?: string
    class?: HTMLAttributes['class']
    disabled?: boolean
  }>(),
  {
    placeholder: '请选择...'
  }
)

const emit = defineEmits<{
  'update:modelValue': [value: string]
}>()

const open = ref(false)

const selectedLabel = computed(() => {
  const selectedOption = props.options.find(option => option.value === props.modelValue)
  return selectedOption?.label
})

function selectOption(option: SelectOption) {
  emit('update:modelValue', option.value)
  open.value = false
}
</script>
