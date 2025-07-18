import {createApp} from 'vue';
import App from './App.vue';
import ElementPlus from 'element-plus';
import 'element-plus/dist/index.css';
import * as ElementPlusIconsVue from '@element-plus/icons-vue';
import router from "./router";
import './style/base.css';
// import {msg} from './util/message.js';

const app = createApp(App);

for (const [key, component] of Object.entries(ElementPlusIconsVue)) {
    app.component(key, component);
}

app.use(ElementPlus,);
app.use(router)

// app.config.globalProperties.$msg = msg;
// console.log('全局挂载的 $msg:', app.config.globalProperties.$msg);

app.mount('#app');