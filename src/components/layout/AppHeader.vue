<script setup>
// 应用头部组件 - 用于展示顶部工具栏
// 这个组件没有复杂的逻辑，主要是UI展示
// TODO: 未来可能需要添加点击事件处理和数据绑定
</script>


<template>
    <!-- 顶部工具栏容器 -->
    <header class="header">
        <div class="toolbar">
            <!-- 左侧：应用标题和云按钮 -->
            <h1>
                <span>CAMFC Cloud</span>
                <!-- 云按钮 - 目前只是个占位符 -->
                <button class="btn-cloud">
                    <!-- 这里应该有个云图标，现在先空着 -->
                </button>
            </h1>
            
            <!-- 右侧：操作按钮区域 -->
            <div class="operation">
                <!-- 下拉菜单按钮 -->
                <button class="btn-dropdown">
                    列表视图 ▼
                </button>
                <!-- 上传按钮 -->
                <button class="btn-upload">
                    ➕ 上传
                </button>
                <!-- 分享按钮 -->
                <button class="btn-share">
                    📤 分享
                </button>
                <!-- 删除按钮 -->
                <button class="btn-delete">
                    🗑️ 删除
                </button>
                <!-- 用户头像按钮 -->
                <button class="btn-avatar">
                    👤
                </button>
            </div>
        </div>
    </header>
</template>


<style scoped>
/* 导入全局CSS文件 */
@import 'src/assets/css/main.css';

/* 应用标题样式 */
h1 {
    padding: 0px;
    margin-left: 20px;
    /* 原来有 display: inline; 和 display: flex; 重复了，只保留flex */
    display: flex;
    align-items: center;
    justify-content: left;
    min-width: 0px;
    width: 100%;
    color: white; /* 文字颜色 */
}

/* 操作按钮区域 - 右侧按钮容器 */
.operation {
    width: 100%;
    display: flex;
    align-items: center;
    flex-wrap: nowrap; /* 防止按钮换行 */
    overflow-x: auto; /* 如果按钮太多允许水平滚动 */
    overflow-y: hidden;
    min-width: 0; /* 允许flex容器缩小以适应空间 */
    justify-content: right; /* 右对齐 */
    margin-right: 20px; /* 右边距 */
}

/* 操作区域内的所有直接子元素 */
.operation>* {
    margin-left: 5px;
    margin-right: 5px;
}

/* 头部容器 */
header {
    width: 100%;
    height: 64px;
    border-bottom: 1px solid #e0e0e6; /* 底部边框 */
}

/* 工具栏 - 主容器 */
.toolbar {
    text-align: center;
    /* 原来有重复的 display: flex; 声明 */
    display: flex;
    justify-content: space-between; /* 左右分布 */
    width: 100%;
    height: 100%;
    align-items: center; /* 垂直居中 */
    background-color: #1e1e1eb3; /* 半透明深色背景 */
}

/* 云按钮样式 */
.btn-cloud {
    background: none;
    border: none;
    color: white;
    font-size: 14px;
    cursor: pointer;
    display: flex;
    align-items: center;
    gap: 6px; /* 图标和文字间距 */
}

/* 下拉菜单按钮 */
.btn-dropdown {
    background-color: rgba(255,255,255,0.1); /* 半透明白色 */
    color: white;
    border: none;
    border-radius: 4px;
    padding: 6px 12px;
    font-size: 14px;
    cursor: pointer;
}

/* 上传按钮 - 主操作按钮 */
.btn-upload {
    background-color: #007BFF; /* 蓝色主题色 */
    color: white;
    border: none;
    border-radius: 4px;
    padding: 6px 12px;
    font-size: 14px;
    cursor: pointer;
    display: flex;
    align-items: center;
    gap: 4px; /* 图标和文字间距 */
}

/* 分享按钮 */
.btn-share {
    background-color: #0056b3; /* 深蓝色 */
    color: white;
    border: none;
    border-radius: 4px;
    padding: 6px 12px;
    font-size: 14px;
    cursor: pointer;
    display: flex;
    align-items: center;
    gap: 4px;
}

/* 删除按钮 */
.btn-delete {
    background-color: #DC3545; /* 红色 */
    color: white;
    border: none;
    border-radius: 4px;
    padding: 6px 12px;
    font-size: 14px;
    cursor: pointer;
    display: flex;
    align-items: center;
    gap: 4px;
}

/* 用户头像按钮 */
.btn-avatar {
    background-color: transparent;
    border: 2px solid #007BFF; /* 蓝色边框 */
    border-radius: 50%; /* 圆形 */
    width: 32px;
    height: 32px;
    color: white;
    font-size: 16px;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    background-color: white; /* 白色背景，与文字颜色冲突？TODO: 检查这个设计 */
    flex-shrink: 0; /* 防止头像按钮被压缩 */
    position: relative; /* 为可能的定位需求做准备 */
}
</style>
