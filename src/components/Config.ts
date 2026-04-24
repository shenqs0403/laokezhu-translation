import {ref} from "vue";
import {Engine} from "./Common.ts";

// 基础配置信息对象
export const basic = ref({shortcut: "",swipe: false})
// 当前编辑的引擎
export const currentEngine = ref<Engine>({
    engineName: "",
    engineZhName: "",
    url: "",
    appid: "",
    engineKey: "",
    enable: false
});