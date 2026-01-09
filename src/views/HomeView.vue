<script setup>
  // 导入应用头部组件
  import AppHeader from '../components/layout/AppHeader.vue'
  // 这个主页视图很简单，就是显示头部和一个内容区域
  // TODO: 未来可能需要在这里添加更多内容，比如文件列表等
</script>

<template>
  <!-- 主页布局 -->
  <AppHeader/>
  <!-- 主要内容区域 - 目前是空的，只是占位 -->
  <div class="content-area"></div>
</template>

<script setup>
</script>


<style scoped>
  /* 内容区域样式 */
  .content-area {
    width: 100%;
    height: 100%;
    background: #1a1a1a; /* 深色背景，与头部协调 */
  }
</style>
