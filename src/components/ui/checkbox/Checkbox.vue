<template>
  <button
    :class="[
      'peer h-4 w-4 shrink-0 rounded-sm border ring-offset-background focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 transition-colors',
      checked
        ? 'bg-primary border-primary text-primary-foreground'
        : 'border-input bg-background hover:bg-accent hover:text-accent-foreground',
      $props.class
    ]"
    :data-state="checked ? 'checked' : 'unchecked'"
    @click="handleClick"
  >
    <svg
      v-if="checked"
      class="h-4 w-4"
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      stroke-width="2"
      stroke-linecap="round"
      stroke-linejoin="round"
    >
      <polyline points="20,6 9,17 4,12"></polyline>
    </svg>
  </button>
</template>

<script setup lang="ts">
interface Props {
  checked?: boolean
  class?: string
}

const props = withDefaults(defineProps<Props>(), {
  checked: false
})

const emit = defineEmits<{
  'update:checked': [value: boolean]
}>()

const handleClick = () => {
  emit('update:checked', !props.checked)
}
</script>
