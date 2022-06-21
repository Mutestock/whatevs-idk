import { createRouter, createWebHashHistory } from "vue-router";
import HelloWorld from "../components/HelloWorld.vue";

export const baseRouter = createRouter ({
    history: createWebHashHistory(),
    routes: [
        { path: "/", component: HelloWorld }
    ]
});
