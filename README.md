# dotfiles utils

一个简单的对dotfiles监控通知。当dotfiles中的文件改变后没有及时的同步在git中，将自动发送通知，如果在通知后没有及时的同步git，则自动同步git保存

## 监控文件

dotfiles主要是对linux的用户配置文件，也存在部分root文件如`/etc/docker/daemon.json`进行备份，在迁移时可一键完成配置。

要如何组织对应的配置文件呢

对于用户配置文件，通常在`~/.config`文件下，也有~/.zshrc等，那么xw

## 通知

### wsl2

参考：

* [Feature Request: Desktop Notifications #2466](https://github.com/microsoft/WSL/issues/2466#issuecomment-370316815)
* [Windos/BurntToast](https://github.com/Windos/BurntToast#burnttoast)

## 自动同步git
