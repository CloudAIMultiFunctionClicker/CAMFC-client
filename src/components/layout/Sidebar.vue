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
先做个简单版本，跟头部样式协调就行
TODO: 以后可能需要加上折叠功能、动态菜单啥的
FIXME: 现在路由高亮还没做，得等加上script再说
-->

<template>
    <!-- 侧边栏容器 -->
    <aside class="sidebar">
        <!-- Logo区域 - 简单放个标题 -->
        <div class="logo-area">
            <h2>
                <i class="ri-folder-line"></i>
                <!-- 文件夹图标，跟云存储主题相关 -->
                <span>文件管理</span>
            </h2>
            <!-- 小提示文字 -->
            <p class="subtitle">CAMFC Cloud侧边导航</p>
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
    height: 100vh;
    /* 全高 */
    background: var(--bg-sidebar, linear-gradient(135deg, #1e293b 0%, #334155 100%));
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
    transition: background 0.3s ease, border-color 0.3s ease;
    /* 主题切换时的过渡效果 */
}

/* Logo区域样式 */
.logo-area {
    padding: 0 20px 20px;
    /* 左右内边距，底部有间距 */
    border-bottom: 1px solid var(--border-color, rgba(255, 255, 255, 0.05));
    margin-bottom: 20px;
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
/* FIXME: 在小屏幕上侧边栏可能太宽，需要考虑折叠或隐藏 */
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
        width: 64px;
        /* 只显示图标 */
        padding: 20px 0;
    }

    .logo-area h2 span,
    .subtitle,
    .menu-title span,
    .menu-link span,
    .footer-btn span {
        display: none;
        /* 隐藏所有文字 */
    }

    .logo-area,
    .main-menu,
    .category-menu,
    .sidebar-footer {
        padding: 0 12px;
        /* 减少内边距 */
    }

    .logo-area h2,
    .menu-title,
    .menu-link,
    .footer-btn {
        justify-content: center;
        /* 居中图标 */
    }

    .menu-link i,
    .footer-btn i {
        width: auto;
        /* 取消固定宽度 */
    }
}
</style>    .footer-btn i {
