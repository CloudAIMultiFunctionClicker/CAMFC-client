<template>
  <div class="empty-container">
  </div>
</template>

<script setup>
import { onMounted } from 'vue'
import { useRouter } from 'vue-router'

const router = useRouter()

onMounted(() => {
  console.log('空白窗口已加载，重定向到截图页面')
  router.push('/screenshot')
})
</script>

<style scoped>
.empty-container {
  width: 100%;
  height: 100%;
  background-color: var(--bg-primary, #0d1117);
}
</style>
