<script setup lang="ts">
import { Plus } from "@element-plus/icons-vue";
import { onMounted } from "vue";
import { openSession } from "@/services/session";
import { useRouter } from "vue-router";

const router = useRouter();

onMounted(async () => {
  openSession().catch(() => {
    console.log("No session");
    router.push({ name: "open-session" });
  });
});
</script>

<template>
  <div class="w-full flex items-center justify-center h-screen">
    <div
      class="w-1/3 h-1/6 flex flex-col justify-center items-center content-center"
    >
      <el-button
        size="large"
        :icon="Plus"
        tag="router-link"
        round
        style="width: 240px"
        type="primary"
        to="/tunnel/create"
        class="m-5"
        >Open your first tunnel 🚀</el-button
      >
    </div>
  </div>
</template>
