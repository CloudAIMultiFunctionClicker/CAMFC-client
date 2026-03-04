/**
 * CAMFC Client - 弹出提示卡片（Toast） - 已迁移到 Vue 风格
 * 
 * Copyright (C) 2026 Jiale Xu (许嘉乐) (ANTmmmmm) <https://github.com/ant-cave>
 * Email: ANTmmmmm@outlook.com, ANTmmmmm@126.com, 1504596931@qq.com
 *
 * Copyright (C) 2026 Xinhang Chen (陈欣航) <https://github.com/cxh09>
 * Email: abc.cxh2009@foxmail.com
 *
 * Copyright (C) 2026 Zimo Wen (温子墨) <https://github.com/lusamaqq>
 * Email: 1220594170@qq.com
 *
 * Copyright (C) 2026 Kaibin Zeng (曾楷彬) <https://github.com/Waple1145>
 * Email: admin@mc666.top
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
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
