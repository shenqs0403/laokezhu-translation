import {ref} from "vue";

// 基础配置信息对象
export const basic = ref({shortcut: "",swipe: false})
// 当前编辑的引擎
export const currentEngine = ref({
    engineName: "",
    url: "",
    appid: "",
    engineKey: "",
    enable: false
})