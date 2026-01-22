<!--
Copyright (C) 2026 Jiale Xu (许嘉乐) (ANTmmmmm) <https://github.com/ant-cave>
Email: ANTmmmmm@outlook.com, ANTmmmmm@126.com, 1504596931@qq.com

Copyright (C) 2026 Xinhang Chen (陈欣航) <https://github.com/cxh09>
Email: abc.cxh2009@foxmail.com

Copyright (C) 2026 Zimo Wen (温子墨) <https://github.com/lusamaqq>
Email: 1220594170@qq.com

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
                <span>文件管理</span>
            </h2>
            <!-- 小提示文字 -->
            <p class="subtitle">CAMFC Cloud侧边导航</p>

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
                <i class="ri-navigation-line"></i>
                导航
            </h3>

            <!-- 导航链接列表 -->
            <ul class="menu-list">
                <!-- 首页链接 -->
                <li class="menu-item">
                    <router-link to="/" class="menu-link">
                        <i class="ri-home-line"></i>
                        <span>首页</span>
                    </router-link>
                </li>

                <!-- 关于页面链接 -->
                <li class="menu-item">
                    <router-link to="/about" class="menu-link">
                        <i class="ri-information-line"></i>
                        <span>关于</span>
                    </router-link>
                </li>

                <!-- 联系页面链接 -->
                <li class="menu-item">
                    <router-link to="/contact" class="menu-link">
                        <i class="ri-contacts-line"></i>
                        <span>联系</span>
                    </router-link>
                </li>
            </ul>
        </nav>

        <!-- 文件分类菜单 -->
        <nav class="category-menu">
            <h3 class="menu-title">
                <i class="ri-folder-open-line"></i>
                文件分类
            </h3>

            <ul class="menu-list">
                <!-- 几个主要的文件分类 -->
                <li class="menu-item">
                    <a href="#" class="menu-link">
                        <i class="ri-file-text-line"></i>
                        <span>文档</span>
                        <!-- TODO: 以后这里可以加个文件数量提示 -->
                    </a>
                </li>

                <li class="menu-item">
                    <a href="#" class="menu-link">
                        <i class="ri-image-line"></i>
                        <span>图片</span>
                    </a>
                </li>

                <li class="menu-item">
                    <a href="#" class="menu-link">
                        <i class="ri-music-line"></i>
                        <span>音乐</span>
                    </a>
                </li>

                <li class="menu-item">
                    <a href="#" class="menu-link">
                        <i class="ri-video-line"></i>
                        <span>视频</span>
                    </a>
                </li>

                <li class="menu-item">
                    <a href="#" class="menu-link">
                        <i class="ri-archive-line"></i>
                        <span>压缩包</span>
                    </a>
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
/* 侧边栏基础样式 - 使用CSS变量支持主题切换 */
/* 现在颜色都从全局变量获取，亮色/暗色模式自动切换 */

