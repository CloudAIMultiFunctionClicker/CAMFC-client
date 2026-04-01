<script setup>
import { ref, onMounted } from "vue";
import Icon from './components/Icon.vue';

const isDark = ref(false);

onMounted(() => {
  const html = document.documentElement;
  isDark.value = html.getAttribute("data-color-mode") === "dark";
});

function toggleTheme() {
  const html = document.documentElement;
  const newMode = isDark.value ? "light" : "dark";
  html.setAttribute("data-color-mode", newMode);
  isDark.value = !isDark.value;
}
</script>

<template>
  <main class="container-xl px-6 py-6">
    <div class="Box p-6">
      <div class="d-flex flex-justify-between flex-items-center mb-6">
        <h1 class="h1-mktg mb-0">Primer CSS 样式展示</h1>
        <button class="btn" @click="toggleTheme">
          <Icon :name="isDark ? 'sun' : 'moon'" :size="16" class="mr-2" />
          {{ isDark ? '亮色模式' : '暗色模式' }}
        </button>
      </div>
      
      <div class="mb-6">
        <h2 class="h2-mktg mb-3">按钮</h2>
        <div class="BtnGroup mr-2 mb-2">
          <button class="btn BtnGroup-item">默认</button>
          <button class="btn btn-primary BtnGroup-item">主要</button>
          <button class="btn btn-success BtnGroup-item">成功</button>
          <button class="btn btn-danger BtnGroup-item">危险</button>
        </div>
        <div class="mt-2">
          <button class="btn btn-sm">小按钮</button>
          <button class="btn">普通按钮</button>
          <button class="btn btn-large">大按钮</button>
        </div>
      </div>

      <div class="mb-6">
        <h2 class="h2-mktg mb-3">表单</h2>
        <div class="form-group mb-3">
          <label class="form-label">用户名</label>
          <input type="text" class="form-control" placeholder="请输入用户名" />
        </div>
        <div class="form-group mb-3">
          <label class="form-label">邮箱</label>
          <input type="email" class="form-control" placeholder="name@example.com" />
        </div>
        <div class="form-group mb-3">
          <label class="form-label">个人简介</label>
          <textarea class="form-control" rows="3" placeholder="介绍一下你自己..."></textarea>
        </div>
      </div>

      <div class="mb-6">
        <h2 class="h2-mktg mb-3">警告框</h2>
        <div class="flash flash-success mb-2">
          <Icon name="check" :size="16" class="mr-2" />
          操作成功完成！
        </div>
        <div class="flash flash-warn mb-2">
          <Icon name="alert" :size="16" class="mr-2" />
          请注意，此操作不可逆。
        </div>
        <div class="flash flash-error">
          <Icon name="x" :size="16" class="mr-2" />
          发生错误，请稍后重试。
        </div>
      </div>

      <div class="mb-6">
        <h2 class="h2-mktg mb-3">GitHub 风格卡片</h2>
        <div class="gh-card mb-3">
          <div class="gh-card-title">创建你的第一个代码项目</div>
          <div class="gh-card-description">在 GitHub 上创建一个项目。这是你自己的空间，可以尝试、学习，并与世界分享代码。</div>
          <button class="gh-card-action">
            <Icon name="repo" :size="12" class="mr-2" />
            创建项目
          </button>
        </div>
        <div class="gh-card">
          <div class="gh-card-title">开始学习 GitHub</div>
          <div class="gh-card-description">无论你是编程新手还是想提升版本控制技能，都能帮助你掌握 GitHub 的基础知识。</div>
          <button class="gh-card-action">开始播放列表</button>
        </div>
      </div>

      <div class="mb-6">
        <h2 class="h2-mktg mb-3">GitHub 风格顶部导航栏</h2>
        <div style="border: 1px solid var(--borderColor-default, #d1d9e0); border-radius: 6px; overflow: hidden;">
          <header class="gh-header">
            <div class="gh-header-left">
              <div class="gh-header-logo">
                <Icon name="mark-github" :size="32" />
              </div>
              <div class="gh-header-search">
                <Icon name="search" :size="16" class="gh-header-search-icon" />
                <input type="text" placeholder="Type / to search" />
              </div>
            </div>
            <div class="gh-header-right">
              <button class="gh-header-btn">
                <Icon name="triangle-down" :size="16" />
              </button>
              <button class="gh-header-btn">
                <Icon name="three-bars" :size="16" />
              </button>
              <img class="gh-header-avatar" src="https://github.com/github.png" alt="@user" width="32" height="32" />
            </div>
          </header>
        </div>
      </div>

      <div class="mb-6">
        <h2 class="h2-mktg mb-3">GitHub 风格侧边栏</h2>
        <div style="display: flex; height: 400px; border: 1px solid var(--borderColor-default, #d1d9e0); border-radius: 6px; overflow: hidden;">
          <aside class="gh-sidebar" style="min-height: auto;">
            <div class="gh-sidebar-header">
              <img class="gh-sidebar-logo" src="https://github.com/github.png" alt="@user" />
              <div>
                <div class="gh-sidebar-title">蚁穴011420</div>
                <div class="gh-sidebar-subtitle">Your personal account</div>
              </div>
            </div>
            <nav class="gh-sidebar-nav">
              <div class="gh-sidebar-section">
                <div class="gh-sidebar-section-title">账户</div>
                <a class="gh-sidebar-item active">
                  <Icon name="person" :size="16" class="gh-sidebar-icon" />
                  公众形象
                </a>
                <a class="gh-sidebar-item">
                  <Icon name="account" :size="16" class="gh-sidebar-icon" />
                  账号
                </a>
                <a class="gh-sidebar-item">
                  <Icon name="paintbrush" :size="16" class="gh-sidebar-icon" />
                  外观
                </a>
              </div>
              <div class="gh-sidebar-section">
                <div class="gh-sidebar-section-title">访问</div>
                <a class="gh-sidebar-item">
                  <Icon name="mail" :size="16" class="gh-sidebar-icon" />
                  电子邮件
                </a>
                <a class="gh-sidebar-item">
                  <Icon name="key" :size="16" class="gh-sidebar-icon" />
                  密码与认证
                </a>
              </div>
            </nav>
          </aside>
          <div style="flex: 1; padding: 24px; background-color: var(--bgColor-default, #fff);">
            <h3 style="font-size: 24px; font-weight: 600; margin-bottom: 16px;">Applications</h3>
            <p style="color: var(--fgColor-muted, #656d76);">这里显示主内容区域</p>
          </div>
        </div>
      </div>

      <div class="mb-6">
        <h2 class="h2-mktg mb-3">头像</h2>
        <div class="d-flex">
          <img class="avatar avatar-small mr-2" src="https://github.com/github.png" alt="@github" width="32" height="32" />
          <img class="avatar mr-2" src="https://github.com/github.png" alt="@github" width="40" height="40" />
          <img class="avatar avatar-large mr-2" src="https://github.com/github.png" alt="@github" width="60" height="60" />
        </div>
      </div>

      <div class="mb-6">
        <h2 class="h2-mktg mb-3">分页</h2>
        <nav class="paginate" aria-label="Pagination">
          <a class="paginate-next" href="#" aria-label="Next Page">Next</a>
        </nav>
      </div>
    </div>
  </main>
</template>

<style>
/* 强制显示标题样式 */
h2.h2-mktg {
  display: block !important;
  font-size: 1.5rem !important;
  font-weight: 600 !important;
  margin-bottom: 0.75rem !important;
  visibility: visible !important;
  opacity: 1 !important;
}

h3.h3-mktg {
  display: block !important;
  font-size: 1.25rem !important;
  font-weight: 600 !important;
  margin-bottom: 0.5rem !important;
  visibility: visible !important;
  opacity: 1 !important;
}
</style>
