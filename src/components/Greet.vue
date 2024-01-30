<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const greetMsg = ref("");
const name = ref("");

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsg.value = await invoke("greet", { name: name.value });
  // 同时 alert 提示方便调试
  // 注意这里需要配置否则无法弹窗:  
  // Unhandled Promise Rejection: The `Dialog` module is not enabled. You must enable one of its APIs in the allowlist.
  alert(greetMsg.value);
}

// 2024-01-30 尝试调用 Rust 中定义的打开某个链接的窗口
// 注意这里的用法, rust 中定义的第一个参数是不用传递的
async function open_url_window(url: string)  {
  await invoke("open_external", { url: url });
}
</script>

<template>
  <div class="card">
    <button type="button" @click="open_url_window('http://slyt8.cn/actions')">打开 tauri 窗口</button>
    <button type="button" @click="greet()">Greet(输入文本后按这里为 JS 中调用 Rust 定义函数)</button>
    <br />
    <textarea id="greet-input"  style="width: 90%; height: 80px;" v-model="name" placeholder="Enter a name..." />
  </div>

  <p>{{ greetMsg }}</p>
</template>
