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

import { ref } from 'vue'
import { NModal, NButton, NInput, NRadio, NSpace, NText, NDivider, NSelect, NAlert } from 'naive-ui'

const props = defineProps<{
  show: boolean
  fileIds: string[]
}>()

const emit = defineEmits<{
  'update:show': [value: boolean]
  close: []
}>()

// 分享设置
const shareType = ref('public') // public 或 private
const hasPassword = ref(false)
const password = ref('')
const expiresAt = ref<number>(-1)
const shareLink = ref('https://camfc-cloud.com/share/abc123')
const copied = ref(false)

// 有效期选项
const expiryOptions = [
  { label: '1天', value: 1 },
  { label: '7天', value: 7 },
  { label: '30天', value: 30 },
  { label: '永久', value: -1 }
]

function handleCopyLink() {
  navigator.clipboard.writeText(shareLink.value)
  copied.value = true
  setTimeout(() => {
    copied.value = false
  }, 2000)
}

function handleClose() {
  emit('update:show', false)
  emit('close')
}

function handleCreateShare() {
  console.log('创建分享链接:', {
    fileIds: props.fileIds,
    shareType: shareType.value,
    hasPassword: hasPassword.value,
    expiresAt: expiresAt.value
  })
  // TODO: 调用实际API
  handleClose()
}
</script>

<template>
  <NModal
    :show="show"
    :mask-closable="false"
    preset="card"
    title="分享文件"
    style="width: 500px; max-width: 90vw;"
    @close="handleClose"
  >
    <div class="share-modal">
      <NAlert type="info" title="分享提示" style="margin-bottom: 20px;">
        分享链接生成后，任何拥有该链接的人都可以访问文件。请谨慎设置分享权限。
      </NAlert>
      
      <!-- 分享链接预览 -->
      <div class="link-preview" style="margin-bottom: 24px;">
        <div class="link-label" style="margin-bottom: 8px; font-weight: 500;">
          分享链接
        </div>
        <div class="link-input-group">
          <NInput
            :value="shareLink"
            readonly
            placeholder="点击复制链接"
            @click="handleCopyLink"
          >
            <template #suffix>
              <NButton
                text
                size="tiny"
                @click="handleCopyLink"
              >
                {{ copied ? '已复制' : '复制' }}
              </NButton>
            </template>
          </NInput>
        </div>
      </div>
      
      <NDivider />
      
      <!-- 分享设置 -->
      <div class="share-settings">
        <div class="setting-item" style="margin-bottom: 20px;">
          <div class="setting-label" style="margin-bottom: 8px; font-weight: 500;">
            分享类型
          </div>
          <NSpace>
            <NRadio
              :value="'public'"
              :checked="shareType === 'public'"
              @update:checked="shareType = 'public'"
              name="share-type"
            >
              公开分享
            </NRadio>
            <NRadio
              :value="'private'"
              :checked="shareType === 'private'"
              @update:checked="shareType = 'private'"
              name="share-type"
            >
              私密分享（需要密码）
            </NRadio>
          </NSpace>
        </div>
        
        <div v-if="shareType === 'private'" class="setting-item" style="margin-bottom: 20px;">
          <div class="setting-label" style="margin-bottom: 8px; font-weight: 500;">
            访问密码
          </div>
          <NInput
            v-model:value="password"
            type="password"
            placeholder="输入访问密码"
            show-password-on="click"
            style="max-width: 200px;"
          />
        </div>
        
        <div class="setting-item" style="margin-bottom: 24px;">
          <div class="setting-label" style="margin-bottom: 8px; font-weight: 500;">
            链接有效期
          </div>
          <NSelect
            v-model:value="expiresAt"
            :options="expiryOptions"
            placeholder="选择有效期"
            style="max-width: 200px;"
          />
          <NText depth="3" style="margin-left: 12px; font-size: 12px;">
            {{ expiresAt !== -1 ? `链接将在${expiresAt}天后失效` : '永久有效' }}
          </NText>
        </div>
      </div>
      
      <NDivider />
      
      <!-- 操作按钮 -->
      <div class="modal-actions">
        <NSpace justify="end" style="width: 100%;">
          <NButton @click="handleClose">
            取消
          </NButton>
          <NButton type="primary" @click="handleCreateShare">
            创建分享
          </NButton>
        </NSpace>
      </div>
    </div>
  </NModal>
</template>

<style scoped>
.share-modal {
  padding: 4px;
}

.link-input-group {
  display: flex;
  gap: 8px;
}

.setting-label {
  color: var(--n-text-color);
}

.modal-actions {
  padding-top: 16px;
}
</style>
