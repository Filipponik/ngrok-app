<script setup lang="js">
import CreateTunnel from "@/components/CreateTunnel.vue";
import { ArrowLeft } from "@element-plus/icons-vue";
</script>

<template>
  <div class="flex justify-between w-full">
    <el-button
      :icon="ArrowLeft"
      tag="router-link"
      round
      plain
      :to="{ name: 'tunnel-list' }"
      class="m-5"
      >Back</el-button
    >
  </div>
  <CreateTunnel />
</template>
