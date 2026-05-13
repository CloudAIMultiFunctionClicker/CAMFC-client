<template>
  <div class="welcome-container">
    <!-- 顶部欢迎横幅 -->
    <div class="welcome-banner">
      <div class="banner-content">
        <h1 class="welcome-title">欢迎来到 CAMFC Cloud</h1>
        <p class="welcome-subtitle">智能蓝牙连接，让学习更高效</p>
      </div>
    </div>

    <!-- 主内容区 -->
    <div class="main-content">
      <!-- 左侧 - 快速入门教程 -->
      <div class="tutorial-section">
        <h2 class="section-title">
          <i class="ri-book-open-line"></i>
          快速入门
        </h2>
        
        <div class="tutorial-cards">
          <!-- 步骤 1 -->
          <div class="tutorial-card">
            <div class="card-header">
              <div class="step-number">1</div>
              <h3>连接设备</h3>
            </div>
            <div class="card-body">
              <p>首次使用需要先连接 Cpen 蓝牙设备</p>
              <ul class="step-list">
                <li>确保设备已开启并处于配对状态</li>
                <li>点击左侧边栏的"连接状态"</li>
                <li>选择您的设备进行连接</li>
              </ul>
            </div>
            <button class="action-btn" @click="goToConnection">
              <i class="ri-bluetooth-line"></i>
              <span>去连接</span>
            </button>
          </div>

          <!-- 步骤 2 -->
          <div class="tutorial-card">
            <div class="card-header">
              <div class="step-number">2</div>
              <h3>使用云盘</h3>
            </div>
            <div class="card-body">
              <p>管理您的文件和数据</p>
              <ul class="step-list">
                <li>浏览和上传文件到云端</li>
                <li>查看文件传输记录</li>
                <li>使用智能体自动化处理</li>
              </ul>
            </div>
            <button class="action-btn" @click="goToFileView">
              <i class="ri-folder-line"></i>
              <span>打开云盘</span>
            </button>
          </div>

          <!-- 步骤 3 -->
          <div class="tutorial-card">
            <div class="card-header">
              <div class="step-number">3</div>
              <h3>记录管理</h3>
            </div>
            <div class="card-body">
              <p>查看和管理您的学习记录</p>
              <ul class="step-list">
                <li>查看课堂记录</li>
                <li>管理个人笔记</li>
                <li>使用设备按键快速翻页</li>
              </ul>
            </div>
            <button class="action-btn" @click="goToNotes">
              <i class="ri-file-list-line"></i>
              <span>查看记录</span>
            </button>
          </div>

          <!-- 步骤 4 -->
          <div class="tutorial-card">
            <div class="card-header">
              <div class="step-number">4</div>
              <h3>班级管理</h3>
            </div>
            <div class="card-body">
              <p>加入或创建班级群组</p>
              <ul class="step-list">
                <li>查看已加入的群组</li>
                <li>申请加入新群组</li>
                <li>与同学共享资源</li>
              </ul>
            </div>
            <button class="action-btn" @click="goToGroupManager">
              <i class="ri-group-line"></i>
              <span>管理班级</span>
            </button>
          </div>
        </div>
      </div>

      <!-- 右侧 - 功能概览和提示 -->
      <div class="info-section">
        <!-- 功能特性 -->
        <div class="features-card">
          <h3 class="card-title">
            <i class="ri-lightbulb-line"></i>
            功能特性
          </h3>
          <div class="feature-list">
            <div class="feature-item">
              <i class="ri-bluetooth-connect-line"></i>
              <div>
                <h4>蓝牙连接</h4>
                <p>快速连接 Cpen 设备，实时同步数据</p>
              </div>
            </div>
            <div class="feature-item">
              <i class="ri-cloud-line"></i>
              <div>
                <h4>云端存储</h4>
                <p>不限容量的云空间，随时随地访问</p>
              </div>
            </div>
            <div class="feature-item">
              <i class="ri-shield-check-line"></i>
              <div>
                <h4>安全可靠</h4>
                <p>数据加密传输，保护隐私安全</p>
              </div>
            </div>
            <div class="feature-item">
              <i class="ri-robot-2-line"></i>
              <div>
                <h4>智能自动化</h4>
                <p>AI 智能体帮助您自动化处理任务</p>
              </div>
            </div>
          </div>
        </div>

        <!-- 快捷键提示 -->
        <div class="tips-card">
          <h3 class="card-title">
            <i class="ri-keyline"></i>
            使用技巧
          </h3>
          <div class="tip-list">
            <div class="tip-item">
              <span class="key-badge">设备按键</span>
              <span>右箭头：下一页 / 左箭头：上一页</span>
            </div>
            <div class="tip-item">
              <span class="key-badge">Ctrl+R</span>
              <span>刷新页面</span>
            </div>
            <div class="tip-item">
              <span class="key-badge">侧边栏</span>
              <span>点击折叠按钮可收起/展开菜单</span>
            </div>
            <div class="tip-item">
              <span class="key-badge">主题切换</span>
              <span>设置中可切换亮色/暗色主题</span>
            </div>
          </div>
        </div>

        <!-- 帮助与支持 -->
        <div class="support-card">
          <h3 class="card-title">
            <i class="ri-customer-service-2-line"></i>
            需要帮助？
          </h3>
          <p class="support-text">
            遇到问题或有任何建议？我们随时为您提供帮助
          </p>
          <div class="support-actions">
            <a href="https://github.com/CloudAIMultiFunctionClicker/CAMFC-client/issues" target="_blank" rel="noopener noreferrer" class="support-link">
              <i class="ri-bug-line"></i>
              <span>问题反馈</span>
            </a>
            <router-link to="/settings_help" class="support-link">
              <i class="ri-question-line"></i>
              <span>帮助中心</span>
            </router-link>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { useRouter } from 'vue-router'

