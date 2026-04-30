<template>
  <div style="padding: 10px">
    <n-tabs>
      <n-tab-pane name="基础配置">
        <n-form label-width="auto">
          <n-form-item label="快捷键" v-show="!isLinux">
            <n-input v-model:value="basic.shortcut"
                     :disabled="isWayland"
                     @keydown="loadShortcut"
                     @blur="saveShortcut"/>
          </n-form-item>
<!--          <n-form-item label="轮询（毫秒）">-->
<!--            <n-input v-model:value="basic.swipe"-->
<!--              @blur="saveSwipe"-->
<!--            />-->
<!--          </n-form-item>-->
          <n-alert type="warning" :show-icon="false">
            Linux系统快捷键需要在系统设置中自行设置。<br>
            启动翻译窗口的命令是： <br>
            laokezhu-translation -- translate
          </n-alert>
        </n-form>
      </n-tab-pane>
      <n-tab-pane name="引擎配置">
        <n-select v-model:value="currentEngine.engine_name"
                  :options="engines"
                  @update:value="engineSelectChangeHandler"
        />
        <n-form style="margin-top: 10px">
          <n-form-item label="url">
            <n-input v-model:value="currentEngine.url"/>
          </n-form-item>
          <n-form-item label="区域" v-if="currentEngine.engine_name == 'tencent'">
            <n-select v-model:value="currentEngine.region" :options="TENCENT_REGIONS"/>
          </n-form-item>
          <n-form-item label="appid">
            <n-input v-model:value="currentEngine.appid"/>
          </n-form-item>
          <n-form-item label="密钥">
            <n-input v-model:value="currentEngine.engine_key"/>
          </n-form-item>
          <n-form-item label="是否启用">
            <n-switch v-model:value="currentEngine.enable"/>
          </n-form-item>
          <n-button type="primary" block @click="saveEngine">保存</n-button>
        </n-form>
      </n-tab-pane>
    </n-tabs>
  </div>
</template>
<script setup lang="ts">

import {basic, currentEngine, TENCENT_REGIONS} from "../components/Config.ts";
import {engines, loadAllEngines} from "../components/Common.ts";
import {onMounted, ref} from "vue";
import {invoke} from "@tauri-apps/api/core";
import {useMessage} from "naive-ui";

let message = useMessage();

const isLinux = navigator.userAgent.indexOf("Linux") > 0;
// 快捷键的数组
let shortcutKeyArray: string[] = [];
// 是否是Wayland,如果是禁用快捷键、swipe
const isWayland = ref(false);

const loadShortcut = (event: KeyboardEvent) => {
  console.log(shortcutKeyArray, "   ", shortcutKeyArray.indexOf(event.code), "   ", event.code)
  if (shortcutKeyArray.indexOf(event.code) < 0) {
    shortcutKeyArray.push(event.code);
    shortcutKeyArray.sort();
  }
  basic.value.shortcut = shortcutKeyArray.join("+")
      .replace(/Left/g,"")
      .replace("/Right/g","");
}

const saveShortcut = () => {
  shortcutKeyArray = [];
  invoke<any>("update_shortcut", {value: basic.value.shortcut})
      .then(() => message.success("保存成功"))
      .catch(e => message.error(e));
}

// const saveSwipe = () => {
//   if (basic.value.swipe < 0) {
//     basic.value.swipe = 0;
//   }
//   invoke("update_swipe", {value: basic.value.swipe + ""})
//       .then(() => message.success("保存成功"))
//       .catch(e => message.error(e));
// }

const engineSelectChangeHandler = (val: string) => {
  engines.value?.forEach((item) => {
    if (item.engine_name == val) {
      currentEngine.value = item;
      console.log(currentEngine.value);
    }
  })
}

const saveEngine = () => {
  invoke<number>("save_engine", {engine: currentEngine.value})
      .then(i => {
        message.success(i > 0 ? "保存成功" : "保存失败");
        loadAllEngines();
      })
      .catch(e => message.error(e));
}

onMounted(() => {
  console.log(navigator.userAgent);
  invoke<boolean>("is_wayland").then(value => {
    isWayland.value = value;
  })

  loadAllEngines();
  invoke<string>("get_key_value", {key: "basic.swipe"})
      .then(value => basic.value.swipe = parseInt(value))
      .catch(e => message.error(e));
  invoke<string>("get_key_value", {key: "basic.shortcut"})
      .then(value => basic.value.shortcut = value)
      .catch(e => message.error(e));
})

</script>
<style scoped>

</style>