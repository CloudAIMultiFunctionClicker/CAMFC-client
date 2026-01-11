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

<script setup>
import { inject } from 'vue'

// 头部组件 - 现在加了主题切换功能
// 之前试过加点击事件，但好像会跟路由冲突？先放着不管
// FIXME: 云按钮点了没反应，得找时间加上去
// TODO: 按钮的状态管理还没做，比如上传中的loading状态

// 从App.vue注入的主题功能
const theme = inject('theme')



setInterval(() => {
    console.log(theme?.isLightMode.value)
}, 2000);
</script>


<template>
    <!-- 顶部工具栏容器 -->
    <header class="header">
        <div class="toolbar">
            <!-- 左侧：应用标题和云按钮 -->
            <h1>
                <span>CAMFC Cloud</span>
                <!-- 云按钮 - 现在使用 Remix Icon 云图标 -->
                <button class="btn-cloud">
                    <i class="ri-cloud-line"></i>
                </button>
            </h1>

            <!-- 右侧：操作按钮区域 -->
            <div class="operation">
                <!-- 主题切换按钮 -->
                <button class="btn-theme" @click="theme?.toggleTheme">
                    <!-- 亮色模式时显示月亮图标（切换到暗色），暗色模式时显示太阳图标（切换到亮色） -->
                    <i class="ri-moon-line" v-if="theme?.isLightMode.value"></i>
                    <i class="ri-sun-line" v-else></i>
                    <!-- 小屏幕时隐藏文字 -->
                    <span class="btn-text">{{ theme?.isLightMode.value ? '切换到暗色' : '切换到亮色' }}</span>
                </button>
                
                <!-- 下拉菜单按钮 -->
                <button class="btn-dropdown">
                    <i class="ri-list-view"></i>
                    <span class="btn-text">列表视图</span>
                    <i class="ri-arrow-down-s-line"></i>
                </button>
                <!-- 上传按钮 -->
                <button class="btn-upload">
                    <i class="ri-upload-cloud-line"></i>
                    <span class="btn-text">上传</span>
                </button>
                <!-- 分享按钮 -->
                <button class="btn-share">
                    <i class="ri-share-forward-line"></i>
                    <span class="btn-text">分享</span>
                </button>
                <!-- 删除按钮 -->
                <button class="btn-delete">
                    <i class="ri-delete-bin-line"></i>
                    <span class="btn-text">删除</span>
                </button>
                <!-- 用户头像按钮 -->
                <button class="btn-avatar">
                    <i class="ri-user-line"></i>
                </button>
            </div>
        </div>
    </header>
</template>


<style scoped>
/* 头部样式 - 现在支持主题切换了 */
/* 之前用纯黑色太压抑了，试了几个渐变，这个看起来还行 */
/* TODO: 亮色模式的阴影可能需要调整，现在看起来还行 */

