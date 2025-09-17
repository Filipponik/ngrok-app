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
  <main class="container">
    <div class="row">
      <el-button
        size="large"
        :icon="Plus"
        tag="router-link"
        round
        style="width: 240px"
        type="primary"
        to="/tunnel/create"
        class="m-5"
        >Create Tunnel</el-button
      >
    </div>
  </main>
</template>
