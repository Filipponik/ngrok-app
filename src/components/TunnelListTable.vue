<script setup lang="ts">
import TunnelListActionButtons from "@/components/TunnelListActionButtons.vue";
import TunnelListTags from "@/components/TunnelListTags.vue";
interface Props {
  tunnels: Tunnel[];
}

const emit = defineEmits(["tunnelClosed"]);
const props = defineProps<Props>();

async function openUrl(url: string) {
  await openPath(url);
}
</script>

<template>
  <el-table :data="props.tunnels" stripe table-layout="auto">
    <el-table-column prop="port" label="local port">
      <template #default="scope">
        <el-link
          type="primary"
          @click="openUrl(`http://localhost:${scope.row.port}`)"
        >
          {{ scope.row.port }}
        </el-link>
      </template>
    </el-table-column>

    <el-table-column prop="url" label="url">
      <template #default="scope">
        <el-link type="primary" @click="openUrl(scope.row.url)">
          {{ scope.row.url.replace("https://", "") }}
        </el-link>
      </template>
    </el-table-column>

    <el-table-column prop="tags" label="tags">
      <template #default="scope">
        <TunnelListTags :row="scope.row" />
      </template>
    </el-table-column>

    <el-table-column align="right">
      <template #default="scope">
        <TunnelListActionButtons
          :row="scope.row"
          @tunnel-closed="emit('tunnelClosed', scope.row.id)"
        />
      </template>
    </el-table-column>
  </el-table>
</template>
