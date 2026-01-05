<script setup lang="ts">
/*
 * Copyright (C) 2026 Jiale Xu (ANTmmmmm) (ant-cave)
 * Email: ANTmmmmm@outlook.com, ANTmmmmm@126.com, 1504596931@qq.com
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

import { computed, h, ref } from 'vue'
import { useRouter } from 'vue-router'
import { 
  NMenu, 
  NLayoutSider, 
  NButton,
  NIcon,
  NDivider
} from 'naive-ui'
import {
  FolderOutline,
  FolderOpenOutline,
  TimeOutline,
  StarOutline,
  ShareSocialOutline,
  TrashBinOutline,
  CloudOutline,
  AddOutline
} from '@vicons/ionicons5'
import { useFileStore } from '@/stores/useFileStore'
import CreateFolderModal from '@/components/modals/CreateFolderModal.vue'

const router = useRouter()
const fileStore = useFileStore()

// 新建文件夹模态框显示状态
const showCreateFolderModal = ref(false)

// 侧边栏菜单项
const menuItems = computed(() => [
  {
    label: '全部文件',
    key: '/',
    icon: () => h(FolderOutline)
  },
  {
    label: '最近',
    key: '/recent',
    icon: () => h(TimeOutline)
  },
  {
    label: '收藏',
    key: '/starred',
    icon: () => h(StarOutline)
  },
  {
    label: '共享',
    key: '/shared',
    icon: () => h(ShareSocialOutline)
  },
  {
    label: '回收站',
    key: '/trash',
    icon: () => h(TrashBinOutline)
  },
  {
    type: 'divider',
    key: 'divider-1'
  },
  {
    label: '文件夹',
    key: 'folders',
    icon: () => h(FolderOpenOutline),
    children: fileStore.folderTree.length > 0 
      ? fileStore.folderTree[0].children?.map(folder => ({
          label: folder.name,
          key: folder.path,
          icon: () => h(FolderOutline)
        })) || []
      : []
  }
])

// 当前选中的菜单项
const activeKey = computed(() => {
  const path = router.currentRoute.value.path
  if (path.startsWith('/folder/')) return path
  return path
})

// 处理菜单选择
function handleMenuSelect(key: string) {
  if (key.startsWith('/')) {
    router.push(key)
  }
}

// 处理新建文件夹
function handleNewFolder() {
  showCreateFolderModal.value = true
}

// 处理刷新文件夹树
function handleRefreshTree() {
  console.log('刷新文件夹树')
  fileStore.loadFolderTree()
}

// 计算存储空间使用情况（模拟数据）
const storageStats = computed(() => ({
  used: 42.8, // GB
  total: 100, // GB
  percent: 42.8
}))
</script>

<template>
  <NLayoutSider
    class="sidebar"
    :native-scrollbar="false"
    :collapsed-width="64"
    :width="240"
    show-trigger="bar"
    bordered
    collapse-mode="width"
  >
    <div class="sidebar-content">
      <!-- 用户信息区域 -->
      <div class="user-section">
        <div class="user-avatar">
          <NIcon size="32" color="#0066ff">
            <CloudOutline />
          </NIcon>
        </div>
        <div class="user-info">
          <div class="user-name">CAMFC Cloud</div>
          <div class="user-email">免费云存储空间</div>
        </div>
      </div>
      
      <NDivider style="margin: 16px 0;" />
      
      <!-- 操作按钮 -->
      <div class="action-buttons">
        <NButton
          type="primary"
          ghost
          block
          @click="handleNewFolder"
        >
          <template #icon>
            <NIcon :component="AddOutline" />
          </template>
          新建文件夹
        </NButton>
      </div>
      
      <!-- 主菜单 -->
      <NMenu
        class="sidebar-menu"
        :value="activeKey"
        :options="menuItems"
        @update:value="handleMenuSelect"
        :indent="18"
      />
      
      <!-- 存储空间使用情况 -->
      <div class="storage-section">
        <NDivider style="margin: 16px 0;">存储空间</NDivider>
        
        <div class="storage-info">
          <div class="storage-header">
            <span class="storage-title">已使用 {{ storageStats.used }} GB</span>
            <span class="storage-total">/ {{ storageStats.total }} GB</span>
          </div>
          
          <div class="storage-progress">
            <div 
              class="storage-progress-bar"
              :style="{ width: `${storageStats.percent}%` }"
            ></div>
          </div>
          
          <div class="storage-details">
            <span class="storage-percent">{{ storageStats.percent.toFixed(1) }}%</span>
            <NButton
              text
              size="tiny"
              @click="handleRefreshTree"
            >
              刷新
            </NButton>
          </div>
        </div>
      </div>
      
      <!-- 侧边栏底部 -->
      <div class="sidebar-footer">
        <div class="version-info">v1.0.0</div>
        <div class="copyright">© 2026 CAMFC Cloud</div>
      </div>
    </div>
  </NLayoutSider>
  
  <!-- 新建文件夹模态框 -->
  <CreateFolderModal
    v-model:show="showCreateFolderModal"
    @created="() => console.log('文件夹创建成功')"
  />
</template>

<style scoped>
.sidebar {
  backdrop-filter: blur(20px);
  background: rgba(255, 255, 255, 0.7) !important;
  border-right: 1px solid var(--n-border-color);
  height: calc(100vh - 64px);
  position: sticky;
  top: 64px;
}

.dark .sidebar {
  background: rgba(30, 30, 30, 0.7) !important;
}

.sidebar-content {
  display: flex;
  flex-direction: column;
  height: 100%;
  padding: 16px;
}

.user-section {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 8px;
  border-radius: 12px;
  background-color: var(--n-color-modal);
  margin-bottom: 8px;
}

.user-avatar {
  width: 40px;
  height: 40px;
  border-radius: 50%;
  background: linear-gradient(135deg, #0066ff, #4caf50);
  display: flex;
  align-items: center;
  justify-content: center;
}

.user-info {
  flex: 1;
}

.user-name {
  font-weight: 600;
  font-size: 14px;
  color: var(--n-text-color);
}

.user-email {
  font-size: 12px;
  color: var(--n-text-color-disabled);
  margin-top: 2px;
}

.action-buttons {
  margin-bottom: 16px;
}

.sidebar-menu {
  flex: 1;
  margin-bottom: 16px;
}

.storage-section {
  background-color: var(--n-color-modal);
  border-radius: 12px;
  padding: 16px;
  margin-bottom: 16px;
}

.storage-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 8px;
}

.storage-title {
  font-weight: 600;
  font-size: 14px;
  color: var(--n-text-color);
}

.storage-total {
  font-size: 12px;
  color: var(--n-text-color-disabled);
}

.storage-progress {
  height: 6px;
  background-color: var(--n-color-border);
  border-radius: 3px;
  overflow: hidden;
  margin-bottom: 8px;
}

.storage-progress-bar {
  height: 100%;
  background: linear-gradient(90deg, #0066ff, #4caf50);
  border-radius: 3px;
  transition: width 0.3s ease;
}

.storage-details {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.storage-percent {
  font-size: 12px;
  color: var(--n-text-color-disabled);
}

.sidebar-footer {
  text-align: center;
  padding-top: 16px;
  border-top: 1px solid var(--n-border-color);
}

.version-info {
  font-size: 11px;
  color: var(--n-text-color-disabled);
  margin-bottom: 4px;
}

.copyright {
  font-size: 10px;
  color: var(--n-text-color-disabled);
  opacity: 0.7;
}

/* 响应式设计 */
@media (max-width: 768px) {
  .sidebar {
    position: fixed;
    top: 64px;
    left: 0;
    z-index: 999;
    height: calc(100vh - 64px);
  }
  
  .user-info .user-email {
    display: none;
  }
}
</style>
