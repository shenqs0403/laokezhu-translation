import { createApp } from "vue";
import App from "./App.vue";
import router from "./components/router.ts";
import NaiveUI from "naive-ui"

createApp(App)
    .use(NaiveUI)
    .use(router)
    .mount("#app");
