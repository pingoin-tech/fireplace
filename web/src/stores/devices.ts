import axios from "axios"
import { defineStore } from "pinia"
import { computed, ref } from "vue"
import { Device } from "../plugins/serverTypes"

export const useDeviceStore = defineStore('device', () => {
  let devices = ref({} as Array<Device>)

  async function refresh() {
    devices.value = await (await axios.get<Array<Device>>("/api/devices")).data;
  }

  return { devices, refresh }
})