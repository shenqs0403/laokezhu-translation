import {createRouter,createWebHashHistory,RouteRecordRaw} from "vue-router";
import ConfigView from "../views/ConfigView.vue";
import TranslateView from "../views/TranslateView.vue";

const routes: RouteRecordRaw[] = [{
    path: "/config",
    component: ConfigView
},{
    path:"/translate",
    component: TranslateView
}];

const router = createRouter({
    history: createWebHashHistory(),
    routes
});

export default router;