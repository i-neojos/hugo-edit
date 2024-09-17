<script setup lang="ts">
import { ref } from "vue";

const content = ref("");

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

</script>

<template>
  <p> 启动 hugo 博客</p>
  <textarea v-model="content" rows="5" cols="30"></textarea>
  <button @click="fetchData">获取数据</button>
</template>
