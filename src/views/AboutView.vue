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
  <!-- 关于页面 - 演示用路由页面 -->
  <div class="about">
    <h2>关于页面</h2>
    <p>这个页面用来演示多路由功能。</p>
    <p>实际上没什么内容，就是证明路由能工作。</p>
    <!-- TODO: 未来可以在这里添加项目介绍、团队信息等 -->
  </div>
</template>

<script>
// 关于页面的组件定义
// 目前只有最基本的展示功能
export default {
  name: 'AboutView'
}
</script>

<style scoped>
/* 关于页面容器样式 */
.about {
  padding: 20px; /* 内边距，让内容不要紧贴边缘 */
}
</style>
