<script setup lang="ts">
import { onMounted, ref } from "vue";
import { Bottom, Plus, Top } from "@element-plus/icons-vue";
import { useRouter } from "vue-router";
import { openTunnel } from "@/services/tunnel";
import { invoke } from "@tauri-apps/api/core";
import { showError } from "@/services/message";
import HeadersEditor, { type Header } from "./HeadersEditor.vue";

const advanced = ref(false);
const port = ref(80);
const domain = ref("");
const host_rewrite = ref("");
const username = ref("");
const password = ref("");
const router = useRouter();
const staticDomains = ref<string[]>([]);
const isLoading = ref(false);
const requestHeaders = ref<Header[]>([]);
const responseHeaders = ref<Header[]>([]);

async function createTunnel() {
  try {
    isLoading.value = true;
    if (!advanced.value) {
      await openTunnel({ port: port.value.toString() });
    } else {
      await openTunnel({
        port: port.value.toString(),
        domain: domain.value,
        host_rewrite: host_rewrite.value,
        request_headers: requestHeaders.value.filter(
          (h) => h.name.trim() && h.value.trim(),
        ),
        response_headers: responseHeaders.value.filter(
          (h) => h.name.trim() && h.value.trim(),
        ),
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
  } catch (error: any) {
    showError(error);
  } finally {
    isLoading.value = false;
  }
}

async function getStaticDomains(): Promise<string[]> {
  return await invoke("get_static_domains");
}

onMounted(async () => {
  staticDomains.value = await getStaticDomains();
});
</script>

<template>
  <div class="w-full h-full flex items-center justify-center">
    <div
      class="w-1/3 h-full flex flex-col justify-start items-center content-center min-w-96"
    >
      <el-scrollbar class="w-full h-full">
        <div class="px-6 py-4">
          <h1 class="text-center text-2xl font-bold mb-6">Create Tunnel</h1>
          <el-input-number
            max="65535"
            min="0"
            class="mb-3 w-full"
            v-model="port"
            placeholder="Port"
          />
          <el-button
            class="my-4"
            :icon="advanced ? Top : Bottom"
            type="primary"
            @click="advanced = !advanced"
            text
          >
            {{ advanced ? "Hide advanced options" : "Show advanced options" }}
          </el-button>

          <el-collapse-transition>
            <div v-show="advanced" class="w-full">
              <el-select
                v-model="domain"
                filterable
                allow-create
                default-first-option
                placeholder="Domain (optional)"
                :reserve-keyword="false"
                class="mb-3 w-full"
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
                class="mb-3 w-full"
                type="text"
                v-model="host_rewrite"
                placeholder=" Host Rewrite (optional)"
              >
                <template #prefix>https://</template>
              </el-input>

              <el-divider />
              <el-text class="text-sm font-medium text-gray-700"
                >Basic Authentication</el-text
              >
              <el-input
                class="mb-3 mt-2 w-full"
                type="text"
                v-model="username"
                placeholder="Username"
              />
              <el-input
                class="mb-3 w-full"
                type="password"
                show-password
                v-model="password"
                placeholder="Password"
              />
              <el-divider />
              <el-text class="text-sm font-medium text-gray-700"
                >Request Headers</el-text
              >
              <div class="mb-4 mt-2">
                <HeadersEditor v-model="requestHeaders" :max-headers="9" />
              </div>

              <el-divider />
              <el-text class="text-sm font-medium text-gray-700"
                >Response Headers</el-text
              >
              <div class="mb-4 mt-2">
                <HeadersEditor v-model="responseHeaders" :max-headers="9" />
              </div>
            </div>
          </el-collapse-transition>
          <el-button
            class="mt-6 mb-4 w-full"
            type="primary"
            size="large"
            :icon="Plus"
            round
            @click="createTunnel"
            :loading="isLoading"
          >
            Create
          </el-button>
        </div>
      </el-scrollbar>
    </div>
  </div>
</template>
