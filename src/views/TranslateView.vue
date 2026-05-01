<script setup lang="ts">
import {onMounted, ref} from "vue";
import {invoke} from "@tauri-apps/api/core";
import {getCurrentWindow} from "@tauri-apps/api/window";
import {engines, loadAllEngines} from "../components/Common.ts";
import {LanguageOption, languages} from "../components/Translate.ts";
import {useMessage} from "naive-ui";

const message = useMessage();
const currentEngineName = ref("");
const targetText = ref("");
const distLang = ref("");
const languageOptions = ref<LanguageOption[]>([]);

const changeEngine = () => {
  distLang.value = "";
  languageOptions.value = languages[currentEngineName.value];
  startTranslate();
}

const startTranslate = () => {
  invoke<any>("translate_selected_text",{engineName: currentEngineName.value,targetLang: distLang.value,sourceLang: ""}).then(val => {
    targetText.value = val.target_text;
    distLang.value = val.target_lang;
  }).catch(e => message.error(e));
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
        currentEngineName.value = item.engine_name;
        languageOptions.value = languages[currentEngineName.value];
      }
    })
  })

  startTranslate();
})
</script>

<template>
  <div style="padding: 10px">
    <n-form inline label-placement="left">
      <n-form-item label="目标语言">
        <n-select v-model:value="distLang"
                  :options="languageOptions"
                  @update:value="startTranslate"
                  style="width: 100px"/>
      </n-form-item>
      <n-form-item label="翻译引擎">
        <n-select v-model:value="currentEngineName"
                  :options="engines"
                  @update:value="changeEngine"
                  style="width: 100px"/>
      </n-form-item>
    </n-form>
    {{targetText}}
  </div>
</template>

<style scoped>
#app .n-form--inline .n-form-item-blank {
  width: 100px;
}
</style>