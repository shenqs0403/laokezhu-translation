import {createRouter,createWebHashHistory,RouteRecordRaw} from "vue-router";
import ConfigView from "../views/ConfigView.vue";
import TranslateView from "../views/TranslateView.vue";
import SwipeMenuView from "../views/SwipeMenuView.vue";
import AboutView from "../views/AboutView.vue";

const routes: RouteRecordRaw[] = [{
    path: "/config",
    component: ConfigView
},{
    path:"/translate",
    component: TranslateView
},{
    path:"/menu",
    component: SwipeMenuView
},{
    path:"/about",
    component: AboutView
}];

const router = createRouter({
    history: createWebHashHistory(),
    routes
});

export default router;