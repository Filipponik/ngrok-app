<script setup lang="ts">
import type { Tunnel } from "@/services/tunnel";

interface Props {
  row: Tunnel;
}

const props = defineProps<Props>();
</script>
<template>
  <div class="flex gap-2">
    <el-tag v-if="props.row.is_static_domain" type="primary"
      >Static domain</el-tag
    >
    <el-tag
      v-if="props.row.request_headers.find((header) => header.name === 'Host')"
      type="primary"
      >Host rewrite</el-tag
    >
    <el-tag v-if="props.row.basic_auth" type="primary">Basic auth</el-tag>
  </div>
</template>
