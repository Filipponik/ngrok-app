<script setup lang="ts">
import { ref, watch, computed } from "vue";
import { Plus, Delete } from "@element-plus/icons-vue";

export interface Header {
  name: string;
  value: string;
}

interface Props {
  modelValue: Header[];
  maxHeaders?: number;
  placeholder?: {
    name?: string;
    value?: string;
  };
}

const props = withDefaults(defineProps<Props>(), {
  maxHeaders: 9,
  placeholder: () => ({
    name: "Name",
    value: "Value",
  }),
});

const emit = defineEmits<{
  "update:modelValue": [headers: Header[]];
}>();

const initializeHeaders = (headers: Header[]): Header[] => {
  if (headers.length === 0) {
    return [{ name: "", value: "" }];
  }
  return headers;
};

const headers = ref<Header[]>(initializeHeaders([...props.modelValue]));

const currentCount = computed(() => headers.value.length);

watch(
  headers,
  (newHeaders) => {
    emit("update:modelValue", newHeaders);
  },
  { deep: true },
);

watch(
  () => props.modelValue,
  (newValue) => {
    const currentNormalized = headers.value.map((h) => ({
      name: h.name.trim(),
      value: h.value.trim(),
    }));
    const newNormalized = newValue.map((h) => ({
      name: h.name.trim(),
      value: h.value.trim(),
    }));

    const isDifferent =
      currentNormalized.length !== newNormalized.length ||
      currentNormalized.some(
        (h, i) =>
          h.name !== newNormalized[i]?.name ||
          h.value !== newNormalized[i]?.value,
      );

    if (isDifferent) {
      headers.value = initializeHeaders([...newValue]);
    }
  },
  { deep: true },
);

function addHeader() {
  if (headers.value.length < props.maxHeaders) {
    headers.value.push({ name: "", value: "" });
  }
}

function removeHeader(index: number) {
  if (headers.value.length > 1) {
    headers.value.splice(index, 1);
  }
}

const isAddButtonDisabled = computed(() => {
  return headers.value.length >= props.maxHeaders;
});

const isRemoveButtonDisabled = computed(() => {
  return headers.value.length <= 1;
});
</script>

<template>
  <div class="headers-editor">
    <div
      v-for="(header, index) in headers"
      :key="index"
      class="header-row mb-2 flex items-center gap-2"
    >
      <el-input
        v-model="header.name"
        :placeholder="placeholder.name"
        class="flex-1"
      />
      <el-input
        v-model="header.value"
        :placeholder="placeholder.value"
        class="flex-1"
      />
      <el-button
        :icon="Delete"
        circle
        plain
        type="danger"
        size="small"
        :disabled="isRemoveButtonDisabled"
        @click="removeHeader(index)"
      />
    </div>

    <div class="flex justify-center mt-2">
      <el-button
        :icon="Plus"
        type="primary"
        plain
        size="small"
        :disabled="isAddButtonDisabled"
        @click="addHeader"
      >
        Add Header ({{ currentCount }}/{{ maxHeaders }})
      </el-button>
    </div>
  </div>
</template>
