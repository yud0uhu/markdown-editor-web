<template>
  <div>
    <NuxtWelcome />
  </div>
</template>
<script lang="ts">
import { defineComponent, ref } from "vue";
import init, { greet } from "markdown-perser";

export default defineComponent({
  name: "App",
  components: {},
  setup() {
    init().then(() => {
      console.log("init wasm-pack");
      greet("from vite!");
    });
  },
});
</script>
