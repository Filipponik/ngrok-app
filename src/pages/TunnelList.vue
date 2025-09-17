<script setup lang="ts">
import {
  closeTunnel as closeTunnelFromApi,
  getTunnels as getTunnelsFromApi,
  type Tunnel,
} from "@/services/tunnel";
import { Check, Close, CopyDocument, Plus } from "@element-plus/icons-vue";
import { onMounted, reactive, ref } from "vue";
import { writeText } from "@tauri-apps/plugin-clipboard-manager";
import { openPath } from "@tauri-apps/plugin-opener";

const tunnels = ref<Tunnel[]>([]);
const copyStates = reactive<{ [id: string]: boolean }>({})

async function getTunnels() {
  tunnels.value = await getTunnelsFromApi();
}

setTimeout(async () => {
  await getTunnels();
}, 1000);

async function copyUrl(url: string, id: string) {
  await writeText(url);

  copyStates[id] = true
  setTimeout(() => {
    copyStates[id] = false
  }, 1000)
}

async function openUrl(url: string) {
  await openPath(url);
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
    <el-table :data="tunnels" stripe>
      <!-- <el-table-column prop="id" label="id" /> -->
      <el-table-column prop="url" label="url">
        <template #default="scope">
          <el-link type="primary" @click="openUrl(scope.row.url)">
            {{ scope.row.url }}
          </el-link>
        </template>
      </el-table-column>
      <el-table-column align="right">
        <template #default="scope">
          <el-button
          :type="copyStates[scope.row.id] ? 'success' : 'primary'"
          :icon="copyStates[scope.row.id] ? Check : CopyDocument"
            circle
            plain
            @click="copyUrl(scope.row.url, scope.row.id)"
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
