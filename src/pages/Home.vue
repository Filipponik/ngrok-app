<script setup lang="ts">
import { Plus } from "@element-plus/icons-vue";
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

const greetMsg = ref("");
const name = ref("");

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  greetMsg.value = await invoke("greet", { name: name.value });
}
</script>

<template>
  <main class="container">
    <h1>Welcome to Tauri + Vue</h1>

    <p>
      <el-text class="mx-1"
        >Click on the Tauri, Vite, and Vue logos to learn more.</el-text
      >
    </p>

    <div class="row">
      <el-input
        v-model="name"
        style="width: 240px"
        placeholder="Enter a name.."
      />
      <el-button @click="greet">Greet 👋</el-button>
    </div>
    <p>{{ greetMsg }}</p>
    <div class="row">
      <el-button
        size="large"
        :icon="Plus"
        tag="router-link"
        round
        style="width: 240px"
        type="primary"
        to="/create-tunnel"
        >Create Tunnel</el-button
      >
    </div>
  </main>
</template>
