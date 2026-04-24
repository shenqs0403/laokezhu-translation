import {createRouter,createWebHashHistory,RouteRecordRaw} from "vue-router";
import ConfigView from "../views/ConfigView.vue";

const routes: RouteRecordRaw[] = [{
    path: "/config",
    component: ConfigView
}];

const router = createRouter({
    history: createWebHashHistory(),
    routes
});

export default router;