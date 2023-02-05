<script setup lang="ts">
import { onMounted } from 'vue'
import { useDeviceStore } from "../stores/devices";
import { storeToRefs } from 'pinia';
import DeviceView from '../components/DeviceView.vue';

defineProps<{ msg: string }>()
const store = useDeviceStore()
const { devices } = storeToRefs(store);
const { refresh } = store;

onMounted(async () => {
  refresh()
})

</script>

<template>
  <main>
    <h1>{{ msg }}</h1>
    <DeviceView v-for="device in devices" :dev="device" />
  </main>
</template>

<style scoped>
.read-the-docs {
  color: #888;
}
</style>
