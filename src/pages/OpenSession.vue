<script setup lang="ts">
import { ref } from 'vue';
import { openSession } from '@/services/session';
import { useRouter } from 'vue-router';

const token = ref('');
const router = useRouter();

async function handleOpenSession() {
  console.log('Opening session...');
  openSession(token.value)
  .then(() => {
    router.go(-1);
    console.log('Session opened successfully');
  })
  .catch(() => {
    console.log('Failed to open session');
  });
}
</script>

<template>
   <el-input type="text" v-model="token" placeholder="Token" />
   <el-button type="primary" @click="handleOpenSession">Open Session</el-button>
</template>
