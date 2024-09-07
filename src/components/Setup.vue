<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const content = ref("");

async function fetchData() {
  try {
    const result = await invoke('dump_content');
    content.value = result
  } catch (error) {
    console.error('Error:', error);
    // 在这里处理错误，例如显示错误信息给用户
  }
}

</script>

<template>
  <p> 启动 hugo 博客</p>
  <textarea v-model="content" rows="5" cols="30"></textarea>
  <button @click="fetchData">获取数据</button>
</template>
