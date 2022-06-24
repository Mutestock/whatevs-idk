import { createRouter, createWebHashHistory } from "vue-router";
import Home from "../views/Home.vue";
import PrimaryDatabaseOverview from "../views/PrimaryDatabaseOverview.vue"

export const baseRouter = createRouter({
    history: createWebHashHistory(),
    routes: [
        { path: "/", component: Home }, { path: "/database-overview", component: PrimaryDatabaseOverview }
    ]
});
