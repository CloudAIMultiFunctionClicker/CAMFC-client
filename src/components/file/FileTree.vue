<!--
保留所有权利

Copyright (C) 2026 Jiale Xu (许嘉乐) (ANTmmmmm) <https://github.com/ant-cave>
Email: ANTmmmmm@outlook.com, ANTmmmmm@126.com, 1504596931@qq.com

Copyright (C) 2026 Xinhang Chen (陈欣航) <https://github.com/cxh09>
Email: abc.cxh2009@foxmail.com

Copyright (C) 2026 Zimo Wen (温子墨) <https://github.com/lusamaqq>
Email: 1220594170@qq.com

Copyright (C) 2026 Kaibin Zeng (曾楷彬) <https://github.com/Waple1145>
Email: admin@mc666.top
-->

<script setup>
import { ref, watch } from 'vue'
import { ls } from '../data/fileSystem.js'
import { getFileIcon } from '../../utils/fileIcon.js'

// 接收当前路径作为参数
const props = defineProps({
  currentPath: {
    type: String,
    default: ''
  }
})

// 发送路径变化事件
const emit = defineEmits(['path-change'])

// 树节点数据结构
const treeData = ref([])
const loading = ref(false)
const expandedFolders = ref(new Set()) // 记录展开的文件夹

// 加载根目录
const loadRoot = async () => {
  loading.value = true
  try {
    const result = await ls('')
    if (result?.entries) {
      treeData.value = buildTreeNodes(result.entries, '')
    }
  } catch (error) {
    console.error('加载根目录失败:', error)
  } finally {
    loading.value = false
  }
}

// 构建树节点
const buildTreeNodes = (entries, parentPath) => {
  return entries.map(item => {
    const isDir = item.is_dir || item.type === 'dir' || item.type === 'folder'
    const path = item.path || (parentPath ? `${parentPath}/${item.name}` : item.name)
    
    return {
      ...item,
      path,
      is_dir: isDir,
      children: isDir ? [] : null, // 文件夹才有 children
      loaded: false // 标记是否已加载过子节点
    }
  })
}

// 加载子文件夹内容
const loadChildren = async (node) => {
  if (node.loaded || !node.is_dir) return
  
  try {
    const result = await ls(node.path)
    if (result?.entries) {
      node.children = buildTreeNodes(result.entries, node.path)
      node.loaded = true
    }
  } catch (error) {
    console.error(`加载文件夹 ${node.path} 失败:`, error)
  }
}

// 点击文件夹节点
const handleNodeClick = async (node) => {
  if (node.is_dir) {
    // 切换展开/折叠状态
    if (expandedFolders.value.has(node.path)) {
      expandedFolders.value.delete(node.path)
    } else {
      expandedFolders.value.add(node.path)
      await loadChildren(node)
    }
    // 同时切换当前路径
    emit('path-change', node.path)
  } else {
    // 点击文件，切换当前路径（用于在 FileTable 中显示）
    emit('path-change', node.path)
  }
}

// 判断节点是否展开
const isExpanded = (node) => {
  return expandedFolders.value.has(node.path)
}

// 监听当前路径变化，自动展开对应的文件夹
watch(() => props.currentPath, (newPath) => {
  if (newPath) {
    autoExpandToPath(newPath)
  }
}, { immediate: true })

// 自动展开到指定路径
const autoExpandToPath = async (targetPath) => {
  const parts = targetPath.split('/').filter(p => p !== '')
  let currentPath = ''
  
  for (const part of parts) {
    currentPath = currentPath ? `${currentPath}/${part}` : part
    
    // 找到对应的节点
    const node = findNodeByPath(treeData.value, currentPath)
    if (node && node.is_dir) {
      expandedFolders.value.add(currentPath)
      await loadChildren(node)
    }
  }
}

// 根据路径查找节点
const findNodeByPath = (nodes, path) => {
  for (const node of nodes) {
    if (node.path === path) return node
    if (node.children) {
      const found = findNodeByPath(node.children, path)
      if (found) return found
    }
  }
  return null
}

// 组件挂载时加载根目录
loadRoot()
</script>

<template>
  <div class="file-tree-container">
    <div class="tree-header">
      <i class="ri-folder-tree-line"></i>
      <span>文件树</span>
    </div>
    
    <div class="tree-content" :class="{ loading: loading }">
      <!-- 加载状态 -->
      <div v-if="loading" class="tree-loading">
        <i class="ri-loader-4-line spin"></i>
        <span>加载中...</span>
      </div>
      
      <!-- 树形列表 -->
      <div v-else class="tree-list">
        <div
          v-for="node in treeData"
          :key="node.path"
          class="tree-node"
        >
          <tree-node-item
            :node="node"
            :current-path="currentPath"
            :expanded-folders="expandedFolders"
            @click="handleNodeClick(node)"
          />
        </div>
      </div>
    </div>
  </div>
