<script setup lang="ts">
import { Check, Close, CopyDocument } from "@element-plus/icons-vue";
import { writeText } from "@tauri-apps/plugin-clipboard-manager";
import { ref } from "vue";
import {
  closeTunnel as closeTunnelFromApi,
  type Tunnel,
} from "@/services/tunnel";

const emit = defineEmits(["tunnelClosed"]);

interface Props {
  row: Tunnel;
}

const props = defineProps<Props>();
const isCopied = ref(false);
const isClosing = ref(false);

async function copyUrl() {
  await writeText(props.row.url);

  isCopied.value = true;
  setTimeout(() => {
    isCopied.value = false;
  }, 1000);
}

async function closeTunnel() {
  isClosing.value = true;
  try {
    await closeTunnelFromApi(props.row.id);
    emit("tunnelClosed");
  } finally {
    isClosing.value = false;
  }
}
</script>

<template>
  <el-button
    :type="isCopied ? 'success' : 'primary'"
    :icon="isCopied ? Check : CopyDocument"
    circle
    plain
    @click="copyUrl()"
  />
  <el-button
    type="danger"
    :icon="Close"
    circle
    plain
    :loading="isClosing"
    @click="closeTunnel()"
  />
</template>
