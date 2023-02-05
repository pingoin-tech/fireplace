<script setup lang="ts">
import { Device } from "../plugins/serverTypes"

const props = defineProps<{
  dev: Device,
}>()
</script>
<template>
  <article>
    <pre>{{ JSON.stringify(dev,null,3) + "\n"}}</pre>
  </article>
</template>