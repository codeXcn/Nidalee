<template>
  <div class="relative w-full">
    <input
      ref="sliderRef"
      type="range"
      :min="min"
      :max="max"
      :step="step"
      :value="currentValue"
      @input="handleInput"
      @change="handleChange"
      class="slider-input w-full h-2 bg-muted rounded-lg appearance-none cursor-pointer focus:outline-none focus:ring-2 focus:ring-primary focus:ring-offset-2"
    />
    <div
      class="slider-track absolute top-1/2 left-0 h-2 bg-muted rounded-lg pointer-events-none"
      :style="{ transform: 'translateY(-50%)', width: '100%' }"
    />
    <div
      class="slider-range absolute top-1/2 left-0 h-2 bg-primary rounded-lg pointer-events-none"
      :style="{
        transform: 'translateY(-50%)',
        width: `${((currentValue - min) / (max - min)) * 100}%`
      }"
    />
    <div
      class="slider-thumb absolute top-1/2 w-4 h-4 bg-background border-2 border-primary rounded-full shadow-sm pointer-events-none transform -translate-x-1/2 -translate-y-1/2 transition-all hover:scale-110"
      :style="{
        left: `${((currentValue - min) / (max - min)) * 100}%`
      }"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue'

interface Props {
  modelValue?: number[] | number
  min?: number
  max?: number
  step?: number
  defaultValue?: number[] | number
}

const props = withDefaults(defineProps<Props>(), {
  min: 0,
  max: 100,
  step: 1,
  defaultValue: () => [50]
})

const emit = defineEmits<{
  'update:model-value': [value: number[] | number]
  'update:modelValue': [value: number[] | number]
}>()

const sliderRef = ref<HTMLInputElement>()

const currentValue = computed(() => {
  if (props.modelValue !== undefined) {
    return Array.isArray(props.modelValue) ? props.modelValue[0] : props.modelValue
  }
  return Array.isArray(props.defaultValue) ? props.defaultValue[0] : props.defaultValue
})

const handleInput = (event: Event) => {
  const target = event.target as HTMLInputElement
  const value = Number(target.value)
  updateValue(value)
}

const handleChange = (event: Event) => {
  const target = event.target as HTMLInputElement
  const value = Number(target.value)
  updateValue(value)
}

const updateValue = (value: number) => {
  if (Array.isArray(props.modelValue) || Array.isArray(props.defaultValue)) {
    emit('update:model-value', [value])
    emit('update:modelValue', [value])
  } else {
    emit('update:model-value', value)
    emit('update:modelValue', value)
  }
}

// 监听modelValue变化，同步到input
watch(currentValue, newValue => {
  if (sliderRef.value && sliderRef.value.value !== String(newValue)) {
    sliderRef.value.value = String(newValue)
  }
})
</script>

<style scoped>
.slider-input {
  -webkit-appearance: none;
  appearance: none;
  background: transparent;
}

.slider-input::-webkit-slider-track {
  background: transparent;
  border: none;
}

.slider-input::-webkit-slider-thumb {
  -webkit-appearance: none;
  appearance: none;
  background: transparent;
  border: none;
  width: 16px;
  height: 16px;
  cursor: pointer;
}

.slider-input::-moz-range-track {
  background: transparent;
  border: none;
}

.slider-input::-moz-range-thumb {
  background: transparent;
  border: none;
  width: 16px;
  height: 16px;
  cursor: pointer;
}

.slider-input:focus {
  outline: none;
}
</style>