.sidebar {
    width: 240px;
    height: calc(100vh - 65px);
    /* 全高 */
    background: var(
        --bg-sidebar,
        linear-gradient(135deg, #1e293b 0%, #334155 100%)
    );
    border-right: 1px solid var(--border-color, rgba(255, 255, 255, 0.05));
    /* 主题化边框 */
    display: flex;
    flex-direction: column;
    /* 垂直布局 */
    padding: 20px 0;
    /* 上下有内边距 */
    box-sizing: border-box;
    position: relative;
    z-index: 900;
    /* 在头部下面 */
    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    /* 使用贝塞尔曲线让动画更自然，包含所有属性的过渡 */
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
    top: 80px; /* 在头部下方一点 */
    background: var(--bg-sidebar, #1e293b);
    border: 1px solid var(--border-color, rgba(255, 255, 255, 0.1));
    width: 36px;
    height: 36px;
    border-radius: 8px; /* 改为圆角矩形，更符合设计 */
    color: var(--text-secondary, #cbd5e1);
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000; /* 确保在最上层 */
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.15);
    transition: all 0.3s ease;
    opacity: 0.9;
}

.float-collapse-btn:hover {
    background: var(--hover-bg, rgba(255, 255, 255, 0.1));
    color: var(--text-primary, #f8fafc);
    transform: translateY(-1px); /* 轻微上浮效果 */
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);
    opacity: 1;
}

.float-collapse-btn:active {
    transform: translateY(0); /* 点击时恢复位置 */
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

/* Logo区域样式 */
.logo-area {
    padding: 0 20px 20px;
    /* 左右内边距，底部有间距 */
    border-bottom: 1px solid var(--border-color, rgba(255, 255, 255, 0.05));
    margin-bottom: 20px;
    position: relative;
    /* 为折叠按钮定位 */
}

/* 折叠按钮样式（侧边栏内的） */
.collapse-btn {
    position: absolute;
    right: 12px;
    top: 12px;
    background: var(--hover-bg, rgba(255, 255, 255, 0.1));
    border: none;
    width: 28px;
    height: 28px;
    border-radius: 6px; /* 与悬浮按钮保持一致 */
    color: var(--text-secondary, #cbd5e1);
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all 0.3s ease;
}

.collapse-btn:hover {
    background: var(--hover-bg, rgba(255, 255, 255, 0.15));
    color: var(--text-primary, #f8fafc);
    transform: rotate(15deg); /* 轻微旋转效果 */
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
    color: var(--text-primary, #f8fafc);
    font-size: 1.25rem;
    font-weight: 600;
    display: flex;
    align-items: center;
    gap: 10px;
    margin-bottom: 8px;
}

.logo-area h2 i {
    font-size: 1.5rem;
    color: var(--accent-blue, #3b82f6);
    /* 使用主题蓝色 */
}

.subtitle {
    margin: 0;
    color: var(--text-muted, #94a3b8);
    font-size: 0.875rem;
    line-height: 1.4;
}

/* 菜单通用样式 */
.main-menu,
.category-menu {
    padding: 0 20px;
    margin-bottom: 24px;
    /* 菜单之间的间距 */
}

.menu-title {
    margin: 0 0 12px 0;
    color: var(--text-secondary, #cbd5e1);
    font-size: 0.875rem;
    font-weight: 500;
    text-transform: uppercase;
    /* 大写字母，看起来像标题 */
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
    /* 菜单项之间的间距 */
}

/* 菜单链接样式 */
.menu-link {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 10px 12px;
    color: var(--text-secondary, #cbd5e1);
    text-decoration: none;
    border-radius: 8px;
    /* 跟按钮一样的圆角 */
    transition: all 0.2s ease;
    /* 过渡效果 */
    font-size: 0.9375rem;
}

.menu-link i {
    font-size: 1.125rem;
    width: 24px;
    /* 固定图标宽度，对齐好看 */
    display: flex;
    justify-content: center;
}

.menu-link:hover {
    background-color: var(--hover-bg, rgba(255, 255, 255, 0.08));
    color: var(--text-primary, #f1f5f9);
}

/* 路由链接激活状态 - 先写个样式，等有script了再用 */
/* TODO: 这里需要router-link-active类来高亮当前页面 */
.menu-link.router-link-active {
    background-color: rgba(var(--accent-blue-rgb, 59, 130, 246), 0.15);
    color: var(--accent-blue, #3b82f6);
    font-weight: 500;
}

.menu-link.router-link-active i {
    color: var(--accent-blue, #3b82f6);
}

/* 底部区域样式 */
.sidebar-footer {
    margin-top: auto;
    /* 推到最底部 */
    padding: 20px 20px 0;
    border-top: 1px solid var(--border-color, rgba(255, 255, 255, 0.05));
}

.footer-btn {
    display: flex;
    align-items: center;
    gap: 10px;
    width: 100%;
    padding: 10px 12px;
    background: none;
    border: none;
    color: var(--text-muted, #94a3b8);
    text-align: left;
    border-radius: 8px;
    cursor: pointer;
    transition: all 0.2s ease;
    font-size: 0.9375rem;
    margin-bottom: 8px;
    /* 按钮之间的间距 */
}

.footer-btn i {
    font-size: 1.125rem;
}

.footer-btn:hover {
    background-color: var(--hover-bg, rgba(255, 255, 255, 0.05));
    color: var(--text-secondary, #cbd5e1);
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
.footer-btn i {
