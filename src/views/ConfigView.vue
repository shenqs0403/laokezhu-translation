<template>
<div style="padding: 10px">
  <n-tabs>
    <n-tab-pane name="基础配置">
      <n-form label-width="auto">
        <n-form-item label="快捷键">
          <n-input v-model:value="basic.shortcut"/>
        </n-form-item>
        <n-form-item label="划词">
          <n-switch v-model:value="basic.swipe"/>
        </n-form-item>
      </n-form>
    </n-tab-pane>
    <n-tab-pane name="引擎配置">
      <n-select ></n-select>
      <n-form style="margin-top: 10px">
        <n-form-item label="url">
          <n-input />
        </n-form-item>
        <n-form-item label="appid">
          <n-input />
        </n-form-item>
        <n-form-item label="密钥">
          <n-input />
        </n-form-item>
        <n-button type="primary" block>保存</n-button>
      </n-form>
    </n-tab-pane>
  </n-tabs>
</div>
</template>
<script setup lang="ts">

import {basic} from "../components/Config.ts";
</script>
<style scoped>

</style>