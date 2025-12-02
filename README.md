# Keyboard Recorder 键盘记录器
自动监测用户键盘输入行为并记录，用于查看和复盘输入习惯。可区分单键与组合键。

## 使用方法
目前只支持 Windows (11) 系统。Windows 其他版本未测试。

### 下载
在 [Releases](https://github.com/RRRexyz/keyboard-recorder/releases) 页面下载最新版压缩包并解压。

### 启动
在项目根目录下打开终端，运行
```powershell
./kero.exe start
```
即可启动软件，使其在后台运行。

### 终止
```powershell
./kero.exe stop
```

### 查询记录
```powershell
./kero.exe query -s 查询所有单键记录
./kero.exe query -c 查询所有组合键记录
./kero.exe query 查询所有记录
```

### 清空数据库
```powershell
./kero.exe clear 直接清空数据库
./kero.exe clear -b 先备份一份backup,再清空数据库
```
