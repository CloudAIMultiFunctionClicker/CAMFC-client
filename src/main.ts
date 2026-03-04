/**
 * CAMFC Client - 主入口文件
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
 */

import { createApp } from "vue";
import { createPinia } from 'pinia'  // 导入Pinia
import App from "./App.vue";
import router from "./router"; // 导入路由配置

// 导入本地样式文件 - 这样Vite会正确打包这些资源
// 1. Remix Icon 图标库 - 之前用CDN，现在用本地npm包
import "remixicon/fonts/remixicon.css";
// 2. Normalize.css - 样式重置，避免CDN依赖
import "normalize.css";

document.addEventListener('contextmenu', e => e.preventDefault());

/**
 * 应用入口文件
 * 这里初始化Vue应用，并挂载到DOM
 * 
 * 修复记录：
 * 1. 删除了CDN动态加载，改用本地导入（Tauri打包后无法访问CDN）
 * 2. Remix Icon图标库现在使用本地npm包，确保离线可用
 * 3. Normalize.css也改用本地包，避免外部依赖
 * 4. 添加Pinia状态管理
 */

// 创建Pinia实例
const pinia = createPinia()

// 创建Vue应用实例，使用路由和Pinia插件，然后挂载到#app元素
createApp(App)
    .use(pinia)  // 注册Pinia
    .use(router) // 注册路由
    .mount("#app");
