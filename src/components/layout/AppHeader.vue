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

<script setup>
import { inject, ref, onMounted, onUnmounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'

// 头部组件 - 现在加了主题切换功能
// 之前试过加点击事件，但好像会跟路由冲突？先放着不管
// FIXME: 云按钮点了没反应，得找时间加上去
// TODO: 按钮的状态管理还没做，比如上传中的loading状态

// 从App.vue注入的主题功能
const theme = inject('theme')

// 按钮状态
const buttonPressed = ref(false)

// 监听按钮状态变化
const handleButtonState = (event) => {
  buttonPressed.value = event.detail.pressed
}

onMounted(() => {
  window.addEventListener('button-state', handleButtonState)
})

onUnmounted(() => {
  window.removeEventListener('button-state', handleButtonState)
})

</script>


<template>
    <!-- 顶部工具栏容器 -->
    <header class="header">
        <div class="toolbar">
            <!-- 左侧：应用标题和云按钮 -->
            <h1>
                <span>CAMFC Cloud</span>
                <!-- 云按钮 - 现在使用 Remix Icon 云图标 -->
                <router-link to="/main">
                <button class="btn-cloud">
                    
                    <i class="ri-cloud-line"></i>
                </button></router-link>
            </h1>

            <!-- 右侧：操作按钮区域 -->
            <div class="operation">
                <!-- 按钮状态指示器 -->
                <div class="btn-button-state" :class="{ 'pressed': buttonPressed }">
                    <i class="ri-checkbox-circle-line" v-if="buttonPressed"></i>
                    <i class="ri-checkbox-blank-circle-line" v-else></i>
                    <span class="btn-text">{{ buttonPressed ? '按键: 按下' : '按键: 松开' }}</span>
                </div>
                
                <!-- 主题切换按钮 -->
                <button class="btn-theme" @click="theme?.toggleTheme">
                    <!-- 亮色模式时显示月亮图标（切换到暗色），暗色模式时显示太阳图标（切换到亮色） -->
                    <i class="ri-moon-line" v-if="theme?.isLightMode.value"></i>
                    <i class="ri-sun-line" v-else></i>
                    <!-- 小屏幕时隐藏文字 -->
                    <span class="btn-text">{{ theme?.isLightMode.value ? '切换到暗色' : '切换到亮色' }}</span>
                </button>
                
                <!-- 用户头像按钮 -->
                <!--现在的跳转 测试用-->
                <router-link to="/fileView">
                <button class="btn-avatar" >
                    
                    <i class="ri-user-line"></i>
                </button></router-link>
            </div>
        </div>
    </header>
</template>


<style scoped>
header {
    width: 100%;
    height: 64px;
    background: #fff;
    border-bottom: 1px solid #eee;
    position: sticky;
    top: 0;
    z-index: 1000;
}

.toolbar {
    display: flex;
    justify-content: space-between;
    width: 100%;
    height: 100%;
    align-items: center;
    max-width: 800px;
    margin: 0 auto;
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
    color: #333;
    font-size: 1.2rem;
    font-weight: 600;
}

.operation {
    display: flex;
    align-items: center;
    gap: 12px;
    flex-wrap: nowrap;
}

.btn-cloud,
.btn-theme,
.btn-dropdown,
.btn-upload,
.btn-share,
.btn-delete,
.btn-avatar,
.btn-test,
.btn-button-state {
    border: none;
    border-radius: 4px;
    padding: 8px 16px;
    font-size: 14px;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    font-weight: 500;
    height: 40px;
}

.btn-button-state {
    background-color: #fff;
    color: #666;
    border: 1px solid #ddd;
}

.btn-button-state.pressed {
    background-color: #333;
    color: white;
    border-color: #333;
}

.btn-button-state:hover {
    background-color: #333;
    color: white;
    border-color: #333;
}

.btn-theme {
    background-color: #fff;
    color: #666;
    border: 1px solid #ddd;
}

.btn-theme:hover {
    background-color: #333;
    color: white;
    border-color: #333;
}

.btn-cloud {
    background: #fff;
    color: #999;
    padding: 8px;
    border-radius: 4px;
    width: 40px;
    height: 40px;
    border: 1px solid #ddd;
}

a {
   text-decoration: none;
}
a:hover { 
    text-decoration: none;
}

.btn-dropdown {
    background-color: #fff;
    color: #666;
    border: 1px solid #ddd;
}

.btn-upload {
    background-color: #333;
    color: white;
    border: none;
}

.btn-share {
    background-color: #f5f5f5;
    color: #333;
    border: 1px solid #ddd;
}

.btn-delete {
    background-color: #fff;
    color: #c00;
    border: 1px solid #ddd;
}

.btn-avatar {
    background-color: #fff;
    border: 1px solid #ddd;
    border-radius: 4px;
    width: 40px;
    height: 40px;
    color: #333;
    padding: 0;
}

.btn-cloud:hover {
    background: #333;
    color: white;
    border-color: #333;
}

.btn-dropdown:hover {
    background-color: #333;
    color: white;
    border-color: #333;
}

.btn-upload:hover {
    background-color: #555;
}

.btn-share:hover {
    background-color: #333;
    color: white;
    border-color: #333;
}

.btn-delete:hover {
    background-color: #c00;
    color: white;
    border-color: #c00;
}

.btn-avatar:hover {
    background-color: #333;
    border-color: #333;
    color: white;
}

.btn-test:hover {
    background-color: #333;
    border-color: #333;
    color: white;
}

.btn-cloud i,
.btn-dropdown i,
.btn-upload i,
.btn-share i,
.btn-delete i,
.btn-avatar i,
.btn-theme i,
.btn-test i {
    font-size: 16px;
    display: flex;
    align-items: center;
    justify-content: center;
}

.btn-text {
    display: inline;
}

@media (max-width: 1024px) {
    .toolbar {
        padding: 0 16px;
    }

    .operation {
        gap: 8px;
    }

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
        justify-content: center;
    }
}
</style>
