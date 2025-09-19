<script setup lang="ts">
import {
  getTunnels as getTunnelsFromApi,
  type Tunnel,
} from "@/services/tunnel";
import { Plus } from "@element-plus/icons-vue";
import { onMounted, ref } from "vue";
import { openPath } from "@tauri-apps/plugin-opener";
import TunnelListActionButtons from "@/components/TunnelListActionButtons.vue";
import TunnelListTags from "@/components/TunnelListTags.vue";

const tunnels = ref<Tunnel[]>([]);

async function getTunnels() {
  tunnels.value = await getTunnelsFromApi();
}

async function openUrl(url: string) {
  await openPath(url);
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
    <el-table :data="tunnels" stripe table-layout="auto">
      <el-table-column prop="port" label="local port">
        <template #default="scope">
          <el-link
            type="primary"
            @click="openUrl(`http://localhost:${scope.row.port}`)"
          >
            {{ scope.row.port }}
          </el-link>
        </template>
      </el-table-column>

      <el-table-column prop="url" label="url">
        <template #default="scope">
          <el-link type="primary" @click="openUrl(scope.row.url)">
            {{ scope.row.url.replace("https://", "") }}
          </el-link>
        </template>
      </el-table-column>

      <el-table-column prop="tags" label="tags">
        <template #default="scope">
          <TunnelListTags :row="scope.row" />
        </template>
      </el-table-column>

      <el-table-column align="right">
        <template #default="scope">
          <TunnelListActionButtons
            :row="scope.row"
            @tunnel-closed="getTunnels()"
          />
        </template>
      </el-table-column>
    </el-table>
  </div>
</template>
