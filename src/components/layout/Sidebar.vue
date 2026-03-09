<!--
Copyright (C) 2026 Jiale Xu (许嘉乐) (ANTmmmmm) <https://github.com/ant-cave>
Email: ANTmmmmm@outlook.com, ANTmmmmm@126.com, 1504596931@qq.com

Copyright (C) 2026 Xinhang Chen (陈欣航) <https://github.com/cxh09>
Email: abc.cxh2009@foxmail.com

Copyright (C) 2026 Zimo Wen (温子墨) <https://github.com/lusamaqq>
Email: 1220594170@qq.com

Copyright (C) 2026 Kaibin Zeng (曾楷彬) <https://github.com/Waple1145>
Email: admin@mc666.top

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
-->

<!--
左侧边栏组件 - 用于主要导航
现在加上新的折叠功能：完全消失 + 悬浮按钮！
TODO: 动态菜单、路由高亮这些以后再加
FIXME: 悬浮按钮的样式还可以再优化，让它更融入整体设计
-->

<script setup>
// 导入Vue的响应式功能
import { ref } from "vue";

// 侧边栏折叠状态 - 默认展开
const isCollapsed = ref(false);

// 定义emit函数，用于触发事件
const emit = defineEmits(["collapse-change"]);

// 切换折叠状态的函数
const toggleCollapse = () => {
    isCollapsed.value = !isCollapsed.value;
    // 触发事件，通知父组件状态变化
    emit("collapse-change", isCollapsed.value);
};
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
        <!-- Logo区域 - 简单放个标题 -->
        <div class="logo-area">
            <h2>
                <i class="ri-folder-line"></i>
                <!-- 文件夹图标，跟云存储主题相关 -->
                <span>云盘</span>
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

            <!-- 折叠按钮 - 放在logo区域右上角 -->
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
            <!-- 导航标题 -->
            <h3 class="menu-title">
                <i class="ri-cloud-line"></i>
                云盘
            </h3>

            <!-- 导航链接列表 -->
            <ul class="menu-list">
                <!-- 文件 -->
                <li class="menu-item">
                    <router-link to="/fileView" class="menu-link">
                        <i class="ri-folder-line"></i>
                        <span>文件</span>
                    </router-link>
                </li>
                
                <!-- 传输 -->
                <li class="menu-item">
                    <router-link to="/transfer" class="menu-link">
                        <i class="ri-exchange-line"></i>
                        <span>传输</span>
                    </router-link>
                </li>
            </ul>
        </nav>

        <!-- 底部区域 -->
        <div class="sidebar-footer">
            <!-- 移除底部的折叠按钮，因为现在有悬浮按钮了 -->
            <!-- 简单放几个底部按钮 -->
            <button class="footer-btn">
                <i class="ri-settings-3-line"></i>
                <span>设置</span>
            </button>

            <button class="footer-btn">
                <i class="ri-question-line"></i>
                <span>帮助</span>
            </button>

            <!-- TODO: 这里以后可以放用户信息 -->
        </div>
    </aside>
</template>

<style scoped>
.sidebar {
    width: 240px;
    height: calc(100vh - 65px);
    background: #fff;
    border-right: 1px solid #eee;
    display: flex;
    flex-direction: column;
    padding: 20px 0;
    box-sizing: border-box;
    position: relative;
    z-index: 900;
    overflow-y: auto;
}

.sidebar::-webkit-scrollbar {
    width: 6px;
}

.sidebar::-webkit-scrollbar-track {
    background: transparent;
}

.sidebar::-webkit-scrollbar-thumb {
    background: #ddd;
    border-radius: 3px;
}

.sidebar::-webkit-scrollbar-thumb:hover {
    background: #999;
}

.sidebar.collapsed {
    width: 240px;
    opacity: 0;
    transform: translateX(-100%);
    overflow: hidden;
    padding: 0;
    border-right: none;
    pointer-events: none;
}

