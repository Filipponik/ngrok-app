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
  <main class="container">
    <div class="row my-3">
      <el-input
        style="width: 240px"
        type="text"
        v-model="token"
        placeholder="Token"
      />
    </div>
    <div class="row">
      <el-input
        style="width: 240px"
        type="text"
        v-model="domain"
        placeholder="Domain"
      />
    </div>
    <div class="row">
      <el-input
        style="width: 240px"
        type="number"
        v-model="port"
        placeholder="Port"
      />
    </div>
    <div class="row">
      <el-button
        type="primary"
        size="large"
        :icon="Plus"
        tag="router-link"
        round
        style="width: 240px"
        @click="createTunnel"
        >Create Tunnel</el-button
      >
    </div>
  </main>
</template>
