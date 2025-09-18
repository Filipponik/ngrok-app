<script setup lang="ts">
import { onMounted, ref } from "vue";
import { Bottom, Plus, Top } from "@element-plus/icons-vue";
import { useRouter } from "vue-router";
import { openTunnel } from "@/services/tunnel";
import { invoke } from "@tauri-apps/api/core";

const advanced = ref(false);
const port = ref("");
const domain = ref("");
const host_rewrite = ref("");
const username = ref("");
const password = ref("");
const router = useRouter();
const staticDomains = ref<string[]>([]);

async function createTunnel() {
  if (!advanced.value) {
    await openTunnel({ port: port.value });
  } else {
    await openTunnel({
      port: port.value,
      domain: domain.value,
      host_rewrite: host_rewrite.value,
      basic_auth:
        username.value && password.value
          ? {
              username: username.value,
              password: password.value,
            }
          : undefined,
    });
  }
  router.push({ name: "tunnel-list" });
}

async function getStaticDomains(): Promise<string[]> {
  return await invoke("get_static_domains");
}

onMounted(async () => {
  staticDomains.value = await getStaticDomains();
});
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

      <el-collapse-transition>
        <div v-show="advanced">
          <el-select
            v-model="domain"
            filterable
            allow-create
            default-first-option
            placeholder="Domain (optional)"
            :reserve-keyword="false"
            class="mb-1"
          >
            <template #prefix>https://</template>
            <el-option
              v-for="item in staticDomains"
              :key="item"
              :label="item"
              :value="item"
            />
          </el-select>
          <el-input
            class="mb-1"
            type="text"
            v-model="host_rewrite"
            placeholder=" Host Rewrite (optional)"
          >
            <template #prefix>https://</template>
          </el-input>
          <el-divider />
          <el-text>Basic Authentication</el-text>
          <el-input
            class="mb-1"
            type="text"
            v-model="username"
            placeholder="Username"
          />
          <el-input
            class="mb-1"
            type="password"
            v-model="password"
            placeholder="Password"
            show-password
          />
        </div>
      </el-collapse-transition>
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
