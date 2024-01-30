# hick

本项目说先考虑实用性, 兼练手 Rust 用途, 可能相关处理比较丑陋, 根据其他时间和兴趣等安排, 持续改进和打磨.

2023-07-05 重新利用 github actions 能编译出 windows 版本来了.  这里也简单记录下, 目前比较弯绕, 待改进的方式如下:

* 在 mac 上的 hick/rust/hir 下可以编辑调试
* 有修改 rsync 同步到 mac ~/data/hir 下提交到 github
* 从 mac 上面命令行触发:
  - 然后 git tag v0.0.5 这样的约定格式
  - git push --tag 就可以出发 github 的 actions 自动编译了
* web触发: hir@github web 界面的 release/draft a new release 
  - 碰到恰好 yaml 格式错了, with 前面少来一个空格, 导致这边也不行, 实际那边没问题这里是可以正常触发构建的

实际现在的做法跟官方的不大一样, 不清楚为什么, 更多详情参考 apps.mm#!github/github actions

230428-092326 前几天搭建好的, mac 环境比较顺畅, 直接 cargo tauri dev 可以运行

## TODO

  * 2024-01-28 菜单栏目前 mac/win 都有些问题
  * p0 2024-01-29 发现之前可以的 a 标签用 target="_blank" 打开新窗口的现在不行了, 不知道为啥, 问过下 AI 似乎还是可以结合 tauri 控制; 同时按住 shift 也可以
  * p1 2024-01-29 github actions 不知道怎么搞出来错误了, 但是实际都可以成功, 先忽略, 后续再探
  * p0 2024-01-29 系统托盘单机打开的 editor window 使用本地窗口, 临时内容尝试 JS 保存 local storage 提供下拉列表可以选择(根据窗口 label + 时间)
  * p1 2024-01-29 增加单元测试
  * 这个 github actions 是搜索来自别人的 github actions , 官方的还是值得理解以后再试试
  * 考虑参考其他项目比如官方的, 还是把版本号改成按照自动 tag , 记录从 package.json 的取法即可

## 平台问题

  * macmini 可以编辑和提交 github
  * deho 不配置 ssh key 失败无法提交 github
  * mac 可以提交 github 也适合调试, 不过调试目录在 hick/rust/hir 而 github 在 ~/data/hir


特别注意:

  * 目前取版本号的方式是走 package.json 中的 version , 对应代码:如下 =b01
  * 因为提交的时候还是需要 git tag 来触发, 考虑这边版本号用第二个, git 的用第三个
  * 出现过实际出现了 linux 版本 .tar.gz 但是结果提示失败了

调试经验

  * 点进某次 action 看详情, 可以大概看出问题在哪个步骤, 找对应关系跟官方版本的差别
  
```shell
# =b01 https://github.com/hick/hir/actions/runs/5465066384/workflow#L22 
echo "PACKAGE_VERSION=$(node -p "require('./package.json').version")" >> $GITHUB_ENV
```


## change log

  * 2024-01-28 点击系统图盘图标动态的创建窗口
  * 2024-01-27 支持菜单栏(Mac 上少俩个, win 前面俩个也不太正常)