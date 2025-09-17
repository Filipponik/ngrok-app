<script setup lang="ts">
import { ref } from "vue";
import { Bottom, Plus, Top } from "@element-plus/icons-vue";
import { useRouter } from "vue-router";
import { openTunnel } from "@/services/tunnel";

const advanced = ref(false);
const port = ref("");
const domain = ref("");
const host_rewrite = ref("");
const router = useRouter();

async function createTunnel() {
  if (!advanced.value) {
    await openTunnel(port.value);
  } else {
    await openTunnel(port.value, domain.value, host_rewrite.value);
  }
  router.push({ name: "tunnel-list" });
}
</script>

<template>
  <div class="w-full flex items-center justify-center">
    <div
      class="w-1/3 h-1/6 flex flex-col justify-center items-center content-center"
    >
      <h1 class="text-center text-large font-600 font-bold mb-4">
        Create Tunnel
      </h1>
      <el-input class="mb-1" type="number" v-model="port" placeholder="Port" />
      <el-button
        class="my-3"
        :icon="advanced ? Top : Bottom"
        type="primary"
        @click="advanced = !advanced"
        text
      >
        {{
          advanced ? "Hide advanced options" : "Show advanced options"
        }}</el-button
      >
      <div v-show="advanced">
        <el-input
          class="mb-1"
          type="text"
          v-model="domain"
          placeholder="Domain (optional)"
        >
          <template #prepend>https://</template>
        </el-input>
        <el-input
          class="mb-1"
          type="text"
          v-model="host_rewrite"
          placeholder="Host Rewrite (optional)"
        >
          <template #prepend>https://</template>
        </el-input>
      </div>
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
