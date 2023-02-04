import axios from "axios"
import { defineStore } from "pinia"
import { computed, ref } from "vue"

export const useDeviceStore = defineStore('device', () => {
    const count = ref(0)
    const name = ref('Eduardo')
    const doubleCount = computed(() => count.value * 2)
    function increment() {
      count.value++
    }
    let devices=ref({})
    
    async function refresh(){
      devices.value=await (await axios.get("/api/devices/")).data;
    }

    return { count, devices, doubleCount, increment, refresh }
  })