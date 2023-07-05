# hick

2023-07-05 重新利用 github actions 能编译出 windows 版本来了.  这里也简单记录下, 目前比较弯绕, 待改进的方式如下:

* 在 mac 上的 hick/rust/hir 下可以编辑调试
* 有修改 rsync 同步到 mac ~/data/hir 下提交到 github
* 然后 git tag v0.0.5 这样的约定格式
* git push --tag 就可以出发 github 的 actions 自动编译了
* 虽然在 hir@github web 界面的 release/draft a new release 也可以创建 tag 那个好像作为草稿跟其他地方容易冲突

实际现在的做法跟官方的不大一样, 不清楚为什么, 更多详情参考 apps.mm#!github/github actions

230428-092326 前几天搭建好的, mac 环境比较顺畅, 直接 cargo tauri dev 可以运行

# Tauri + Vue 3 + TypeScript

This template should help get you started developing with Vue 3 and TypeScript in Vite. The template uses Vue 3 `<script setup>` SFCs, check out the [script setup docs](https://v3.vuejs.org/api/sfc-script-setup.html#sfc-script-setup) to learn more.

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Volar](https://marketplace.visualstudio.com/items?itemName=Vue.volar) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## Type Support For `.vue` Imports in TS

Since TypeScript cannot handle type information for `.vue` imports, they are shimmed to be a generic Vue component type by default. In most cases this is fine if you don't really care about component prop types outside of templates. However, if you wish to get actual prop types in `.vue` imports (for example to get props validation when using manual `h(...)` calls), you can enable Volar's Take Over mode by following these steps:

1. Run `Extensions: Show Built-in Extensions` from VS Code's command palette, look for `TypeScript and JavaScript Language Features`, then right click and select `Disable (Workspace)`. By default, Take Over mode will enable itself if the default TypeScript extension is disabled.
2. Reload the VS Code window by running `Developer: Reload Window` from the command palette.

You can learn more about Take Over mode [here](https://github.com/johnsoncodehk/volar/discussions/471).
