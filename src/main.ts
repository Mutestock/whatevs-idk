import { createApp } from 'vue'
import { createPinia } from 'pinia'
import App from './App.vue'
import { baseRouter } from './router/baseRoutes';
import devalue from '@nuxt/devalue';

const app = createApp(App);
const pinia = createPinia();
app.use(pinia);
app.use(baseRouter);
devalue(pinia.state.value)

app.mount('#app')
