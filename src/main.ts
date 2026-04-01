// 应用入口文件
// 创建 Vue 实例，注册 Pinia 和路由，然后挂载

import { createApp } from "vue"
import { createPinia } from 'pinia'
import App from "./App.vue"
import router from "./router"

// 导入样式
import "remixicon/fonts/remixicon.css"
import "normalize.css"

// 禁用右键菜单
document.addEventListener('contextmenu', e => e.preventDefault())

// 创建 Pinia 实例
const pinia = createPinia()

// 创建并挂载应用
createApp(App)
  .use(pinia)
  .use(router)
  .mount("#app")
