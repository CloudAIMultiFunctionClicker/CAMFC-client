<script setup lang="ts">
/*
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
*/

import { onMounted } from 'vue'
import AppHeader from '@/components/layout/AppHeader.vue'
import Sidebar from '@/components/layout/Sidebar.vue'
import FileGrid from '@/components/file/FileGrid.vue'
import UploadZone from '@/components/file/UploadZone.vue'
import { useFileStore } from '@/stores/useFileStore'

const fileStore = useFileStore()

onMounted(() => {
  // 初始化加载数据
  fileStore.init()
})
</script>

<template>
  <div class="home-view">
    <AppHeader></AppHeader>
    
    <div class="main-content">
      <Sidebar></Sidebar>
      
      <div class="content-area">
        <FileGrid></FileGrid>
      </div>
    </div>
    
    <UploadZone></UploadZone>
  </div>
</template>

<style scoped>
.home-view {
  display: flex;
  flex-direction: column;
  height: 100vh;
  overflow: hidden;
}

.main-content {
  display: flex;
  flex: 1;
  overflow: hidden;
}

.content-area {
  flex: 1;
  padding: 20px;
  overflow-y: auto;
  background-color: var(--n-color-body);
}

/* 响应式设计：小屏幕时隐藏侧边栏 */
@media (max-width: 768px) {
  .main-content {
    flex-direction: column;
  }
  
  .content-area {
    padding: 12px;
  }
}
</style>
