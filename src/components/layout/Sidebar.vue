<!--
保留所有权利

Copyright (C) 2026 Jiale Xu (许嘉乐) (ANTmmmmm) <https://github.com/ant-cave>
Email: ANTmmmmm@outlook.com, ANTmmmmm@126.com, 1504596931@qq.com

Copyright (C) 2026 Xinhang Chen (陈欣航) <https://github.com/cxh09>
Email: abc.cxh2009@foxmail.com

Copyright (C) 2026 Zimo Wen (温子墨) <https://github.com/lusamaqq>
Email: 1220594170@qq.com

Copyright (C) 2026 Kaibin Zeng (曾楷彬) <https://github.com/Waple1145>
Email: admin@mc666.top
-->

<!--
左侧边栏组件 - 用于主要导航
现在加上新的折叠功能：完全消失 + 悬浮按钮！
TODO: 动态菜单、路由高亮这些以后再加
FIXME: 悬浮按钮的样式还可以再优化，让它更融入整体设计
-->

<script setup>
// 导入 Vue 的响应式功能
import { ref, computed, onMounted } from "vue";
import { useRoute } from "vue-router";
// 导入 Pinia store 来获取蓝牙状态
import { useBluetoothStore } from "../../stores/bluetooth";
import { WebviewWindow } from '@tauri-apps/api/webviewWindow'

// 侧边栏折叠状态 - 默认展开
const isCollapsed = ref(false);

// 定义 emit 函数，用于触发事件
const emit = defineEmits(["collapse-change"]);

// 切换折叠状态的函数
const toggleCollapse = () => {
    isCollapsed.value = !isCollapsed.value;
    // 触发事件，通知父组件状态变化
    emit("collapse-change", isCollapsed.value);
};

// 获取蓝牙 store
const bluetoothStore = useBluetoothStore();

// 计算蓝牙是否已连接
const isConnected = computed(() => bluetoothStore.isConnected());

const route = useRoute()

// 根据路径匹配所属的一级菜单
const getMenuKeyByPath = (path) => {
    if (path === '/' || path === '/welcome') return 'connection'
    if (path.startsWith('/fileView') || path.startsWith('/transfer') || path.startsWith('/recent-activities') || path.startsWith('/agent-window')) return 'cloud'
    if (path.startsWith('/notes') || path.startsWith('/notes_')) return 'records'
    if (path.startsWith('/group-manager') || path.startsWith('/group-detail')) return 'class'
    if (path.startsWith('/settings') || path.startsWith('/settings_')) return 'settings'
    return null
}

// 菜单折叠状态管理
const expandedMenus = ref({
    connection: false,  // 连接状态
    cloud: false,       // 云盘
    records: false,     // 记录
    class: false,       // 班级管理
    settings: false     // 设置
});

// 检查菜单是否可展开（连接状态始终可用，其他需要蓝牙连接）
const isMenuDisabled = (menuKey) => {
    if (menuKey === 'connection') return false
    return !isConnected.value
}

// 切换菜单展开/折叠（手风琴：同时只展开一个）
const toggleMenu = (menuKey) => {
    if (isMenuDisabled(menuKey)) return
    const willExpand = !expandedMenus.value[menuKey]
    // 全部收起
    Object.keys(expandedMenus.value).forEach(key => {
        expandedMenus.value[key] = false
    })
    // 只展开当前点击的那个
    expandedMenus.value[menuKey] = willExpand
};

// 打开 agent 自动化窗口（从废弃的 Main.vue 移过来）
const openAgentWindow = async () => {
  const agentWindow = new WebviewWindow('agent-window', {
    url: '/agent-window',
    title: '自动执行 - CAMFC',
    width: 600,
    height: 700,
    resizable: true,
    center: true,
    decorations: true,
    maximizable: false,
    fullscreen: false,
  })

  agentWindow.once('tauri://created', () => {
    console.log('agent 窗口已创建')
  })

  agentWindow.once('tauri://error', (e) => {
    console.error('创建 agent 窗口失败:', e)
    WebviewWindow.getByLabel('agent-window').then(w => {
      if (w) { w.show(); w.setFocus() }
    })
  })
}

