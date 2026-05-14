

<script setup>

  import AppHeader from '../components/layout/AppHeader.vue'

  import { ref } from 'vue'

  import FileTable from '../components/file/FileTable.vue'

  import FileTree from '../components/file/FileTree.vue'

  const currentPath = ref('')

  const handlePathChange = (newPath) => {
    console.log('路径变化:', newPath)
    currentPath.value = newPath
  }
</script>

<template>

  <div class="main-container">

    <div class="content-area">

      <FileTable
        :currentPath="currentPath"
        @path-change="handlePathChange"
      />
    </div>
  </div>
</template>

<style scoped>

  .main-container {
    display: flex;
    width: 100%;
    height: calc(100vh - 48px);
    overflow: hidden;
  }

  .content-area {
    flex: 1;
    background: var(--bg-primary, #0f172a);
    padding: 24px;
    padding-left: 24px;
    box-sizing: border-box;
    overflow: hidden;
    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    display: flex;
    flex-direction: column;
  }

  .hint {
    margin-top: 10px;
    font-size: 0.9rem;
    color: var(--text-muted, #64748b);
    font-style: italic;
  }

  @media (max-width: 768px) {
    .main-container {
      height: calc(100vh - 64px);
    }

    .content-area {
      padding: 16px;
    }
  }
</style>
