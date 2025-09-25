<script setup lang="js">
import CreateTunnel from "@/components/CreateTunnel.vue";
import { ArrowLeft } from "@element-plus/icons-vue";
</script>

<template>
  <div class="h-screen flex flex-col">
    <div class="flex justify-between w-full flex-shrink-0">
      <el-button
        :icon="ArrowLeft"
        tag="router-link"
        round
        plain
        :to="{ name: 'tunnel-list' }"
        class="m-5"
        >Back to tunnel list</el-button
      >
    </div>
    <div class="flex-1 overflow-hidden">
      <CreateTunnel />
    </div>
  </div>
</template>
