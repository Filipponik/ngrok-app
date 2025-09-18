<script setup lang="ts">
import { ref } from "vue";
import { openSession } from "@/services/session";
import { useRouter } from "vue-router";
import { showError } from "@/services/message";

const token = ref("");
const router = useRouter();

async function handleOpenSession() {
  console.log("Opening session...");
  openSession(token.value)
    .then(() => {
      console.log("Session opened successfully");
      router.push({ name: "home" });
    })
    .catch((err) => {
      showError(err);
      console.log("Failed to open session");
    });
}
</script>

<template>
  <div class="w-full flex items-center justify-center h-screen">
    <div
      class="w-1/3 h-1/6 flex flex-col justify-center items-center content-center"
    >
      <div class="my-2">
        <el-text bold tag="b" size="large">Welcome to Ngrok App!</el-text>
      </div>
      <el-text>Please enter your Ngrok API Token</el-text>
      <el-input
        type="password"
        show-password
        class="my-3"
        v-model="token"
        placeholder="Ngrok API Token"
      />
      <el-button type="primary" class="my-3" @click="handleOpenSession" round
        >Open Session</el-button
      >
    </div>
  </div>
</template>
