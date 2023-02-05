<script setup lang="ts">
import { useDeviceStore } from "./stores/devices";
import ReloadPWA from "./components/ReloadPWA.vue";
import { RouterLink, RouterView } from 'vue-router'
import { onMounted } from "vue";

const store = useDeviceStore()
const { refresh } = store;

refresh();

onMounted(() => {
  setInterval(()=>{
    refresh();
  },1000)
})


</script>

<template>
  <ReloadPWA />
  <header>
    <div id="header_left">

    </div>
    <div id="header_middle">
      Home Center
    </div>
    <div id="header_right">

    </div>
  </header>
  <nav class="App__nav">
    <ul>
      <li><router-link to="/">Home</router-link></li>
    </ul>
  </nav>
  <router-view />
  <footer>
    &copy; Awesome Company
  </footer>
</template>
