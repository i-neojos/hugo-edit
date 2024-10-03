<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

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
  <p>hugo配置变更.</p>
  <textarea v-model="conf_data" rows="5" cols="30"></textarea>
  <button @click="update_data">获取数据</button>
</template>