.float-collapse-btn {
    position: fixed;
    left: 16px;
    top: 80px;
    background: #fff;
    border: 1px solid #ddd;
    width: 36px;
    height: 36px;
    border-radius: 4px;
    color: #666;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
}

.float-collapse-btn:hover {
    background: #333;
    color: #fff;
}

.float-collapse-btn i {
    font-size: 1.3rem;
}

.logo-area {
    padding: 0 20px 20px;
    border-bottom: 1px solid #eee;
    margin-bottom: 20px;
    position: relative;
}

.collapse-btn {
    position: absolute;
    right: 12px;
    top: 12px;
    background: #f5f5f5;
    border: none;
    width: 28px;
    height: 28px;
    border-radius: 4px;
    color: #666;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
}

.collapse-btn:hover {
    background: #333;
    color: #fff;
}

.collapse-btn i {
    font-size: 1.2rem;
}

.sidebar:not(.collapsed) .collapse-btn i {
    transform: rotate(0deg);
}

.sidebar.collapsed .collapse-btn i {
    transform: rotate(180deg);
}

.logo-area h2 {
    margin: 0;
    color: #333;
    font-size: 1.25rem;
    font-weight: 600;
    display: flex;
    align-items: center;
    gap: 10px;
    margin-bottom: 8px;
}

.logo-area h2 i {
    font-size: 1.5rem;
    color: #333;
}

.subtitle {
    margin: 0;
    color: #999;
    font-size: 0.875rem;
    line-height: 1.4;
}

.storage-usage {
    padding: 8px 0;
}

.usage-label {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 8px;
    font-size: 12px;
    color: #999;
}

.usage-text {
    color: #333;
    font-weight: 500;
}

.usage-bar {
    height: 6px;
    background: #f5f5f5;
    border-radius: 3px;
    overflow: hidden;
}

.usage-progress {
    height: 100%;
    border-radius: 3px;
}

.usage-progress.unlimited {
    width: 100%;
    background: #333;
}

.main-menu {
    padding: 0 20px;
    margin-bottom: 24px;
}

.menu-title {
    margin: 0 0 12px 0;
    color: #666;
    font-size: 0.875rem;
    font-weight: 500;
    letter-spacing: 0.05em;
    display: flex;
    align-items: center;
    gap: 8px;
}

.menu-title i {
    font-size: 1rem;
    opacity: 0.7;
}

.menu-list {
    list-style: none;
    padding: 0;
    margin: 0;
}

.menu-item {
    margin-bottom: 4px;
}

.menu-link {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 10px 12px;
    color: #666;
    text-decoration: none;
    border-radius: 4px;
    font-size: 0.9375rem;
}

.menu-link i {
    font-size: 1.125rem;
    width: 24px;
    display: flex;
    justify-content: center;
}

.menu-link:hover {
    background-color: #f5f5f5;
    color: #333;
}

.menu-link.router-link-active {
    background-color: #f5f5f5;
    color: #333;
    font-weight: 500;
}

.menu-link.router-link-active i {
    color: #333;
}

.sidebar-footer {
    margin-top: auto;
    padding: 20px 20px 0;
    border-top: 1px solid #eee;
}

.footer-btn {
    display: flex;
    align-items: center;
    gap: 10px;
    width: 100%;
    padding: 10px 12px;
    background: none;
    border: none;
    color: #999;
    text-align: left;
    border-radius: 4px;
    cursor: pointer;
    font-size: 0.9375rem;
    margin-bottom: 8px;
}

.footer-btn i {
    font-size: 1.125rem;
}

.footer-btn:hover {
    background-color: #f5f5f5;
    color: #333;
}

@media (max-width: 1024px) {
    .sidebar {
        width: 200px;
    }
}

@media (max-width: 768px) {
    .sidebar {
        width: 200px;
    }

    .float-collapse-btn {
        top: 70px;
        left: 8px;
        width: 36px;
        height: 36px;
    }
}
</style>
