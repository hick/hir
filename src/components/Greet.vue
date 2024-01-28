<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const greetMsg = ref("");
const name = ref("");

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsg.value = await invoke("greet", { name: name.value });
}
</script>

<template>
  <div class="card">
    <button type="button" @click="greet()">Greet(输入文本后按这里为 JS 中调用 Rust 定义函数)</button>
    <br />
    <textarea id="greet-input"  style="width: 90%; height: 80px;" v-model="name" placeholder="Enter a name..." />
  </div>

  <p>{{ greetMsg }}</p>
</template>
