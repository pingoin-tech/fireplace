<script setup lang="ts">
import { Device, Shelly } from "../plugins/serverTypes";
import axios from "axios"

const props = defineProps<{
  dev: Device,
}>()

let shelly: Shelly | undefined;

if (props.dev.subdevice.type == "shelly") {
  shelly = props.dev.subdevice as Shelly
}

let action = (action: string) => {
  let data = { action_string: `${props.dev.id}/${action}` };
  axios.post("/api/trigger-action/", data);
}

</script>
<template>
  <tr>
    <td>
      {{ dev.id }}
    </td>
    <td><a :href="'http://' + dev.ip" target="_blank">{{ dev.ip }}</a></td>
    <td>{{ dev.rssi }}</td>
    <td>
      <ul>
        <li>
          typ: {{ dev.subdevice.type }}<span v-if="shelly">/{{ shelly.shelly_type }}</span>
        </li>
        <template v-if="shelly">
          <li v-if="shelly.rollers">
            Rollo: {{ shelly.rollers[0].current_pos }}
          </li>
          <li v-if="shelly.lights">
            licht {{ shelly.lights[0].ison }} <span v-if="shelly.lights[0].brightness">({{
              shelly.lights[0].brightness
            }})</span>
          </li>
        </template>

      </ul>
    </td>
    <td>
      <button v-for="actionstring in dev.available_actions" @click="action(actionstring)">{{ actionstring }}</button>
    </td>
  </tr>
</template>