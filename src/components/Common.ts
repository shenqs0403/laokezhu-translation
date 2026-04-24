// 一些通用的
import {ref} from "vue";
import {invoke} from "@tauri-apps/api/core";

/**
 * 引擎信息封装对象
 */
export interface Engine {
    engine_name: string,
    engine_zh_name: string,
    url: string,
    appid: string,
    engine_key: string,
    enable: boolean,
    label?: string,
    value?: string
}

export const engines = ref<Engine[]>();

export const loadAllEngines = () => {
  invoke<Engine[]>("get_all_engines").then(value => {
      console.log("加载引擎列表返回：",value)
      value.forEach(v => {
          v.label = v.engine_zh_name;
          v.value = v.engine_name;
      })
      engines.value = value;
      console.log("设置的引擎对象：",engines.value)
  })
}