import { createRouter, createWebHashHistory } from "vue-router";
import Home from "../components/Home.vue";

export const baseRouter = createRouter ({
    history: createWebHashHistory(),
    routes: [
        { path: "/", component: Home }
    ]
});
