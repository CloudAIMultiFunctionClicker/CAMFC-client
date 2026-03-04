<!--
Copyright (C) 2026 Jiale Xu (许嘉乐) (ANTmmmmm) <https://github.com/ant-cave>
Email: ANTmmmmm@outlook.com, ANTmmmmm@126.com, 1504596931@qq.com

Copyright (C) 2026 Xinhang Chen (陈欣航) <https://github.com/cxh09>
Email: abc.cxh2009@foxmail.com

Copyright (C) 2026 Zimo Wen (温子墨) <https://github.com/lusamaqq>
Email: 1220594170@qq.com

Copyright (C) 2026 Kaibin Zeng (曾楷彬) <https://github.com/Waple1145>
Email: admin@mc666.top

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
  <div class="placeholder-container">
    <h1 class="placeholder-title">软件设置</h1>
    
    <div class="placeholder-content">
      <!-- 占位图标 -->
      <div class="placeholder-icon">🖥️</div>
      
      <p class="placeholder-message">
        软件设置功能正在开发中...
      </p>
      
      <p class="placeholder-desc">
        这里将用于配置CAMFC软件的偏好设置，如主题切换、通知选项、界面语言等。
        <br>
        目前该页面仅为占位，后续会逐步完善。
      </p>
      
      <!-- 返回按钮 -->
      <button class="back-btn" @click="goBack">
        返回主仪表板
      </button>
    </div>
    
    <!-- TODO: 软件设置可能涉及用户偏好保存，需要localStorage或后端API -->
    <p class="placeholder-footer">
      TODO: 软件设置需要考虑用户偏好持久化存储方案
    </p>
  </div>
</template>

<script setup>
// 软件设置占位页面
// 和硬件设置页面类似，就是个占位
// 注：后续这里可能会比较复杂，因为涉及用户偏好保存

import { useRouter } from 'vue-router'

const router = useRouter()

// 返回主仪表板
// 这里用固定路径，避免router.back()的不确定性
function goBack() {
  console.log('返回主仪表板')
  router.push('/main')
}

// 思考：软件设置应该包括哪些内容？
// 1. 主题切换（亮色/暗色） - 已经有了
// 2. 语言选择 - 可能需要国际化
// 3. 通知设置 - 比如蓝牙连接成功提示
// 4. 数据清理 - 清除本地缓存
// 这些等以后再做吧
</script>

<style scoped>
/* 样式复用硬件设置页面的，保持一致性 */
/* 其实可以抽成公共样式，但用户说简单实现，先复制一份吧 */
/* TODO: 重复的CSS，考虑抽成公共样式文件 */

.placeholder-container {
  padding: 40px 20px;
  max-width: 800px;
  margin: 0 auto;
  text-align: center;
}

.placeholder-title {
  font-size: 28px;
  margin-bottom: 30px;
  color: var(--text-primary);
}

.placeholder-content {
  background-color: var(--bg-secondary);
  border-radius: 12px;
  padding: 40px 30px;
  border: 1px solid var(--border-color);
  margin-bottom: 30px;
}

.placeholder-icon {
  font-size: 64px;
  margin-bottom: 20px;
}

.placeholder-message {
  font-size: 20px;
  color: var(--text-primary);
  margin-bottom: 15px;
  font-weight: 500;
}

.placeholder-desc {
  font-size: 16px;
  color: var(--text-secondary);
  line-height: 1.6;
  margin-bottom: 25px;
  max-width: 600px;
  margin-left: auto;
  margin-right: auto;
}

.back-btn {
  padding: 12px 30px;
  background-color: var(--accent-blue);
  color: white;
  border: none;
  border-radius: 8px;
  font-size: 16px;
  cursor: pointer;
  transition: background-color 0.2s;
  
  &:hover {
    background-color: #4a8bd6;
  }
  
  &:active {
    transform: translateY(1px);
  }
}

.placeholder-footer {
  font-size: 14px;
  color: var(--text-muted);
  font-style: italic;
  margin-top: 20px;
  padding-top: 20px;
  border-top: 1px dashed var(--border-color);
}

/* 响应式调整 */
@media (max-width: 768px) {
  .placeholder-container {
    padding: 20px 15px;
  }
  
  .placeholder-content {
    padding: 30px 20px;
  }
  
  .placeholder-icon {
    font-size: 48px;
  }
}
</style>