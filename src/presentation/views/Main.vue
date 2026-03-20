<template>
  <div class="main-view">
    <h1>文件管理</h1>
    <FileBrowser />
  </div>
</template>

<script setup lang="ts">
import FileBrowser from '../components/FileBrowser.vue';
</script>

<style scoped>
.main-view {
  max-width: 1000px;
  margin: 0 auto;
  padding: 20px;
}

h1 {
  color: #333;
  margin-bottom: 30px;
  text-align: center;
}
</style>