// 挂载时自动展开当前页面所在的一级菜单
onMounted(() => {
    const key = getMenuKeyByPath(route.path)
    if (key) {
        expandedMenus.value[key] = true
    }
})
</script>

<template>
    <!-- 悬浮按钮 - 只在侧边栏收起时显示 -->
    <button
        v-if="isCollapsed"
        class="float-collapse-btn"
        @click="toggleCollapse"
        title="展开侧边栏"
    >
        <i class="ri-side-bar-line"></i>
    </button>

    <!-- 侧边栏容器 - 根据折叠状态添加类名 -->
    <!-- 移除了v-show，让CSS过渡处理显示/隐藏 -->
    <aside
        class="sidebar"
        :class="{ collapsed: isCollapsed }"
    >
        <!-- Logo 区域 - 简单放个标题 -->
        <div class="logo-area">
            <h2>
                <i class="ri-folder-line"></i>
                <!-- 文件夹图标，跟云存储主题相关 -->
                <span>CAMFC Cloud</span>
            </h2>
            <!-- 用量进度条 -->
            <div class="storage-usage">
                <div class="usage-label">
                    <span>云空间用量</span>
                    <span class="usage-text">不限容量</span>
                </div>
                <div class="usage-bar">
                    <div class="usage-progress unlimited"></div>
                </div>
            </div>

            <!-- 折叠按钮 - 放在 logo 区域右上角 -->
            <button
                class="collapse-btn"
                @click="toggleCollapse"
                title="收起侧边栏"
            >
                <i class="ri-arrow-left-s-line"></i>
            </button>
        </div>

        <!-- 主菜单区域 -->
        <nav class="main-menu">
            <!-- 连接状态（默认显示蓝牙连接扫描） -->
            <div class="menu-section">
                <div class="menu-section-header" @click="toggleMenu('connection')">
                    <div class="menu-section-title">
                        <i class="ri-bluetooth-line"></i>
                        <span>连接状态</span>
                    </div>
                    <i :class="expandedMenus.connection ? 'ri-arrow-down-s-line' : 'ri-arrow-right-s-line'" class="expand-icon"></i>
                </div>
                <ul v-show="expandedMenus.connection" class="menu-list">
                    <li class="menu-item">
                        <router-link to="/welcome" class="menu-link">
                            <i class="ri-home-smile-line"></i>
                            <span>欢迎页面</span>
                        </router-link>
                    </li>
                    <li class="menu-item">
                        <router-link to="/" class="menu-link">
                            <i class="ri-wifi-line"></i>
                            <span>蓝牙扫描</span>
                        </router-link>
                    </li>
                </ul>
            </div>

            <!-- 云盘 -->
            <div class="menu-section">
                <div class="menu-section-header" :class="{ disabled: isMenuDisabled('cloud') }" :title="isMenuDisabled('cloud') ? '需要连接蓝牙先' : ''" @click="toggleMenu('cloud')">
                    <div class="menu-section-title">
                        <i class="ri-cloud-line"></i>
                        <span>云盘</span>
                    </div>
                    <i :class="expandedMenus.cloud ? 'ri-arrow-down-s-line' : 'ri-arrow-right-s-line'" class="expand-icon"></i>
                </div>
                <ul v-show="expandedMenus.cloud" class="menu-list">
                    <li class="menu-item">
                        <router-link to="/fileView" class="menu-link" :class="{ disabled: !isConnected }">
                            <i class="ri-folder-line"></i>
                            <span>文件</span>
                        </router-link>
                    </li>
                    <li class="menu-item">
                        <router-link to="/transfer" class="menu-link" :class="{ disabled: !isConnected }">
                            <i class="ri-exchange-line"></i>
                            <span>传输</span>
                        </router-link>
                    </li>
                    <li class="menu-item">
                        <router-link to="/recent-activities" class="menu-link" :class="{ disabled: !isConnected }">
                            <i class="ri-history-line"></i>
                            <span>最近活动</span>
                        </router-link>
                    </li>
                    <li class="menu-item">
                        <a class="menu-link" :class="{ disabled: !isConnected }" @click="openAgentWindow">
                            <i class="ri-robot-2-line"></i>
                            <span>智能体</span>
                        </a>
                    </li>
                </ul>
            </div>

            <!-- 记录 -->
            <div class="menu-section">
                <div class="menu-section-header" :class="{ disabled: isMenuDisabled('records') }" :title="isMenuDisabled('records') ? '需要连接蓝牙先' : ''" @click="toggleMenu('records')">
                    <div class="menu-section-title">
                        <i class="ri-file-list-line"></i>
                        <span>记录</span>
                    </div>
                    <i :class="expandedMenus.records ? 'ri-arrow-down-s-line' : 'ri-arrow-right-s-line'" class="expand-icon"></i>
                </div>
                <ul v-show="expandedMenus.records" class="menu-list">
                    <li class="menu-item">
                        <router-link to="/notes_meetings" class="menu-link" :class="{ disabled: !isConnected }">
                            <i class="ri-team-line"></i>
                            <span>课堂记录</span>
                        </router-link>
                    </li>
                    <li class="menu-item">
                        <router-link to="/notes_notes" class="menu-link" :class="{ disabled: !isConnected }">
                            <i class="ri-sticky-note-line"></i>
                            <span>笔记</span>
                        </router-link>
                    </li>
                </ul>
            </div>

            <!-- 班级管理 -->
            <div class="menu-section">
                <div class="menu-section-header" :class="{ disabled: isMenuDisabled('class') }" :title="isMenuDisabled('class') ? '需要连接蓝牙先' : ''" @click="toggleMenu('class')">
                    <div class="menu-section-title">
                        <i class="ri-group-line"></i>
                        <span>班级管理</span>
                    </div>
                    <i :class="expandedMenus.class ? 'ri-arrow-down-s-line' : 'ri-arrow-right-s-line'" class="expand-icon"></i>
                </div>
                <ul v-show="expandedMenus.class" class="menu-list">
                    <li class="menu-item">
                        <router-link to="/group-manager_groups" class="menu-link" :class="{ disabled: !isConnected }">
                            <i class="ri-group-line"></i>
                            <span>我的群组</span>
                        </router-link>
                    </li>
                    <li class="menu-item">
                        <router-link to="/group-manager_applications" class="menu-link" :class="{ disabled: !isConnected }">
                            <i class="ri-notification-badge-line"></i>
                            <span>待处理申请</span>
                        </router-link>
                    </li>
                </ul>
            </div>

            <!-- 设置 -->
            <div class="menu-section">
                <div class="menu-section-header" :class="{ disabled: isMenuDisabled('settings') }" :title="isMenuDisabled('settings') ? '需要连接蓝牙先' : ''" @click="toggleMenu('settings')">
                    <div class="menu-section-title">
                        <i class="ri-settings-line"></i>
                        <span>设置</span>
                    </div>
                    <i :class="expandedMenus.settings ? 'ri-arrow-down-s-line' : 'ri-arrow-right-s-line'" class="expand-icon"></i>
                </div>
                <ul v-show="expandedMenus.settings" class="menu-list">
                    <li class="menu-item">
                        <router-link to="/settings_cpen" class="menu-link" :class="{ disabled: !isConnected }">
                            <i class="ri-settings-3-line"></i>
                            <span>Cpen 设置</span>
                        </router-link>
                    </li>
                    <li class="menu-item">
                        <router-link to="/settings_hardware" class="menu-link" :class="{ disabled: !isConnected }">
                            <i class="ri-link"></i>
                            <span>连接设置</span>
                        </router-link>
                    </li>
                    <li class="menu-item">
                        <router-link to="/settings_student" class="menu-link" :class="{ disabled: !isConnected }">
                            <i class="ri-user-line"></i>
                            <span>学生认证</span>
                        </router-link>
                    </li>
                    <li class="menu-item">
                        <router-link to="/settings_download" class="menu-link" :class="{ disabled: !isConnected }">
                            <i class="ri-download-line"></i>
                            <span>下载设置</span>
                        </router-link>
                    </li>
                    <li class="menu-item">
                        <router-link to="/settings_application" class="menu-link" :class="{ disabled: !isConnected }">
                            <i class="ri-apps-line"></i>
                            <span>应用设置</span>
                        </router-link>
                    </li>
                    <li class="menu-item">
                        <router-link to="/settings_theme" class="menu-link" :class="{ disabled: !isConnected }">
                            <i class="ri-moon-line"></i>
                            <span>深色模式</span>
                        </router-link>
                    </li>
                    <li class="menu-item">
                        <router-link to="/settings_help" class="menu-link" :class="{ disabled: !isConnected }">
                            <i class="ri-question-line"></i>
                            <span>帮助与反馈</span>
                        </router-link>
                    </li>
                    <li class="menu-item">
                        <router-link to="/settings_about" class="menu-link" :class="{ disabled: !isConnected }">
                            <i class="ri-information-line"></i>
                            <span>关于</span>
                        </router-link>
                    </li>
                </ul>
            </div>
        </nav>


    </aside>