const router = useRouter()

function goToConnection() {
  router.push('/')
}

function goToFileView() {
  router.push('/fileView')
}

function goToNotes() {
  router.push('/notes')
}

function goToGroupManager() {
  router.push('/group-manager')
}
</script>

<style scoped>
.welcome-container {
  display: flex;
  flex-direction: column;
  min-height: 100%;
  width: 100%;
  background: var(--bg-primary);
  overflow: hidden;
}

/* 欢迎横幅 */
.welcome-banner {
  padding: 40px 40px 30px;
  background: var(--bg-secondary);
  border-bottom: 1px solid var(--border-color);
  color: var(--text-primary);
  flex-shrink: 0;
}

.banner-content {
  max-width: 1200px;
  margin: 0 auto;
}

.welcome-title {
  font-size: 36px;
  font-weight: 700;
  margin: 0 0 8px 0;
  letter-spacing: -0.5px;
}

.welcome-subtitle {
  font-size: 18px;
  opacity: 0.9;
  margin: 0;
}

/* 主内容区 */
.main-content {
  flex: 1;
  display: grid;
  grid-template-columns: 2fr 1fr;
  gap: 30px;
  padding: 0 40px 40px;
  overflow-y: auto;
  max-width: 1600px;
  margin: 0 auto;
  width: 100%;
  box-sizing: border-box;
}

