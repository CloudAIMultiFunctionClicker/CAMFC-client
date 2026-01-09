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