</template>

<style scoped>
/* 侧边栏基础样式 - 使用 CSS 变量支持主题切换 */
/* 现在颜色都从全局变量获取，亮色/暗色模式自动切换 */

.sidebar {
    width: 240px;
    height: calc(100vh - 48px); /* 减去标题栏高度 */
    background: var(--bg-sidebar, #161b22);
    border-right: 1px solid var(--border-color, #30363d);
    display: flex;
    flex-direction: column;
    padding: 20px 0;
    box-sizing: border-box;
    position: fixed;
    top: 48px; /* 标题栏高度 */
    left: 0;
    bottom: 0;
    z-index: 900;
    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    overflow-y: auto;
}

/* 自定义滚动条样式 */
.sidebar::-webkit-scrollbar {
    width: 6px;
}

.sidebar::-webkit-scrollbar-track {
    background: transparent;
}

.sidebar::-webkit-scrollbar-thumb {
    background: var(--border-color, #30363d);
    border-radius: 2px;
}

.sidebar::-webkit-scrollbar-thumb:hover {
    background: var(--text-muted, #8b949e);
}

/* 折叠状态 - 侧边栏向左滑出屏幕 */
.sidebar.collapsed {
    width: 240px; /* 保持宽度，但靠transform来移动 */
    opacity: 0;
    transform: translateX(-100%); /* 完全滑出屏幕左边 */
    overflow: hidden;
    padding: 0;
    border-right: none;
    /* 移除了visibility: hidden，让opacity和transform来处理隐藏效果 */
    pointer-events: none; /* 确保折叠时无法点击 */
    transition: all 0.4s cubic-bezier(0.4, 0, 0.2, 1); /* 折叠时过渡时间稍长 */
}

/* 悬浮按钮样式 - 调整得更协调 */
.float-collapse-btn {
    position: fixed;
    left: 16px;
    top: 80px;
    background: var(--bg-sidebar, #ffffff);
    border: 1px solid var(--border-color, #d0d7de);
    width: 36px;
    height: 36px;
    border-radius: 2px;
    color: var(--text-secondary, #57606a);
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
    transition: all 0.3s ease;
    opacity: 0.9;
}

.float-collapse-btn:hover {
    background: var(--hover-bg, #f3f4f6);
    color: var(--text-primary, #24292f);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
    opacity: 1;
}

.float-collapse-btn:active {
}

.float-collapse-btn i {
    font-size: 1.3rem;
    transition: transform 0.3s ease;
}

/* 悬浮按钮的动画效果 */
.float-collapse-btn {
    animation: floatIn 0.3s ease-out;
}

@keyframes floatIn {
    from {
        opacity: 0;
        transform: translateX(-10px);
    }
    to {
        opacity: 0.9;
        transform: translateX(0);
    }
}

/* Logo 区域样式 */
.logo-area {
    padding: 0 20px 20px;
    border-bottom: 1px solid var(--border-color, #d0d7de);
    margin-bottom: 20px;
    position: relative;
}

.collapse-btn {
    position: absolute;
    right: 12px;
    top: 12px;
    background: var(--hover-bg, #f3f4f6);
    border: none;
    width: 28px;
    height: 28px;
    border-radius: 2px;
    color: var(--text-secondary, #57606a);
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all 0.3s ease;
}

.collapse-btn:hover {
    background: var(--hover-bg, #f3f4f6);
    color: var(--text-primary, #24292f);
    transform: rotate(15deg);
}

.collapse-btn i {
    font-size: 1.2rem;
    transition: transform 0.5s cubic-bezier(0.4, 0, 0.2, 1); /* 更平滑的旋转动画 */
}

/* 折叠状态下侧边栏内按钮旋转 */
.sidebar:not(.collapsed) .collapse-btn i {
    transform: rotate(0deg);
}

.sidebar.collapsed .collapse-btn i {
    transform: rotate(180deg);
}

.logo-area h2 {
    margin: 0;
    color: var(--text-primary, #24292f);
    font-size: 1.25rem;
    font-weight: 600;
    display: flex;
    align-items: center;
    gap: 10px;
    margin-bottom: 8px;
}

.logo-area h2 i {
    font-size: 1.5rem;
    color: var(--accent-blue, #0969da);
}

.subtitle {
    margin: 0;
    color: var(--text-muted, #8c959f);
    font-size: 0.875rem;
    line-height: 1.4;
}

/* 用量进度条样式 */
.storage-usage {
    padding: 8px 0;
}

.usage-label {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 8px;
    font-size: 12px;
    color: var(--text-muted, #8c959f);
}

.usage-text {
    color: var(--accent-blue, #0969da);
    font-weight: 500;
}

.usage-bar {
    height: 6px;
    background: var(--bg-tertiary, #f6f8fa);
    border-radius: 2px;
    overflow: hidden;
}

.usage-progress {
    height: 100%;
    border-radius: 2px;
    transition: width 0.3s ease;
}

.usage-progress.unlimited {
    width: 100%;
    background: var(--accent-green, #2da44e);
}

/* 菜单通用样式 */
.main-menu {
    padding: 0 20px;
    margin-bottom: 24px;
}

/* 菜单分组样式 */
.menu-section {
    margin-bottom: 16px;
}

.menu-section-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 8px 12px;
    border-radius: 2px;
    cursor: pointer;
    transition: all 0.2s ease;
    margin-bottom: 4px;
}

.menu-section-header:hover {
    background-color: var(--hover-bg, #f3f4f6);
}

.menu-section-header.disabled {
    opacity: 0.5;
    cursor: not-allowed;
}

.menu-section-title {
    display: flex;
    align-items: center;
    gap: 10px;
    color: var(--text-primary, #24292f);
    font-size: 0.9375rem;
    font-weight: 500;
}

.menu-section-title i {
    font-size: 1.125rem;
    color: var(--text-secondary, #57606a);
}

.expand-icon {
    font-size: 1.125rem;
    color: var(--text-muted, #8c959f);
    transition: transform 0.2s ease;
}

.menu-list {
    list-style: none;
    padding: 0 0 0 12px;
    margin: 4px 0 0 0;
}

.menu-item {
    margin-bottom: 4px;
}

.menu-link {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 8px 12px;
    color: var(--text-secondary, #57606a);
    text-decoration: none;
    border-radius: 2px;
    transition: all 0.2s ease;
    font-size: 0.875rem;
    position: relative;
    cursor: pointer;
}

.menu-link i {
    font-size: 1rem;
    width: 20px;
    display: flex;
    justify-content: center;
}

.menu-link:hover:not(.disabled) {
    background-color: var(--hover-bg, #f3f4f6);
    color: var(--text-primary, #24292f);
}

.menu-link.router-link-active:not(.disabled) {
    background-color: var(--selected-bg, #ddf4ff);
    color: var(--accent-blue, #0969da);
    font-weight: 500;
}

.menu-link.router-link-active:not(.disabled) i {
    color: var(--accent-blue, #0969da);
}

/* 禁用状态 */
.menu-link.disabled {
    opacity: 0.5;
    cursor: not-allowed;
    pointer-events: none;
}

/* 响应式设计 - 小屏幕时可能需要调整 */
/* 现在折叠功能能用了，但手机端可能还需要调整 */
@media (max-width: 1024px) {
    .sidebar {
        width: 200px;
        /* 稍微窄一点 */
    }
}

/* 超小屏幕 - 可能需要完全不同的布局 */
/* TODO: 在手机上侧边栏可能应该变成底部导航或者可滑出的抽屉 */
@media (max-width: 768px) {
    .sidebar {
        width: 200px;
        /* 手机端稍微窄一点 */
    }

    .float-collapse-btn {
        top: 70px; /* 手机端调整悬浮按钮位置 */
        left: 8px;
        width: 36px;
        height: 36px;
    }
}
</style>