import { createApp } from 'vue'
import router from './route'
import store from './store'
import App from './App.vue'

import '~/styles/index.scss'

import { SvgIcon } from './icons'

const app = createApp(App)
app.use(router)
app.use(store)
app.component('svg-icon', SvgIcon)
app.mount('#app')
