import { createApp } from 'vue'
import router from './router/index'
import App from './App.vue'
import TDesign from 'tdesign-vue-next';
import './router/auth'

import 'tdesign-vue-next/es/style/index.css';
import './style.scss';


createApp(App).use(router).use(TDesign).mount('#app')
