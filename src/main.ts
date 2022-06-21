import { createApp } from 'vue'
import { createPinia } from 'pinia'
import App from './App.vue'
import { baseRouter } from './router/baseRoutes';

const app = createApp(App);
app.use(createPinia());
app.use(baseRouter);

app.mount('#app')
