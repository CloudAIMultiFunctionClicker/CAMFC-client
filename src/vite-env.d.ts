/// <reference types="vite/client" />

declare module "*.vue" {
  import type { DefineComponent } from "vue";
  const component: DefineComponent<{}, {}, any>;
  export default component;
}


export default {
  build: {
    rollupOptions: {
      output: {
        manualChunks: {
          vendor: ['vue', 'axios'], // 第三方库单独打包
          // 或自动拆分 node_modules
          // vendor(id) { if (id.includes('node_modules')) return 'vendor'; }
        }
      }
    }
  }
}