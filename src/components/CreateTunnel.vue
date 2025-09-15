<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { Plus } from "@element-plus/icons-vue";

const token = ref("");
const domain = ref("");
const port = ref("");

async function createTunnel() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  await invoke("create_tunnel", {
    authToken: token.value,
    domain: domain.value,
    port: port.value,
  });
}
</script>

<template>
  <main class="w-full flex justify-center h-full">
    <div class="w-1/3 h-1/6 flex flex-col justify-center items-center">
      <div class="w-full my-2">
        <el-input type="text" v-model="token" placeholder="Token" />
      </div>
      <div class="w-full my-2">
        <el-input type="text" v-model="domain" placeholder="Domain" />
      </div>
      <div class="w-full my-2">
        <el-input type="number" v-model="port" placeholder="Port" />
      </div>
      <div>
        <el-button
          type="primary"
          size="large"
          :icon="Plus"
          tag="router-link"
          round
          @click="createTunnel"
          >Create Tunnel</el-button
        >
      </div>
    </div>
  </main>
</template>
