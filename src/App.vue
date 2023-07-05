<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import Greet from "./components/Greet.vue";
import { reactive, nextTick, onMounted } from 'vue';
import { appWindow, getCurrent } from '@tauri-apps/api/window';

// hirn: 这个用法比较特色, 先声明类型
type Datas = {
  imgList: Record<string, any>[];  
  tip: string;
  winTop: string;
  quality: number;
};

const datas = reactive<Datas>({
  imgList: [],
  tip: '拖放图片文件到上方区域',
  winTop: '窗口置顶',
  quality: 80,
});

// 窗口置顶:
// 2023-06-10 已经在 Mac 核实, 这里有赖于在 rust/hir/src-tauri/tauri.conf.json 中设置 setAlwaysOnTop 才让程序有权限
function handleWindowTop() {
  console.log("==hiug old", datas.winTop);
  if (datas.winTop === '窗口置顶') {
    getCurrent().setAlwaysOnTop(true);
    datas.winTop = '取消置顶@230705-234954';
  } else {
    getCurrent().setAlwaysOnTop(false);
    datas.winTop = '窗口置顶';
  }
}

</script> 

<template>
  <div class="container">
    <h1>Welcome to Tauri!</h1>

    <div class="row">
      <a href="https://vitejs.dev" target="_blank">
        <img src="/vite.svg" class="logo vite" alt="Vite logo" />
      </a>
      <a href="https://tauri.app" target="_blank">
        <img src="/tauri.svg" class="logo tauri" alt="Tauri logo" />
      </a>
      <a href="https://vuejs.org/" target="_blank">
        <img src="./assets/vue.svg" class="logo vue" alt="Vue logo" />
      </a>
    </div>

    <p>
    
    Click on the Tauri, Vite, and Vue logos to learn more.
    <a href="https://slyt8.cn/">链接到我的 portal</a> | 
    <a href="https://slyt8.cn/admin">登录</a> | 
    <a href="https://slyt8.cn/actions" target="_blank">新窗口打开</a>
    
    </p>

    <p>
      Recommended IDE setup:
      <a href="https://code.visualstudio.com/" target="_blank">VS Code</a>
      +
      <a href="https://github.com/johnsoncodehk/volar" target="_blank">Volar</a>
      +
      <a href="https://github.com/tauri-apps/tauri-vscode" target="_blank"
        >Tauri</a
      >
      +
      <a href="https://github.com/rust-lang/rust-analyzer" target="_blank"
        >rust-analyzer</a
      >

    </p>

    <div class="action-btn" @click="handleWindowTop">{{ datas.winTop }}</div>

    <!-- hirn: 结合上面的 import Greet from "./components/Greet.vue"; 等同于导入/包含了一个外部模板 -->
    <Greet />
  </div>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}
</style>
