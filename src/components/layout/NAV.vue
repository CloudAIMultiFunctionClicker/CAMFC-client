<template>
    <div class="header">
        <h1>最简陋的多路由网页</h1>
        <nav>
            <!-- 使用router-link进行导航 -->
            <router-link to="/">首页</router-link> | 
            <router-link to="/about">关于</router-link> | 
            <router-link to="/contact">联系</router-link>
        </nav>
    </div>
</template>

<style scoped>
.header {
    background-color: #f0f0f0;
    padding: 20px;
    text-align: center;
}

nav {
    margin-top: 10px;
}

nav a {
    margin: 0 10px;
    text-decoration: none;
    color: #333;
}

nav a.router-link-active {
    font-weight: bold;
    color: #007bff;
}
</style>
