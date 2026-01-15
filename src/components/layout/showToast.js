/**
 * 弹出提示卡片（Toast） - 已迁移到 Vue 风格
 * 
 * 注意：这个文件现在只是重新导出来自 composables/useToast.js 的函数
 * 为了向后兼容，保持原来的 API 不变
 * 
 * @deprecated 建议改用 import { useToast } from '@/composables/useToast'
 * 或者直接使用 import { showToast } from '@/composables/useToast'
 * 
 * TODO: 等所有使用的地方都更新后，可以删除这个文件
 */

// 重新导出新的 showToast 函数，保持 API 兼容
// 这里用动态导入避免循环依赖？应该不需要，直接导入吧
import { showToast } from '../../composables/useToast.js'

// 保持原来的导出名称
export { showToast }
