import { createApp } from "vue";
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
 */

// 创建Vue应用实例，使用路由插件，然后挂载到#app元素
createApp(App)
    .use(router) // 注册路由
    .mount("#app");
