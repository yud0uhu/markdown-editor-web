<template>
  <div
    class="w-screen h-full bg-gradient-to-r from-cyan-500 to-blue-500 hero py-10"
  >
    <div class="hero-content text-center">
      <h1 class="text-5xl font-bold pr-24">Markdown Editor Web</h1>
      <div class="max-w-xl">
        <div class="mockup-window border bg-base-300">
          <div class="flex justify-center px-4 py-16 bg-base-200">
            <div class="w-screen max-h-screen">
              <form>
                <div class="w-full border bg-gray-700 border-gray-200">
                  <div
                    class="flex items-center justify-between px-3 py-2 border-b"
                  >
                    <div
                      class="flex flex-wrap items-center divide-gray-200 sm:divide-x"
                    >
                      <div class="flex items-center space-x-1 sm:pr-4">
                        <button
                          type="button"
                          class="p-2 cursor-pointer text-gray-400 hover:text-white hover:bg-gray-600"
                        >
                          <svg
                            aria-hidden="true"
                            class="w-5 h-5"
                            fill="currentColor"
                            viewBox="0 0 20 20"
                            xmlns="http://www.w3.org/2000/svg"
                          >
                            <path
                              fill-rule="evenodd"
                              d="M12.316 3.051a1 1 0 01.633 1.265l-4 12a1 1 0 11-1.898-.632l4-12a1 1 0 011.265-.633zM5.707 6.293a1 1 0 010 1.414L3.414 10l2.293 2.293a1 1 0 11-1.414 1.414l-3-3a1 1 0 010-1.414l3-3a1 1 0 011.414 0zm8.586 0a1 1 0 011.414 0l3 3a1 1 0 010 1.414l-3 3a1 1 0 11-1.414-1.414L16.586 10l-2.293-2.293a1 1 0 010-1.414z"
                              clip-rule="evenodd"
                            ></path>
                          </svg>
                          <span class="sr-only">Format code</span>
                        </button>
                      </div>
                    </div>
                    <button
                      type="button"
                      data-tooltip-target="tooltip-fullscreen"
                      class="p-2 rounded cursor-pointer sm:ml-auto text-gray-400 hover:text-white hover:bg-gray-600"
                    >
                      <svg
                        aria-hidden="true"
                        class="w-5 h-5"
                        fill="currentColor"
                        viewBox="0 0 20 20"
                        xmlns="http://www.w3.org/2000/svg"
                      >
                        <path
                          fill-rule="evenodd"
                          d="M3 4a1 1 0 011-1h4a1 1 0 010 2H6.414l2.293 2.293a1 1 0 11-1.414 1.414L5 6.414V8a1 1 0 01-2 0V4zm9 1a1 1 0 010-2h4a1 1 0 011 1v4a1 1 0 01-2 0V6.414l-2.293 2.293a1 1 0 11-1.414-1.414L13.586 5H12zm-9 7a1 1 0 012 0v1.586l2.293-2.293a1 1 0 111.414 1.414L6.414 15H8a1 1 0 010 2H4a1 1 0 01-1-1v-4zm13-1a1 1 0 011 1v4a1 1 0 01-1 1h-4a1 1 0 010-2h1.586l-2.293-2.293a1 1 0 111.414-1.414L15 13.586V12a1 1 0 011-1z"
                          clip-rule="evenodd"
                        ></path>
                      </svg>
                      <span class="sr-only">Full screen</span>
                    </button>
                    <div
                      id="tooltip-fullscreen"
                      role="tooltip"
                      class="absolute z-10 invisible inline-block px-3 py-2 text-sm font-medium text-white transition-opacity duration-300 rounded-lg shadow-sm opacity-0 tooltip bg-gray-700"
                    >
                      Show full screen
                      <div class="tooltip-arrow" data-popper-arrow></div>
                    </div>
                  </div>
                  <div class="px-4 py-2 bg-gray-800">
                    <label for="editor" class="sr-only">Publish post</label>
                    <textarea
                      id="editor"
                      rows="20"
                      class="block w-full px-0 text-sm border-0 bg-gray-800 focus:ring-0 text-white placeholder-gray-400"
                      placeholder="Write an article..."
                      required
                      @change="convert"
                      v-model="inputText"
                    ></textarea>
                  </div>
                </div>
              </form>
            </div>

            <div
              class="w-screen max-h-screen py-6 bg-white border border-gray-200"
            >
              <article class="prose max-h-none" v-html="outputHtml"></article>
            </div>
          </div>
          <Footer />
        </div>
      </div>
    </div>
  </div>
</template>
<script lang="ts">
import { defineComponent, ref } from "vue";
import init, { text_to_token } from "../markdown-parser/pkg";

let wasmContainer: { text_to_token: typeof text_to_token };
import("../markdown-parser/pkg").then((wasm) => (wasmContainer = wasm));

export default defineComponent({
  name: "App",
  components: {},
  setup() {
    init().then(() => {
      // console.log("init wasm-pack");
      // greet("from vite!");
    });
    const inputText = ref("");
    const outputHtml = ref("");

    const convert = () => {
      console.log(inputText.value);
      console.log(wasmContainer?.text_to_token(inputText.value));
      outputHtml.value = wasmContainer?.text_to_token(inputText.value);
      // const parser = new DOMParser();
      // const doc = parser.parseFromString(outputHtml.value, "text/html");

      // console.log(wasmContainer?.pulldown_cmark(inputText.value));
    };
    return { convert, inputText, outputHtml };
  },
});
</script>
