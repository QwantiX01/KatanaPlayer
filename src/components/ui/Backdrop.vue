<script setup lang="ts">
import {computed} from "vue";

interface Props {
  height?: number
  width?: number
  roundness?: number
  variant: 'black' | 'dark'
}

const props = withDefaults(defineProps<Props>(), {
  roundness: 0,
})

const computedStyles = computed(() => ({
  height: props.height ? `${props.height}px` : undefined,
  width: props.width ? `${props.width}px` : undefined,
  borderRadius: props.roundness ? `${props.roundness}px` : undefined,
}))
</script>

<template>
  <div
      :class="[
      'control-bar',
      `control-bar--${props.variant}`
    ]"
      :style="computedStyles"
  >
    <slot></slot>
  </div>
</template>

<style scoped>
.control-bar {
  width: 100%;
  height: 100%;
  border-radius: 22px;
}

.control-bar--dark {
  background-color: var(--background-primary);
}

.control-bar--black {
  background-color: var(--background-dark);
}
</style>