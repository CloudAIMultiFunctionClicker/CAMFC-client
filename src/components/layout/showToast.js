/**
 * CAMFC Client - Toast 提示快捷导出
 * 其实就是个转发层，为了兼容旧代码
 * 
 * 试过直接把所有引用都改了，但太麻烦了，以后再说吧
 */

// 从 composables 重新导出，保持 API 兼容
import { showToast } from '../../composables/useToast.js'

export { showToast }

// TODO: 等所有引用都更新后删除这个文件
