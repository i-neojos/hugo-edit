<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

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
    <p class="selected-value">{{ selectedValue }}</p>
    <select v-model="selectedValue"  class="dropdown-select">
      <option v-for="option in options" :value="option" :key="option">
        {{ option }}
      </option>
    </select>
  </div>
</template>


<style scoped>
.dropdown-container {
  display: flex;
  align-items: center; /* 居中对齐 */
}

.dropdown-select {
  padding: 8px;
  font-size: 16px;
  border: 1px solid #ccc;
  border-radius: 4px;
  margin-right: 8px; /* 添加一些间距 */
}

.selected-value {
  font-size: 16px;
  color: #333;
}

</style>