/* 教程区域 */
.tutorial-section {
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.section-title {
  display: flex;
  align-items: center;
  gap: 12px;
  font-size: 24px;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0;
}

.section-title i {
  font-size: 28px;
  color: var(--text-primary);
}

.tutorial-cards {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
  gap: 20px;
}

.tutorial-card {
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: 2px;
  padding: 24px;
  display: flex;
  flex-direction: column;
}

.card-header {
  display: flex;
  align-items: center;
  gap: 16px;
  margin-bottom: 16px;
}

.step-number {
  width: 40px;
  height: 40px;
  border-radius: 2px;
  background: var(--bg-tertiary);
  color: var(--text-primary);
  border: 1px solid var(--border-color);
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 18px;
  font-weight: 700;
  flex-shrink: 0;
}

.card-header h3 {
  margin: 0;
  font-size: 18px;
  font-weight: 600;
  color: var(--text-primary);
}

.card-body {
  flex: 1;
  margin-bottom: 20px;
}

.card-body p {
  margin: 0 0 12px 0;
  color: var(--text-secondary);
  font-size: 14px;
}

.step-list {
  margin: 0;
  padding-left: 20px;
  color: var(--text-secondary);
}

.step-list li {
  margin-bottom: 8px;
  font-size: 14px;
  line-height: 1.5;
}

.action-btn {
  display: inline-flex;
  align-items: center;
  gap: 8px;
  padding: 10px 20px;
  background: var(--bg-tertiary);
  border: 1px solid var(--border-color);
  border-radius: 2px;
  color: var(--text-primary);
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  width: 100%;
  justify-content: center;
}

.action-btn:hover {
  background: var(--text-primary);
  border-color: var(--text-primary);
  color: var(--bg-primary);
}

.action-btn i {
  font-size: 18px;
}

/* 信息区域 */
.info-section {
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.features-card,
.tips-card,
.support-card {
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: 2px;
  padding: 24px;
}

.card-title {
  display: flex;
  align-items: center;
  gap: 10px;
  font-size: 18px;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0 0 20px 0;
}

.card-title i {
  font-size: 22px;
  color: var(--text-primary);
}

/* 功能列表 */
.feature-list {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.feature-item {
  display: flex;
  gap: 12px;
  align-items: flex-start;
}

.feature-item i {
  font-size: 20px;
  color: var(--text-primary);
  flex-shrink: 0;
  margin-top: 2px;
}

.feature-item h4 {
  margin: 0 0 4px 0;
  font-size: 15px;
  font-weight: 600;
  color: var(--text-primary);
}

.feature-item p {
  margin: 0;
  font-size: 13px;
  color: var(--text-secondary);
  line-height: 1.4;
}

/* 技巧列表 */
.tip-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.tip-item {
  display: flex;
  align-items: center;
  gap: 12px;
  font-size: 14px;
  color: var(--text-secondary);
}

.key-badge {
  padding: 4px 8px;
  background: var(--bg-tertiary);
  border: 1px solid var(--border-color);
  border-radius: 2px;
  font-size: 12px;
  font-weight: 500;
  color: var(--text-primary);
  white-space: nowrap;
  flex-shrink: 0;
}

/* 支持卡片 */
.support-text {
  margin: 0 0 16px 0;
  font-size: 14px;
  color: var(--text-secondary);
  line-height: 1.5;
}

.support-actions {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.support-link {
  display: inline-flex;
  align-items: center;
  gap: 8px;
  padding: 10px 16px;
  background: var(--bg-tertiary);
  border: 1px solid var(--border-color);
  border-radius: 2px;
  color: var(--text-primary);
  text-decoration: none;
  font-size: 14px;
  font-weight: 500;
}

.support-link:hover {
  background: var(--hover-bg);
  border-color: var(--text-primary);
  color: var(--text-primary);
}

.support-link i {
  font-size: 18px;
}

/* 响应式调整 */
@media (max-width: 1200px) {
  .main-content {
    grid-template-columns: 1fr;
  }
  
  .tutorial-cards {
    grid-template-columns: repeat(2, 1fr);
  }
}

@media (max-width: 768px) {
  .welcome-banner {
    padding: 30px 20px 20px;
  }
  
  .welcome-title {
    font-size: 28px;
  }
  
  .welcome-subtitle {
    font-size: 16px;
  }
  
  .main-content {
    padding: 0 20px 20px;
  }
  
  .tutorial-cards {
    grid-template-columns: 1fr;
  }
  
  .section-title {
    font-size: 20px;
  }
}
</style>
