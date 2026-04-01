<template>
  <div class="dashboard-container">
    <h1 class="dashboard-title">CAMFC</h1>
    
    <div class="nav-grid">
      <button class="nav-card file-manager" @click="goToFileView">
        <Cloud :size="48" class="card-icon" />
        <h3 class="card-title">云盘</h3>
      </button>
      
      <button class="nav-card notes" @click="goToNotes">
        <FileText :size="48" class="card-icon" />
        <h3 class="card-title">笔记</h3>
      </button>
      
      <button class="nav-card settings" @click="goToSettings">
        <Settings :size="48" class="card-icon" />
        <h3 class="card-title">设置</h3>
      </button>
    </div>
  </div>
</template>

<script setup>
import { useRouter } from 'vue-router'
import { Cloud, FileText, Settings } from 'lucide-vue-next'

const router = useRouter()

// 跳转到文件管理页面
function goToFileView() {
  router.push('/fileView')
}

// 跳转到设置页面
function goToSettings() {
  router.push('/settings')
}

// 跳转到笔记页面
function goToNotes() {
  router.push('/notes')
}
</script>

<style scoped>
.dashboard-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  min-height: calc(100vh - 100px);
  padding: 40px 20px;
  text-align: center;
}

.dashboard-title {
  font-size: 28px;
  margin-bottom: 40px;
  color: var(--text-primary, #f0f6fc);
}

.nav-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(220px, 1fr));
  gap: 25px;
  max-width: 1000px;
  width: 100%;
  margin-bottom: 40px;
}

.nav-card {
  background-color: var(--bg-secondary, #161b22);
  border: 1px solid var(--border-color, #30363d);
  border-radius: .375rem;
  padding: 25px 20px;
  cursor: pointer;
  transition: all 0.3s ease;
  text-align: center;
  outline: none;
  font-family: inherit;
  
  &:hover {
    box-shadow: 0 8px 20px rgba(0, 0, 0, 0.3);
    border-color: var(--accent-blue, #3178c6);
  }
}

.file-manager:hover {
  box-shadow: 0 8px 20px rgba(var(--accent-blue-rgb, 49, 120, 198), 0.2);
}

.settings:hover {
  box-shadow: 0 8px 20px rgba(var(--accent-green-rgb, 63, 185, 80), 0.2);
}

.notes:hover {
  box-shadow: 0 8px 20px rgba(var(--accent-purple-rgb, 188, 140, 255), 0.2);
}

.card-icon {
  font-size: 48px;
  margin-bottom: 15px;
  color: var(--text-primary, #24292f) !important;
}

.card-title {
  font-size: 20px;
  margin-bottom: 10px;
  color: var(--text-primary, #f0f6fc);
}

@media (max-width: 768px) {
  .nav-grid {
    grid-template-columns: 1fr;
    max-width: 400px;
  }
  
  .dashboard-title {
    font-size: 24px;
  }
}
</style>