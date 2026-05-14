

import { createApp } from "vue";
import { createPinia } from 'pinia'
import App from "./App.vue";
import router from "./router";

import "./styles/global.css";

import "remixicon/fonts/remixicon.css";

import "normalize.css";

document.addEventListener('contextmenu', e => e.preventDefault());

const pinia = createPinia()

createApp(App)
    .use(pinia)
    .use(router)
    .mount("#app");
