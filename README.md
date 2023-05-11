# east

高性能、低占用的一款TCP端口转发工具。可以方便地将本地端口转发到远程服务器上。

![](https://img.shields.io/github/stars/cedar12/east)
![](https://img.shields.io/github/forks/cedar12/east)
![](https://img.shields.io/github/watchers/cedar12/east)
![](https://img.shields.io/github/languages/code-size/cedar12/east)
![](https://img.shields.io/badge/license-Apache%202-blue)
![](https://img.shields.io/github/downloads/cedar12/east/total)

## 使用技术

*   开发语言：rust
*   异步框架：tokio

## 特性

*   支持tcp、http、rdp、ssh。
*   支持ip规则过滤阻止连接代理端口。
*   插件化设计，支持加载第三方插件（请谨慎使用第三方插件）。
*   灵活的配置方式，可自定义端口转发规则、监听地址和转发目标地址等信息。
*   支持在Web控制台管理转发，可停止启动转发。需使用插件[east\_actix](https://github.com/cedar12/east/tree/main/plugin_actix)
*   支持转发限速
*   ~~支持在Web控制台显示每一个代理转发的速率、流量统计（待实现）~~

默认只支持配置文件配置端口转发规则，如需使用数据库配置请使用[数据库插件](#服务端插件)

## 安装说明

### 编译环境要求

Rust 版本：1.67.1 或更高版本

### 下载和编译

你可以从 Github 上下载最新版的 east 源代码：

```sh
git clone https://github.com/cedar12/east.git
# 编译服务端
cargo build --release --package east_server
# 编译代理端
cargo build --release --package east_agent
# 编译数据库插件
cargo build --release --package east_sqlite
# 编译Web控制台插件
cargo build --release --package east_actix
```

编译完成后，可执行文件位于 target/release 目录中。你可以将该文件拷贝到你的项目目录中。

> 服务端和代理端还需根据需求修改配置文件 conf.yml [参考](#配置文件)

> 数据库插件和Web控制台插件需拷贝到服务端根据配置文件所配置的插件目录

### 运行

要运行east\_server，只需在终端中执行以下命令：

```sh
# 运行服务端
./east_server
# 运行代理端
./east_agent
```

## 配置文件

> 配置文件只支持yml格式，服务端、代理端默认均为conf.yml。可通过以下方式运行指定的配置文件

```sh
# 运行服务端
./east_server server_conf.yml
# 运行代理端
./east_agent agent_conf.yml
```

### 服务端配置

```yml
server:
  # 服务器绑定端口
  bind: 0.0.0.0:3555
  # 插件配置
  plugin: 
    # 插件目录 默认plugin
    dir: plugin
    # 数据库插件配置
    database:
      url: ../plugin_sqlite/data.db
      username: root
      password: root
    web:
      # web插件绑定端口 默认 127.0.0.1:8088
      bind: 127.0.0.1:8088
      # web插件控制台用户名 默认east
      username: east
      # web插件控制台密码 默认East&*!2023
      password: East&*!2023
# 如使用了数据库插件以下配置无效，需在数据库中配置
agent:
  # agent连接的id
  test:
      # 在服务器上绑定的端口号
    - bind_port: 8089
      # agent上转发的目标ip或域名
      target_host: 127.0.0.1
      # agent上转发的目标端口号
      target_port: 42880
      # 白名单 ip规则
      whitelist: 
        - 192.168.0.0/16
        - 127.0.0.1
    - bind_port: 8989
      target_host: 127.0.0.1
      target_port: 5880
      # 限制最大速率64kb/s 默认不限速
      max_rate: 64
```

### 代理端配置

```yml
# 服务端ip端口
server: 127.0.0.1:3555
# 对应服务端配置的agent.test。如服务端未配置的id，服务端会拒绝连接
id: test
```

## 服务端插件

服务端插件的作用是在不侵入自身代码的前提下，扩展服务端的能力
可以自行实现服务端插件，并编译成.so、.dll、.dylib等动态链接库。

插件需要实现Plugin trait。

详细的插件开发指南请参考[east\_plugin](https://github.com/cedar12/east/tree/main/east_plugin)

*   windows 插件使用dll
*   macos 插件使用dylib
*   linux 插件使用so

> 插件定义

*   [east\_plugin](https://github.com/cedar12/east/tree/main/east_plugin)

### 插件安装

将插件文件放入服务端所配置的插件目录，重启服务端即可

### 插件卸载

将插件文件移除服务端所配置的插件目录，重启服务端即可

### 插件分类

> 同类型插件不支持多个，暂定以下类型

*   database 数据库插件类型

    可通过更换插件来实现数据从不同数据库中读写
    sqlite数据库插件

    *   [east\_sqlite](https://github.com/cedar12/east/tree/main/plugin_sqlite)
*   web Web插件类型（依赖数据库插件）

    可实现不同的web界面

    Web控制台插件

    *   [east\_actix](https://github.com/cedar12/east/tree/main/plugin_sqlite)

### 下载使用

*   [下载地址](https://github.com/cedar12/east/releases/latest)

下载分为server（服务端）、agent（代理端），进入服务端目录或代理端目录配置conf.yml文件运行即可

### 帮助
#### windows系统运行时提示丢失``VCRUNTIME140.dll``
> 下载[Microsoft Visual C++ 2015 Redistributable](https://www.microsoft.com/en-us/download/details.aspx?id=53840)安装即可
#### windwos系统注册成系统服务
> 下载[nssm](http://www.nssm.cc/download)注册成系统服务

### 许可证

本项目基于Apache2.0许可证发布。

### 注意

1.  该开源项目仅供学习，使用者应自行承担责任。
2.  该开源项目不保证无错误或无缺陷。使用者应在自己的风险下使用该项目。
3.  该开源项目作者不承担任何由于使用该项目而引起的直接或间接损失，包括但不限于利润损失、数据丢失等。

