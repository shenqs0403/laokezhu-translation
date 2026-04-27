<script setup lang="ts">
import {onMounted, ref} from "vue";
import {invoke} from "@tauri-apps/api/core";
import {getCurrentWindow} from "@tauri-apps/api/window";

const targetText = ref("");

onMounted(() => {
  let currentWindow = getCurrentWindow();
  // currentWindow.once("tauri://blur",() => {
  //   currentWindow.close();
  // });

  invoke<string>("translate_selected_text",{engineName: "",lang: ""}).then(val => {
    console.log(val);
    targetText.value = val;
  }).catch(e => alert(e));
})
</script>

<template>
{{ targetText }}
</template>

<style scoped>

</style>