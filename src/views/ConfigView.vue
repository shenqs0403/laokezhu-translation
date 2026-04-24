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
            <n-switch v-model:value="basic.swipe"/>
          </n-form-item>
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
      .catch(e => "");
}

onMounted(() => {
  loadAllEngines();
})

</script>
<style scoped>

</style>