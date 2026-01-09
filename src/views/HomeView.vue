<template>
  <div class="home">
    <h2>首页</h2>
    <p>这是最简陋的多路由网页的首页。</p>
    <p>点击上面的导航链接可以切换到其他页面。</p>
  </div>
</template>

<script>
// 最简单的组件，不需要任何逻辑
export default {
  name: 'HomeView'
}
</script>

<style scoped>
/* 加点最基本的样式，但保持简陋 */
.home {
  padding: 20px;
}
</style>