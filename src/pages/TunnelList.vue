<script setup lang="ts">
import {
  closeTunnel as closeTunnelFromApi,
  getTunnels as getTunnelsFromApi,
  type Tunnel,
} from "@/services/tunnel";
import { ArrowLeft, Close, CopyDocument, Plus } from "@element-plus/icons-vue";
import { onMounted, ref } from "vue";
import { writeText } from "@tauri-apps/plugin-clipboard-manager";

const tunnels = ref<Tunnel[]>([]);

async function getTunnels() {
  tunnels.value = await getTunnelsFromApi();
}

setTimeout(async () => {
  await getTunnels();
}, 1000);

async function copyUrl(url: string) {
  await writeText(url);
}

async function closeTunnel(id: string) {
  await closeTunnelFromApi(id);
  await getTunnels();
}

onMounted(async () => {
  await getTunnels();
});

</script>

<template>
  <div class="flex justify-between w-full">
    <el-button
      :icon="ArrowLeft"
      tag="router-link"
      round
      plain
      to="/tunnel/create"
      class="m-5"
      >Back</el-button
    >
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
    <el-table :data="tunnels" stripe class="">
      <!-- <el-table-column prop="id" label="id" /> -->
      <el-table-column prop="url" label="url" />
      <el-table-column align="right">
        <template #default="scope">
          <el-button
            type="primary"
            :icon="CopyDocument"
            circle
            plain
            @click="copyUrl(scope.row.url)"
          />
          <el-button
            type="danger"
            :icon="Close"
            circle
            plain
            @click="closeTunnel(scope.row.id)"
          />
        </template>
      </el-table-column>
    </el-table>
  </div>
</template>
