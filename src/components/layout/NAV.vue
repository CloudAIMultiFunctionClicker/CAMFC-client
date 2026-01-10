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

<template>
    <!-- 导航栏组件 - 用于路由切换 -->
    <div class="header">
        <h1>最简陋的多路由网页</h1>
        <!-- 导航菜单 -->
        <nav>
            <!-- 使用router-link进行导航 -->
            <router-link to="/">首页</router-link> | 
            <router-link to="/about">关于</router-link> | 
            <router-link to="/contact">联系</router-link>
        </nav>
    </div>
</template>

<style scoped>
/* 导航栏容器样式 */
.header {
    background-color: #f0f0f0; /* 浅灰色背景 */
    padding: 20px; /* 内边距 */
    text-align: center; /* 居中文本 */
}

/* 导航菜单样式 */
nav {
    margin-top: 10px; /* 与标题的间距 */
}

/* 导航链接基础样式 */
nav a {
    margin: 0 10px; /* 左右边距 */
    text-decoration: none; /* 去除下划线 */
    color: #333; /* 深灰色文字 */
}

/* 激活状态的路由链接样式 */
nav a.router-link-active {
    font-weight: bold; /* 加粗 */
    color: #007bff; /* 蓝色主题色 */
}
</style>
