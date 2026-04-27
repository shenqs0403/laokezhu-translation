<script setup lang="ts">
import {onMounted} from "vue";
import {getCurrentWindow} from "@tauri-apps/api/window";
import {invoke} from "@tauri-apps/api/core";

onMounted(() => {
  let currentWindow = getCurrentWindow();
  currentWindow.once("tauri://blur",() => {
    currentWindow.close();
  });
})

const openTranslateWindow = () => {
  invoke("open_translate_window").catch(e => alert(e));
}

</script>

<template>
<div class="menu-root">
  <a @click="openTranslateWindow">翻译</a>
</div>
</template>

<style scoped>
.menu-root {
  background-color: #ededed;
  padding: 8px;
  text-decoration: underline;
  cursor: pointer;
}
</style>