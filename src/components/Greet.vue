<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

const conf_data = ref("");

async function load_data() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  conf_data.value = await invoke("load_conf");
}

async function update_data() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  conf_data.value = await invoke("update_conf", { content: conf_data.value });
}

load_data();
</script>

<template>
  <div class="container">
    <p class="left-align">hugo配置变更</p>
    <textarea v-model="conf_data" rows="5" class="custom-textarea"></textarea>
    <button @click="update_data" class="custom-button">获取数据</button>
  </div>
</template>

<style scoped>
.custom-button {
  margin-top: 1%;
  background-color: #4CAF50; /* 按钮背景色 */
  color: #fff; /* 按钮文字颜色 */
  border: none;
  border-radius: 5px;
  padding: 10px 20px;
  cursor: pointer;
  transition: all 0.3s ease-in-out; /* 添加过渡效果 */
}

.custom-button:hover {
  background-color: #3e8e41; /* 鼠标悬停时背景色变深 */
  box-shadow: 0px 5px 10px rgba(0, 0, 0, 0.2); /* 添加阴影效果 */
}

.left-align {
  text-align: left;
}

.container {
  width: 100%; /* 容器占满整个页面 */
}

.custom-textarea {
  width: 100%; /* textarea 占满容器宽度 */
  height: 33vh; /* 占据页面高度的 33% */
  border: none; /* 去除默认边框 */
  border-radius: 5px; /* 添加圆角 */
  padding: 10px;
  font-family: 'Consolas', monospace; /* 设置字体，适合代码输入 */
  font-size: 16px;
  box-shadow: 0 2px 5px rgba(0, 0, 0, 0.1); /* 添加阴影 */
  resize: vertical; /* 允许垂直缩放 */
  overflow: auto; /* 当内容超出时出现滚动条 */
  background-color: #f2f2f2; /* 设置背景色 */
}
</style>