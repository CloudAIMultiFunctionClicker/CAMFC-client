<script setup>
  import AppHeader from '../components/layout/AppHeader.vue'

</script>

<template>
  <AppHeader/>
  <div class="content-area"></div>
</template>

<script>
// 最简单的组件，不需要任何逻辑
export default {
  name: 'HomeView'
}
</script>


<style scoped>
  .content-area {
    width: 100%;
    height: 100%;
    background:#1a1a1a
  }
</style>