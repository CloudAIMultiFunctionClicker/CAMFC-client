<template>
  <div class="empty-container">
  </div>
</template>

<script setup>
import { onMounted } from 'vue'

onMounted(() => {
  console.log('空白窗口已加载')
})
</script>

<style scoped>
.empty-container {
  width: 100%;
  height: 100%;
  background-color: var(--bg-primary, #0d1117);
}
</style>
