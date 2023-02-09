<script setup lang="ts">
import { Device, Shelly, EventType } from "../plugins/serverTypes";
import axios from "axios"

const props = defineProps<{
  dev: Device,
}>()

let shelly: Shelly | undefined;

if (props.dev.subdevice.type == "shelly") {
  shelly = props.dev.subdevice as Shelly
}

let action = (action: string) => {
  let data: EventType = {
    id: props.dev.id,
    action: action
  }

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
        <template v-for="devValue, index in dev.values">
          <li v-if="devValue.type == 'string'">
            {{ index }}: {{ devValue.val }}
          </li>
          <li v-if="devValue.type == 'number'">
            {{ index }}: {{ devValue.val }}
          </li>
          <li v-if="devValue.type == 'bool'">
            {{ index }}: {{ devValue.val }}
          </li>
        </template>

      </ul>
    </td>
    <td>
      <button v-for="actionstring in dev.available_actions" @click="action(actionstring)">{{ actionstring }}</button>
    </td>
  </tr>
</template>