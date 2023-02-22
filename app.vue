<template>
  <div>
    <textarea @change="convert" v-model="inputText"></textarea>
    <!-- <NuxtWelcome /> -->
  </div>
</template>
<script lang="ts">
import { defineComponent, ref } from "vue";
import init, { greet, pulldown_cmark } from "markdown-perser";

let wasmContainer: { pulldown_cmark: typeof pulldown_cmark };
import("markdown-perser").then((wasm) => (wasmContainer = wasm));
export default defineComponent({
  name: "App",
  components: {},
  setup() {
    init().then(() => {
      // console.log("init wasm-pack");
      // greet("from vite!");
    });
    const inputText = ref("");
    const convert = () => {
      console.log(inputText.value);
      console.log(wasmContainer?.pulldown_cmark(inputText.value));
    };
    return { convert, inputText };
  },
});
</script>
