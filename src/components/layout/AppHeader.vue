<script setup>

</script>


<template>

    <header class="header">
        <div class="toolbar">
            <h1>
                <span>CAMFC Cloud</span>
                <button class="btn-cloud">

                </button>
            </h1>
            <div class="operation">
                <button class="btn-dropdown">
                    列表视图 ▼
                </button>
                <button class="btn-upload">
                    ➕ 上传
                </button>
                <button class="btn-share">
                    📤 分享
                </button>
                <button class="btn-delete">
                    🗑️ 删除
                </button>
                <button class="btn-avatar">
                    👤
                </button>
            </div>
        </div>
    </header>


</template>


<style scoped>
@import 'src/assets/css/main.css';

h1 {
    padding: 0px;
    margin-left: 20px;
    display: inline;
    display: flex;
    align-items: center;
    justify-content: left;
    min-width: 0px;
    width: 100%;
    
    color:white
}

.operation {
    width: 100%;
    display: flex;
    align-items: center;
    flex-wrap: nowrap; /* 防止换行 */
    overflow-x: auto; /* 允许水平滚动而不是隐藏 */
    overflow-y: hidden;
    min-width: 0; /* 允许flex容器缩小 */
    justify-content: right;
    margin-right: 20px;
}

.operation>*{
    margin-left:5px;
    margin-right:5px;
}


header {
    width: 100%;
    height: 64px;
    border-bottom: 1px solid #e0e0e6;

}

.toolbar {
    text-align: center;
    display: flex;
    justify-content: space-between;
    width: 100%;
    height: 100%;
    display: flex;
    align-items: center;
    background-color: #1e1e1eb3;
}

.btn-cloud {
    background: none;
    border: none;
    color: white;
    font-size: 14px;
    cursor: pointer;
    display: flex;
    align-items: center;
    gap: 6px;
}

.btn-dropdown {
    background-color: rgba(255,255,255,0.1);
    color: white;
    border: none;
    border-radius: 4px;
    padding: 6px 12px;
    font-size: 14px;
    cursor: pointer;
}

.btn-upload {
    background-color: #007BFF;
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

.btn-share {
    background-color: #0056b3;
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

.btn-delete {
    background-color: #DC3545;
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

.btn-avatar {
    background-color: transparent;
    border: 2px solid #007BFF;
    border-radius: 50%;
    width: 32px;
    height: 32px;
    color: white;
    font-size: 16px;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    background-color: white;
    flex-shrink: 0; /* 头像不缩小 */
    /* 移除margin: 5px; 因为.operation>*已经设置了左右边距 */
    position: relative; /* 确保按钮在滚动时正常显示 */
}
</style>