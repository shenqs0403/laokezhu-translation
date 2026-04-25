<template>
  <div style="padding: 10px">
    <n-tabs>
      <n-tab-pane name="基础配置">
        <n-form label-width="auto">
          <n-form-item label="快捷键">
            <n-input v-model:value="basic.shortcut"
                     @keydown="loadShortcut"
                     @blur="saveShortcut"/>
          </n-form-item>
          <n-form-item label="划词">
            <n-switch v-model:value="basic.swipe" @update:value="saveSwipe"/>
          </n-form-item>
          <n-alert type="warning" :show-icon="false">
            快捷键和划词在Linux Wayland 环境无效，具体请看“关于”界面说明
          </n-alert>
        </n-form>
      </n-tab-pane>
      <n-tab-pane name="引擎配置">
        <n-select :options="engines"></n-select>
        <n-form style="margin-top: 10px">
          <n-form-item label="url">
            <n-input/>
          </n-form-item>
          <n-form-item label="appid">
            <n-input/>
          </n-form-item>
          <n-form-item label="密钥">
            <n-input/>
          </n-form-item>
          <n-button type="primary" block>保存</n-button>
        </n-form>
      </n-tab-pane>
    </n-tabs>
  </div>
</template>
<script setup lang="ts">

import {basic} from "../components/Config.ts";
import {engines, loadAllEngines} from "../components/Common.ts";
import {onMounted} from "vue";
import {invoke} from "@tauri-apps/api/core";
import {useMessage} from "naive-ui";

let message = useMessage();

// 快捷键的数组
let shortcutKeyArray:string[] = [];

const loadShortcut = (event: KeyboardEvent) => {
  console.log(shortcutKeyArray,"   ",shortcutKeyArray.indexOf(event.code),"   ",event.code)
  if (shortcutKeyArray.indexOf(event.code) < 0) {
    shortcutKeyArray.push(event.code);
    shortcutKeyArray.sort();
  }
  basic.value.shortcut = shortcutKeyArray.join("+")
      .replace("AltLeft", "Alt")
      .replace("AltRight", "AltGr");
}

const saveShortcut = () => {
  shortcutKeyArray = [];
  console.log(basic.value.shortcut)
  invoke<any>("save_key_value",{key: "basic.shortcut",value: basic.value.shortcut})
      .then(() => message.success("保存成功"))
      .catch(e => message.error(e));
}

const saveSwipe = () => {
  invoke("save_key_value",{key: "basic.swipe",value: basic.value.swipe +""})
      .then(() => message.success("保存成功"))
      .catch(e => message.error(e));
}

onMounted(() => {
  loadAllEngines();
  invoke<string>("get_key_value",{key: "basic.swipe"})
      .then(value => basic.value.swipe = value == "true")
      .catch(e => message.error(e));
  invoke<string>("get_key_value",{key: "basic.shortcut"})
      .then(value => basic.value.shortcut = value)
      .catch(e => message.error(e));
})

</script>
<style scoped>

</style>