header {
    width: 100%;
    height: 64px;
    border-bottom: 1px solid var(--border-color, rgba(255, 255, 255, 0.1));
    /* 使用主题边框色 */
    background: var(--bg-header, linear-gradient(135deg, #0f172a 0%, #1e293b 100%));
    /* 使用主题头部背景 */
    position: relative;
    z-index: 1000;
    /* 确保在最上面 */
    transition: background 0.3s ease, border-color 0.3s ease;
    /* 主题切换过渡效果 */
}

/* 工具栏布局 */
.toolbar {
    display: flex;
    justify-content: space-between;
    width: 100%;
    height: 100%;
    align-items: center;
    /* 两边留点空间 */
}
.toolbar>*:first-child {
    margin-left: 24px;
}
.toolbar>*:last-child {
    margin-right: 24px;

}

h1 {
    margin: 0;
    display: flex;
    align-items: center;
    gap: 12px;
    /* flex gap真好用 */
    color: var(--text-primary, #f8fafc);
    /* 使用主题主要文字色 */
    font-size: 1.5rem;
    font-weight: 600;
    letter-spacing: -0.025em;
    /* 字距收紧一点感觉更现代？ */
    transition: color 0.3s ease;
    /* 文字颜色过渡 */
}

/* 右侧按钮区域 */
.operation {
    display: flex;
    align-items: center;
    gap: 12px;
    /* 用gap代替margin-left/margin-right */
    flex-wrap: nowrap;
    /* 不换行 */
}

/* 按钮基础样式 - 统一一下 */
.btn-cloud,
.btn-theme,
.btn-dropdown,
.btn-upload,
.btn-share,
.btn-delete,
.btn-avatar {
    border: none;
    border-radius: 8px;
    /* 圆角大一点现代感强 */
    padding: 8px 16px;
    font-size: 14px;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    /* 图标和文字的间距 */
    font-weight: 500;
    transition: all 0.2s ease;
    /* 过渡效果，hover用 */
    height: 40px;
    /* 统一高度 */
}

/* 主题切换按钮 - 放在第一个 */
.btn-theme {
    background-color: var(--hover-bg, rgba(255, 255, 255, 0.08));
    color: var(--text-secondary, #cbd5e1);
    border: 1px solid var(--border-color, rgba(255, 255, 255, 0.1));
}

.btn-theme:hover {
    background-color: var(--accent-blue, #3b82f6);
    color: white;
    border-color: var(--accent-blue, #3b82f6);
}

/* 云按钮 - 就是个装饰性的 */
.btn-cloud {
    background: var(--hover-bg, rgba(255, 255, 255, 0.08));
    /* 使用主题hover背景色 */
    color: var(--text-muted, #94a3b8);
    /* 使用主题次要文字色 */
    padding: 8px;
    border-radius: 50%;
    /* 圆形 */
    width: 40px;
    height: 40px;
}

/* 下拉按钮 - 中性色 */
.btn-dropdown {
    background-color: var(--hover-bg, rgba(255, 255, 255, 0.08));
    /* 使用主题hover背景色 */
    color: var(--text-secondary, #cbd5e1);
    /* 使用主题次要文字色 */
    border: 1px solid var(--border-color, rgba(255, 255, 255, 0.1));
}

/* 上传按钮 - 主操作按钮，突出显示 */
.btn-upload {
    background: linear-gradient(135deg, var(--accent-blue, #3b82f6) 0%, #1d4ed8 100%);
    /* 使用主题蓝色 */
    color: white;
    border: none;
    box-shadow: 0 2px 10px rgba(var(--accent-blue-rgb, 59, 130, 246), 0.3);
    /* 使用主题蓝色发光 */
}

/* 分享按钮 - 深蓝色 */
.btn-share {
    background-color: rgba(var(--accent-blue-rgb, 59, 130, 246), 0.2);
    /* 使用主题蓝色，半透明 */
    color: white;
    border: 1px solid rgba(var(--accent-blue-rgb, 59, 130, 246), 0.3);
}

/* 删除按钮 - 红色警告色 */
.btn-delete {
    background-color: rgba(var(--accent-red-rgb, 220, 53, 69), 0.8);
    /* 使用主题红色，半透明 */
    color: white;
    border: 1px solid rgba(var(--accent-red-rgb, 220, 53, 69), 0.3);
}

/* 头像按钮 - 圆形 */
.btn-avatar {
    background-color: rgba(var(--accent-blue-rgb, 59, 130, 246), 0.1);
    border: 2px solid rgba(var(--accent-blue-rgb, 59, 130, 246), 0.5);
    /* 蓝色边框 */
    border-radius: 50%;
    width: 40px;
    height: 40px;
    color: var(--accent-blue, #3b82f6);
    /* 蓝色图标 */
    padding: 0;
}

/* ====== HOVER 效果 ====== */

.btn-cloud:hover {
    background: var(--accent-blue, #3b82f6);
    /* hover时用主题蓝色 */
    color: white;
}

.btn-dropdown:hover {
    background-color: var(--accent-blue, #3b82f6);
    /* hover时用主题蓝色 */
    color: white;
    border-color: var(--accent-blue, #3b82f6);
}

/* 上传按钮hover - 让它亮一点 */
.btn-upload:hover {
    background: linear-gradient(135deg, #4a94ff 0%, #2563eb 100%);
    box-shadow: 0 4px 15px rgba(var(--accent-blue-rgb, 59, 130, 246), 0.4);
    /* hover时阴影强一点 */
}

.btn-share:hover {
    background-color: rgba(var(--accent-blue-rgb, 59, 130, 246), 0.3);
    /* hover时更不透明 */
    border-color: rgba(var(--accent-blue-rgb, 59, 130, 246), 0.5);
}

.btn-delete:hover {
    background-color: rgba(var(--accent-red-rgb, 220, 53, 69), 0.95);
    border-color: rgba(var(--accent-red-rgb, 220, 53, 69), 0.5);
}

.btn-avatar:hover {
    background-color: rgba(var(--accent-blue-rgb, 59, 130, 246), 0.2);
    border-color: var(--accent-blue, #3b82f6);
    color: #60a5fa;
    /* 亮一点的蓝 */
}

/* 图标统一样式 */
.btn-cloud i,
.btn-dropdown i,
.btn-upload i,
.btn-share i,
.btn-delete i,
.btn-avatar i,
.btn-theme i {
    font-size: 16px;
    display: flex;
    align-items: center;
    justify-content: center;
}

/* 按钮文字通用类 - 方便响应式隐藏 */
.btn-text {
    display: inline;
}

/* 响应式 - 小屏幕时按钮只显示图标 */
/* FIXME: 在小屏幕上图标按钮有点挤，可能需要调整 */
@media (max-width: 1024px) {
    .toolbar {
        padding: 0 16px;
        /* 内边距减小 */
    }

    .operation {
        gap: 8px;
        /* 间距减小 */
    }

    /* 隐藏按钮文字，只留图标 */
    .btn-text {
        display: none;
    }

    .btn-theme,
    .btn-dropdown,
    .btn-upload,
    .btn-share,
    .btn-delete {
        padding: 8px;
        width: 40px;
        /* 固定宽度 */
        justify-content: center;
    }

    /* TODO: 超小屏幕可能需要更多调整 */
}
</style>