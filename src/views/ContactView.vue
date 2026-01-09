<template>
  <!-- 联系页面 - 演示用路由页面 -->
  <div class="contact">
    <h2>联系页面</h2>
    <p>这是第三个路由页面。</p>
    <p>路由切换应该能正常工作。</p>
    <!-- TODO: 未来可以在这里添加联系方式、表单等 -->
  </div>
</template>

<script>
// 联系页面的组件定义
// 目前只有最基本的展示功能
export default {
  name: 'ContactView'
}
</script>

<style scoped>
/* 联系页面容器样式 */
.contact {
  padding: 20px; /* 内边距，保持与其他页面一致 */
}
</style>
