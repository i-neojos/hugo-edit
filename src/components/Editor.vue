<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";

const options = ref([]);
const selectedValue = ref(null);

onMounted(async () => {
  try {
    console.log("start call")
    options.value = await invoke('get_applications');
    if (options.value.length >0) {
      selectedValue.value = options.value[0];
    }
  } catch (error) {
    console.error('Error fetching options:', error);
  }
});

</script>

<template>
  <div class="dropdown-container">
    <p class="selected-value">选择使用应用：「{{ selectedValue }}」 打开博客</p>
    <select v-model="selectedValue" class="dropdown-select">
      <option v-for="option in options" :value="option" :key="option">
        {{ option }}
      </option>
    </select>
  </div>
</template>

<style scoped>
.dropdown-container {
  background-color: #fff;
  border: 1px solid #ccc;
  border-radius: 5px;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  padding: 10px;
}

.dropdown-select {
  -webkit-appearance: none; /* 隐藏默认样式 */
  -moz-appearance: none;
  appearance: none;
  width: 100%;
  padding: 8px 15px;
  border: none;
  border-radius: 4px;
  background-color: #f2f2f2;
  cursor: pointer;
  transition: all 0.2s ease-in-out;
}

.dropdown-select:hover {
  background-color: #e8e8e8;
}

.dropdown-select option {
  padding: 10px;
}

.dropdown-select option:hover {
  background-color: #e0e0e0;
}
</style>