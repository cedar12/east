# east
高性能、低占用的一款TCP端口转发工具

## 使用技术
* 开发语言：rust
* 异步框架：tokio

## 端口转发
默认只支持配置文件配置端口转发，如需使用数据库配置文件请使用[数据库插件](#服务端插件)

1. 支持tcp、http、rdp、ssh
2. 支持ip规则过滤阻止连接代理端口

## 配置文件
> 配置文件只支持yml格式，服务端、代理端名称均固定为conf.yml

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
agent:
  # agent连接的id
  test:
      # 在服务器上绑定的端口号
    - bind_port: 8089
      # agent上转发的目标ip
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

* windows 插件使用dll
* macos 插件使用dylib
* linux 插件使用so

> 插件定义[east_plugin](https://gitee.com/cedar12/east/tree/main/east_plugin)
### 插件安装
将插件文件放入服务端所配置的插件目录，重启服务端即可
### 插件卸载
将插件文件移除服务端所配置的插件目录，重启服务端即可
### 插件分类
> 同类型插件不支持多个，暂定以下类型
* database 数据库插件类型

  可通过更换插件来实现数据从不同数据库中读写

  [east_sqlite](https://gitee.com/cedar12/east/tree/main/plugin_sqlite) sqlite数据库插件
* web Web插件类型（依赖数据库插件）

  可实现不同的web界面

### 下载
* [0.0.1](https://gitee.com/cedar12/east/releases/tag/0.0.1)