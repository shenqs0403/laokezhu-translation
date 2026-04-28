<script setup lang="ts">

import {version} from "../../package.json";
</script>

<template>
  当前版本 {{ version}}<br>
  本软件主要是适配Linux，Windows和Mac系统建议使用官方提供的软件
  <h3>Linux 快捷键</h3>
  <b>以Debian13 Gnome 为例</b><br>
  系统设置 -> 键盘 -> 查看及自定义快捷键 -> 自定义快捷键 <br>
  名称：自定义，随便写<br>
  命令： laokezhu-translation -- translate<br>
  快捷键： 自定义，设置成自己喜欢的<br>
</template>

<style scoped>

</style>