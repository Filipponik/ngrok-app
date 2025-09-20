<script setup lang="ts">
import {
  getTunnels as getTunnelsFromApi,
  type Tunnel,
} from "@/services/tunnel";
import { Plus } from "@element-plus/icons-vue";
import { onMounted, ref } from "vue";
import { openPath } from "@tauri-apps/plugin-opener";
import TunnelListTable from "@/components/TunnelListTable.vue";

const tunnels = ref<Tunnel[]>([]);

async function getTunnels() {
  tunnels.value = await getTunnelsFromApi();
}

onMounted(async () => {
  await getTunnels();
  setInterval(getTunnels, 500);
});
</script>

<template>
  <div class="flex justify-end w-full">
    <el-button
      :icon="Plus"
      tag="router-link"
      round
      type="primary"
      plain
      to="/tunnel/create"
      class="m-5"
      >Create Tunnel</el-button
    >
  </div>

  <div class="mx-2 my-3">
    <TunnelListTable :tunnels="tunnels" @tunnel-closed="getTunnels()" />
  </div>
</template>
