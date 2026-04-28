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

const showResult = (res: string) => {
  let json = JSON.parse(res);
  if (currentEngineName.value == "baidu") {
    targetText.value = json.trans_result.map((item: any) => item.dst).join("  ");
    distLang.value = json.to;
  } else if (currentEngineName.value == "youdao") {
    targetText.value = json.translation.join(" ");
    distLang.value = json.l.split("2")[1];
  }
}

const changeEngine = () => {
  distLang.value = "";
  languageOptions.value = languages[currentEngineName.value];
  startTranslate();
}

const startTranslate = () => {
  invoke<string>("translate_selected_text",{engineName: currentEngineName.value,lang: distLang.value}).then(val => {
    console.log(val);
    showResult(val)
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