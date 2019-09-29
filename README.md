# DBCli

`dbcli` 让你能够在本地直接连接服务端数据库. 

`dbcli` 首先利用 `ssh` 和跳板机建立隧道， 然后调用相关的数据库客户端连接本地的端口。
目前支持 `mysql`, `mongodb`。


## Install
todo

## Config

在 `$HOME` 目录下新建 `.dbcli`, 配置文件采用 [`toml`](https://github.com/toml-lang/toml) 格式.
`.dbcli` 配置例子：
```toml
[jump_server]
username = "username"
host = "host"
port = 0 

# mysql server 1
[[mysql]]
db = "name1"
host = "host2"
port = 0
username = ""
password = ""

# mysql server 2
[[mysql]]
db = "name2"
host = "host2"
port = 0
username = ""
password = ""

# mongodb server 1
[[mongo]]
db = "name3"
host = "host3"
port = 0
username = ""
password = ""
```

### Custom Cli
添加下面的配置到 `.dbcli` 里面
```toml
[client]
mysql = "custom mysql cli" # e.g. mycli
mongo = "custom mongo cli"
```

## Usage
```
dbcli db_name
```