<script setup lang="ts">
import {onMounted, ref} from "vue";
import {invoke} from "@tauri-apps/api/core";
import {getCurrentWindow} from "@tauri-apps/api/window";
import {Engine, loadAllEngines} from "../components/Common.ts";

var currentEngine: Engine = {appid: "", enable: false, engine_key: "", engine_name: "", engine_zh_name: "", url: ""};
const targetText = ref("");

const showResult = (res: string) => {
  let json = JSON.parse(res);
  if (currentEngine.engine_name == "baidu") {
    targetText.value = json.trans_result.map((item: any) => item.dst).join("  ");
  }
}

onMounted(() => {
  let currentWindow = getCurrentWindow();
  currentWindow.once("tauri://blur",() => {
    currentWindow.close();
  });

  loadAllEngines().then(engineArr => {
    engineArr.forEach(item => {
      console.log(item,"    ",item.enable)
      if (item.enable) {
        currentEngine = item;
      }
    })
  })

  invoke<string>("translate_selected_text",{engineName: "",lang: ""}).then(val => {
    console.log(val);
    showResult(val)
  }).catch(e => alert(e));
})
</script>

<template>
{{ targetText }}
</template>

<style scoped>

</style>