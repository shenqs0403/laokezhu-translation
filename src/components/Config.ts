import {ref} from "vue";
import {Engine} from "./Common.ts";

// 基础配置信息对象
export const basic = ref({shortcut: "", swipe: 300})
// 当前编辑的引擎
export const currentEngine = ref<Engine>({
    engine_key: "",
    engine_name: "",
    engine_zh_name: "",
    url: "",
    appid: "",
    enable: false
});