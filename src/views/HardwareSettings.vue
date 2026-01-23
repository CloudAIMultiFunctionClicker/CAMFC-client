<template>
  <div class="placeholder-container">
    <h1 class="placeholder-title">硬件设置</h1>
    
    <div class="placeholder-content">
      <!-- 占位图标 -->
      <div class="placeholder-icon">⚙️</div>
      
      <p class="placeholder-message">
        硬件设置功能正在开发中...
      </p>
      
      <p class="placeholder-desc">
        这里将用于配置Cpen设备的硬件参数，如蓝牙连接设置、设备校准等。
        <br>
        目前该页面仅为占位，后续会逐步完善。
      </p>
      
      <!-- 返回按钮 -->
      <button class="back-btn" @click="goBack">
        返回主仪表板
      </button>
    </div>
    
    <!-- TODO: 这里可以加个进度条或者预计完成时间 -->
    <p class="placeholder-footer">
      TODO: 硬件设置的具体功能还需要和硬件团队讨论确定
    </p>
  </div>
</template>

<script setup>
// 硬件设置占位页面
// 就是一个简单的占位页面，显示"功能开发中"
// 设计思路：和Settings.vue保持一致，简单明了

import { useRouter } from 'vue-router'

const router = useRouter()

// 返回主仪表板
// 思考：要不要用router.back()？但用户可能从不同地方来
// 还是固定跳转到/main吧，这样明确
function goBack() {
  console.log('返回主仪表板')
  router.push('/main')
}

// 注：这个页面没有复杂的逻辑，就是个展示页
// 如果以后要加表单或者设置项，再重构
</script>

<style scoped>
.placeholder-container {
  padding: 40px 20px;
  max-width: 800px;
  margin: 0 auto;
  text-align: center;
}

.placeholder-title {
  font-size: 28px;
  margin-bottom: 30px;
  color: var(--text-primary);
}

.placeholder-content {
  background-color: var(--bg-secondary);
  border-radius: 12px;
  padding: 40px 30px;
  border: 1px solid var(--border-color);
  margin-bottom: 30px;
}

.placeholder-icon {
  font-size: 64px;
  margin-bottom: 20px;
  /* 图标稍微有点大，但占位页面醒目点也行 */
}

.placeholder-message {
  font-size: 20px;
  color: var(--text-primary);
  margin-bottom: 15px;
  font-weight: 500;
}

.placeholder-desc {
  font-size: 16px;
  color: var(--text-secondary);
  line-height: 1.6;
  margin-bottom: 25px;
  max-width: 600px;
  margin-left: auto;
  margin-right: auto;
}

.back-btn {
  padding: 12px 30px;
  background-color: var(--accent-blue);
  color: white;
  border: none;
  border-radius: 8px;
  font-size: 16px;
  cursor: pointer;
  transition: background-color 0.2s;
  
  &:hover {
    background-color: #4a8bd6;
  }
  
  &:active {
    transform: translateY(1px);
  }
}

.placeholder-footer {
  font-size: 14px;
  color: var(--text-muted);
  font-style: italic;
  margin-top: 20px;
  padding-top: 20px;
  border-top: 1px dashed var(--border-color);
}

/* 响应式调整 */
@media (max-width: 768px) {
  .placeholder-container {
    padding: 20px 15px;
  }
  
  .placeholder-content {
    padding: 30px 20px;
  }
  
  .placeholder-icon {
    font-size: 48px;
  }
}
</style>