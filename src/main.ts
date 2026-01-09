import { createApp } from "vue";
import App from "./App.vue";
import router from "./router"; // 导入路由配置

/**
 * 应用入口文件
 * 这里初始化Vue应用，并挂载到DOM
 */

// 创建Vue应用实例，使用路由插件，然后挂载到#app元素
createApp(App)
    .use(router) // 注册路由
    .mount("#app");


// 样式重置 - 使用Normalize.css统一不同浏览器的默认样式
// 注意：这会在运行时动态加载CSS，可能影响首次加载性能
// TODO: 考虑将Normalize.css内置到项目中，避免CDN依赖
const link = document.createElement("link");
link.rel = "stylesheet";
link.href =
    "https://cdnjs.cloudflare.com/ajax/libs/normalize/8.0.1/normalize.min.css";
document.head.appendChild(link);
