<template>
  <div>
    <textarea @change="convert" v-model="inputText"></textarea>
    <p>{{ outputText }}</p>
    <!-- <NuxtWelcome /> -->
  </div>
</template>
<script lang="ts">
import { defineComponent, ref } from "vue";
import init, { greet, text_to_token } from "markdown-perser";

let wasmContainer: { text_to_token: typeof text_to_token };
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
    const outputText = ref("");
    const convert = () => {
      console.log(inputText.value);
      console.log(wasmContainer?.text_to_token(inputText.value));
      outputText.value = wasmContainer?.text_to_token(inputText.value);
      // console.log(wasmContainer?.pulldown_cmark(inputText.value));
    };
    return { convert, inputText, outputText };
  },
});
</script>
