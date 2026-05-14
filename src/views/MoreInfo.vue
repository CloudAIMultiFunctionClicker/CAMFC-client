

<template>
  <div class="placeholder-container">
    <h1 class="placeholder-title">更多信息</h1>

    <div class="placeholder-content">

      <div class="placeholder-icon">ℹ️</div>

      <p class="placeholder-message">
        更多信息页面正在开发中...
      </p>

      <p class="placeholder-desc">
        这里将提供关于CAMFC软件的详细信息，包括版本说明、帮助文档、技术支持等。
        <br>
        目前该页面仅为占位，后续会逐步完善。
      </p>

      <div class="temp-links">
        <p class="temp-links-title">您可以先访问以下现有页面：</p>
        <div class="link-buttons">
          <button class="link-btn" @click="goToAbout">
            关于我们
          </button>
          <button class="link-btn" @click="goToContact">
            联系我们
          </button>
        </div>
      </div>

      <button class="back-btn" @click="goBack">
        返回主仪表板
      </button>
    </div>

    <p class="placeholder-footer">
      TODO: 更多信息页面可能需要集成文档、FAQ、更新日志等
    </p>
  </div>
</template>

<script setup>

import { useRouter } from 'vue-router'

const router = useRouter()

function goBack() {
  console.log('返回主仪表板')
  router.push('/main')
}

function goToAbout() {
  console.log('跳转到关于页面')
  router.push('/about')
}

function goToContact() {
  console.log('跳转到联系页面')
  router.push('/contact')
}

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
  border-radius: 2px;
  padding: 40px 30px;
  border: 1px solid var(--border-color);
  margin-bottom: 30px;
}

.placeholder-icon {
  font-size: 64px;
  margin-bottom: 20px;
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

.temp-links {
  margin: 25px 0;
  padding: 20px;
  background-color: rgba(var(--accent-blue-rgb), 0.1);
  border-radius: 2px;
  border: 1px solid rgba(var(--accent-blue-rgb), 0.2);
}

.temp-links-title {
  font-size: 16px;
  color: var(--text-primary);
  margin-bottom: 15px;
}

.link-buttons {
  display: flex;
  justify-content: center;
  gap: 15px;
  flex-wrap: wrap;
}

.link-btn {
  padding: 10px 20px;
  background-color: var(--accent-blue);
  color: white;
  border: none;
  border-radius: 2px;
  font-size: 14px;
  cursor: pointer;
  transition: background-color 0.2s;

  &:hover {
    background-color: #4a8bd6;
  }

  &:active {
  }
}

.back-btn {
  padding: 12px 30px;
  background-color: var(--accent-blue);
  color: white;
  border: none;
  border-radius: 2px;
  font-size: 16px;
  cursor: pointer;
  transition: background-color 0.2s;
  margin-top: 20px;

  &:hover {
    background-color: #4a8bd6;
  }

  &:active {
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

  .link-buttons {
    flex-direction: column;
    align-items: center;
  }

  .link-btn {
    width: 100%;
    max-width: 200px;
  }
}
</style>