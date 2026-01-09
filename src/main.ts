import { createApp } from "vue";
import App from "./App.vue";
import router from "./router"; // 导入路由配置

createApp(App)
    .use(router) // 使用路由
    .mount("#app");


// 样式清空
const link = document.createElement("link");
link.rel = "stylesheet";
link.href =
    "https://cdnjs.cloudflare.com/ajax/libs/normalize/8.0.1/normalize.min.css";
document.head.appendChild(link);
