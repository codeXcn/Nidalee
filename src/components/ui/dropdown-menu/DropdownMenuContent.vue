<script setup lang="ts">
import type { HTMLAttributes } from 'vue'
import { reactiveOmit } from '@vueuse/core'
import {
  DropdownMenuContent,
  type DropdownMenuContentEmits,
  type DropdownMenuContentProps,
  DropdownMenuPortal,
  useForwardPropsEmits
} from 'reka-ui'
import { cn } from '@/lib/utils'

const props = withDefaults(
  defineProps<DropdownMenuContentProps & { class?: HTMLAttributes['class'] }>(),
  {
    sideOffset: 4
  }
)
const emits = defineEmits<DropdownMenuContentEmits>()

const delegatedProps = reactiveOmit(props, 'class')

const forwarded = useForwardPropsEmits(delegatedProps, emits)
</script>

<template>
  <DropdownMenuPortal>
    <DropdownMenuContent
      data-slot="dropdown-menu-content"
      v-bind="forwarded"
      :class="
        cn(
          `
        bg-popover/95 text-popover-foreground border-border 
        backdrop-blur-xl backdrop-saturate-150
        data-[state=open]:animate-in data-[state=closed]:animate-out 
        data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 
        data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95 
        data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 
        data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 
        z-50 max-h-(--reka-dropdown-menu-content-available-height) 
        min-w-[8rem] origin-(--reka-dropdown-menu-content-transform-origin) 
        overflow-x-hidden overflow-y-auto rounded-lg border border-border/50
        p-1 shadow-xl 
        dark:shadow-2xl dark:shadow-black/20
        dark:border-border/30 dark:bg-popover/90
      `,
          props.class
        )
      "
    >
      <slot />
    </DropdownMenuContent>
  </DropdownMenuPortal>
</template>