</template>

<!-- 递归组件：树节点 -->
<script setup>
// 定义一个内部组件用于递归渲染
const TreeNodeItem = {
  name: 'TreeNodeItem',
  props: ['node', 'currentPath', 'expandedFolders'],
  setup(props) {
    const emit = defineEmits(['click'])
    
    const handleClick = (e) => {
      e.stopPropagation()
      emit('click', props.node)
    }
    
    return { handleClick }
  },
  template: `
    <div class="tree-node-item" :class="{ 
      'is-dir': node.is_dir, 
      'is-file': !node.is_dir,
      'active': currentPath === node.path
    }">
      <div class="node-content" @click="handleClick">
        <!-- 展开/折叠图标 -->
        <span v-if="node.is_dir" class="expand-icon">
          <i :class="expandedFolders.has(node.path) ? 'ri-arrow-down-s-line' : 'ri-arrow-right-s-line'"></i>
        </span>
        <span v-else class="expand-placeholder"></span>
        
        <!-- 文件/文件夹图标 -->
        <i :class="node.is_dir ? 'ri-folder-line' : getFileIcon(node.name)" class="file-icon"></i>
        
        <!-- 名称 -->
        <span class="node-name" :title="node.name">{{ node.name }}</span>
      </div>
      
      <!-- 递归渲染子节点 -->
      <div v-if="node.is_dir && expandedFolders.has(node.path)" class="node-children">
        <tree-node-item
          v-for="child in node.children"
          :key="child.path"
          :node="child"
          :current-path="currentPath"
          :expanded-folders="expandedFolders"
          @click="$emit('click', $event)"
        />
      </div>
    </div>
  `
}
</script>

<style scoped>
.file-tree-container {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
  background: var(--bg-secondary, #ffffff);
  border-radius: .375rem;
  border: 1px solid var(--border-color, #d0d7de);
  overflow: hidden;
}

.tree-header {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 12px 16px;
  background: var(--bg-tertiary, #f6f8fa);
  border-bottom: 1px solid var(--border-color, #d0d7de);
  font-weight: 600;
  color: var(--text-secondary, #57606a);
  font-size: 14px;
}

.tree-header i {
  font-size: 16px;
  color: var(--accent-blue, #0969da);
}

.tree-content {
  flex: 1;
  overflow-y: auto;
  padding: 8px 0;
  position: relative;
}

.tree-content.loading {
  opacity: 0.6;
}

.tree-loading {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 40px 20px;
  color: var(--text-muted, #8c959f);
  gap: 12px;
}

.spin {
  animation: spin 1s linear infinite;
  font-size: 24px;
  color: var(--accent-blue, #0969da);
}

@keyframes spin {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}

.tree-list {
  min-height: 100px;
}

.tree-node {
  user-select: none;
}

.tree-node-item {
  transition: background-color 0.15s ease;
}

.node-content {
  display: flex;
  align-items: center;
  gap: 4px;
  padding: 6px 12px;
  cursor: pointer;
  transition: background-color 0.15s ease;
}

.node-content:hover {
  background: var(--hover-bg, #f3f4f6);
}

.node-content.active {
  background: var(--selected-bg, #ddf4ff);
  color: var(--accent-blue, #0969da);
}

.expand-icon,
.expand-placeholder {
  width: 16px;
  height: 16px;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}

.expand-icon {
  color: var(--text-muted, #8c959f);
  cursor: pointer;
  transition: transform 0.2s ease;
}

.expand-icon i {
  font-size: 14px;
}

.file-icon {
  font-size: 14px;
  color: var(--text-secondary, #57606a);
  flex-shrink: 0;
  width: 16px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.node-content.active .file-icon {
  color: var(--accent-blue, #0969da);
}

.node-name {
  flex: 1;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  font-size: 13px;
  color: var(--text-primary, #24292f);
}

.node-content:hover .node-name {
  color: var(--text-primary, #24292f);
}

.node-content.active .node-name {
  color: var(--accent-blue, #0969da);
  font-weight: 500;
}

.node-children {
  padding-left: 16px; /* 缩进 */
}

/* 滚动条样式 */
.tree-content::-webkit-scrollbar {
  width: 6px;
}

.tree-content::-webkit-scrollbar-track {
  background: transparent;
}

.tree-content::-webkit-scrollbar-thumb {
  background: var(--border-color, #d0d7de);
  border-radius: 3px;
}

.tree-content::-webkit-scrollbar-thumb:hover {
  background: var(--text-muted, #8c959f);
}

/* 空状态 */
.tree-list:empty {
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 40px 20px;
  color: var(--text-muted, #8c959f);
  text-align: center;
}
</style>
