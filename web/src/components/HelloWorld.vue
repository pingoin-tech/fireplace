<script setup lang="ts">
import { onMounted, ref } from 'vue'
import axios from "axios";
import { useDeviceStore } from "../stores/devices";
import { storeToRefs } from 'pinia';

defineProps<{ msg: string }>()
const store = useDeviceStore()
const { doubleCount } = storeToRefs(store);
const { increment, refresh } = store;

onMounted(async () => {
  refresh()
})

</script>

<template>
  <h1>{{ msg }}</h1>

  <div class="card">
    <button type="button" @click="increment">count is {{ doubleCount }}</button>
    <p>
      Edit
      <code>components/HelloWorld.vue</code> to test HMR
    </p>
  </div>

  <p>
    Check out
    <a href="https://vuejs.org/guide/quick-start.html#local" target="_blank">create-vue</a>, the official Vue + Vite
    starter
  </p>
  <p>
    Install
    <a href="https://github.com/johnsoncodehk/volar" target="_blank">Volar</a>
    in your IDE for a better DX
  </p>
  <p class="read-the-docs">Click on the Vite and Vue logos to learn more</p>
</template>

<style scoped>
.read-the-docs {
  color: #888;
}
</style>
