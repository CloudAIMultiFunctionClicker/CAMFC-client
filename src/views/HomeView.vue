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
  // 导入应用头部组件
  import AppHeader from '../components/layout/AppHeader.vue'
  // 这个主页视图很简单，就是显示头部和一个内容区域
  // TODO: 未来可能需要在这里添加更多内容，比如文件列表等
</script>

<template>
  <!-- 主页布局 -->
  <AppHeader/>
  <!-- 主要内容区域 - 目前是空的，只是占位 -->
  <div class="content-area"></div>
</template>

<script setup>
</script>


<style scoped>
  /* 内容区域样式 */
  .content-area {
    width: 100%;
    height: 100%;
    background: #1a1a1a; /* 深色背景，与头部协调 */
  }
</style>
