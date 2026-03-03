<template>
  <div class="float-container">
    <span class="float-text">云盘</span>
  </div>
</template>

<script setup>
</script>

<style scoped>
.float-container {
  width: 100vw;
  height: 100vh;
  display: flex;
  align-items: center;
  justify-content: center;
  background-color: transparent;
  -webkit-app-region: drag;
  user-select: none;
}

.float-text {
  font-size: 14px;
  font-weight: bold;
  color: var(--text-primary, #333);
}
</style>
