<script setup lang="ts">
import { onMounted } from 'vue'
import { useDeviceStore } from "../stores/devices";
import { storeToRefs } from 'pinia';
import DeviceView from '../components/DeviceView.vue';

const store = useDeviceStore()
const { devices } = storeToRefs(store);
const { refresh } = store;

onMounted(async () => {
  refresh()
})

</script>

<template>
  <main>
    <article>
    <h1>Devices</h1>
    <table>
      <thead>
        <tr>
          <th>id</th>
          <th>ip</th>
          <th>sub-device</th>
          <th>actions</th>
        </tr>
      </thead>
      <tbody>
        <DeviceView v-for="device in devices" :dev="device" />
      </tbody>
    </table>
  </article>
  </main>
</template>
