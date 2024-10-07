<script setup lang="ts">

import { ref, watch } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const content = ref("");
const textmodel = ref(null);

async function fetchData() {
  ws.send("start_hugo")
}

var ws = new WebSocket('ws://127.0.0.1:3030/chat');

ws.onopen = () => {
  console.log('WebSocket connected');
  // 连接成功后可以发送消息
};

ws.onmessage = (event) => {
  content.value += event.data + '\n';
};

ws.onerror = (error) => {
  console.error('WebSocket error:', error);
};

ws.onclose = () => {
  console.log('WebSocket closed');
};

watch(content, (to_value, from_value ) => {
  textmodel.value.scrollTop = textmodel.value.scrollHeight;
});

async function close_hugo() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  await invoke("close_hugo");
}

</script>

<template>
  <div class="container">
    <p class="left-align" > 启动 hugo 博客</p>
    <textarea ref="textmodel" v-model="content" rows="5" class="custom-textarea"></textarea>
    <div>
      <button @click="fetchData" class="custom-button">启动博客</button>
      <button @click="close_hugo" class="custom-button">关闭博客</button>
    </div>
  </div>
</template>


<style scoped>
.custom-button {
  background-color: #4CAF50; /* 按钮背景色 */
  color: #fff; /* 按钮文字颜色 */
  border: none;
  border-radius: 5px;
  padding: 10px 20px;
  cursor: pointer;
  transition: all 0.3s ease-in-out; /* 添加过渡效果 */
  margin-top: 1%;
  width: 33%;
  margin-right: 1%;
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