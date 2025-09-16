<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { Plus } from "@element-plus/icons-vue";
import { useRouter } from "vue-router";

const port = ref("");
const domain = ref("");
const router = useRouter();

async function createTunnel() {
  await invoke("tunnel_open", {
    port: port.value,
    domain: domain.value,
  });
  router.push({ name: "tunnel-list" });
}
</script>

<template>
  <div class="w-full flex items-center justify-center h-screen">
    <div
      class="w-1/3 h-1/6 flex flex-col justify-center items-center content-center"
    >
      <h1 class="text-center text-large font-600 font-bold mb-4">
        Create Tunnel
      </h1>
      <el-input class="mb-1" type="number" v-model="port" placeholder="Port" />
      <el-input
        class="mb-1"
        type="text"
        v-model="domain"
        placeholder="Domain (optional)"
      >
        <template #prepend>https://</template>
      </el-input>
      <el-button
        class="my-4 w-full"
        type="primary"
        size="large"
        :icon="Plus"
        round
        @click="createTunnel"
        >Create</el-button
      >
    </div>
  </div>
</template>
