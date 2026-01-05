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

import { ref, computed, watch } from 'vue'
import {
  NModal,
  NInput,
  NButton,
  NForm,
  NFormItem,
  NSpace,
  NText
} from 'naive-ui'
import { useFileStore } from '@/stores/useFileStore'

const props = defineProps<{
  show: boolean
}>()

const emit = defineEmits<{
  (e: 'update:show', value: boolean): void
  (e: 'created'): void
}>()

const fileStore = useFileStore()

// 文件夹名称
const folderName = ref('')
// 正在加载状态
const loading = ref(false)
// 错误信息
const error = ref('')

// 计算模态框是否显示
const modalShow = computed({
  get: () => props.show,
  set: (value: boolean) => {
    emit('update:show', value)
  }
})

// 重置表单
function resetForm() {
  folderName.value = ''
  loading.value = false
  error.value = ''
}

// 当模态框显示时重置表单
watch(() => props.show, (newVal) => {
  if (newVal) {
    resetForm()
  }
})

// 验证文件夹名称
function validateFolderName(name: string): string {
  if (!name.trim()) {
    return '文件夹名称不能为空'
  }
  
  if (name.length > 100) {
    return '文件夹名称不能超过100个字符'
  }
  
  // 检查文件夹名称是否包含非法字符
  const invalidChars = /[<>:"/\\|?*\x00-\x1F]/
  if (invalidChars.test(name)) {
    return '文件夹名称不能包含以下字符：<>:"/\\|?*'
  }
  
  return ''
}

// 创建文件夹
async function createFolder() {
  const name = folderName.value.trim()
  const validationError = validateFolderName(name)
  
  if (validationError) {
    error.value = validationError
    return
  }
  
  loading.value = true
  error.value = ''
  
  try {
    // 调用store中的创建文件夹action
    await fileStore.createNewFolder(name)
    
    // 创建成功后关闭模态框并触发created事件
    modalShow.value = false
    emit('created')
    
  } catch (err: any) {
    error.value = err.message || '创建文件夹失败，请重试'
    console.error('创建文件夹失败:', err)
  } finally {
    loading.value = false
  }
}

// 处理键盘事件
function handleKeydown(e: KeyboardEvent) {
  if (e.key === 'Enter' && !loading.value) {
    createFolder()
  }
}
</script>

<template>
  <NModal
    v-model:show="modalShow"
    preset="card"
    title="新建文件夹"
    :bordered="false"
    :closable="!loading"
    :mask-closable="!loading"
    style="width: 480px; max-width: 90vw;"
    :segmented="{
      footer: 'soft'
    }"
    @after-leave="resetForm"
  >
    <template #header>
      <div style="font-weight: 600; font-size: 18px;">新建文件夹</div>
    </template>

    <NForm label-placement="top" :show-feedback="!!error">
      <NFormItem label="文件夹名称" :feedback="error" :validation-status="error ? 'error' : undefined">
        <NInput
          v-model:value="folderName"
          placeholder="请输入文件夹名称"
          :maxlength="100"
          :disabled="loading"
          @keydown="handleKeydown"
          clearable
          autofocus
        />
      </NFormItem>
      
      <NText depth="3" style="font-size: 12px;">
        将在当前目录 "{{ fileStore.currentPath }}" 下创建文件夹
      </NText>
    </NForm>

    <template #footer>
      <NSpace justify="end" style="width: 100%;">
        <NButton
          :disabled="loading"
          @click="modalShow = false"
        >
          取消
        </NButton>
        <NButton
          type="primary"
          :loading="loading"
          @click="createFolder"
        >
          创建
        </NButton>
      </NSpace>
    </template>
  </NModal>
</template>
