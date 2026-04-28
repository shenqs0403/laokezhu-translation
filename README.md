## 翻译

当前软件版本： 1.0.0

目前仅支持有道和百度两个，之后每个小版本的升级都会增加至少一个翻译引擎，例如升级到1.0.1

## Linux

快捷键设置需要在系统设置 -> 键盘 -> 查看及自定义快捷键 -> 自定义快捷键    界面设置（这里是Debian13 Gnome桌面的）

名称：自定义，随便写

命令：

~~~ shell
laokezhu-translation -- translate
~~~

快捷键： 自定义，设置成自己喜欢的

**提示**

如果直接设置命令，不能使用，将命令写出到文件，并赋予执行权限，在配置成这个可执行文件即可。
例如：将命令写出到 ~/laokezhu_translation_translate 文件，可以进行下边的操作：

~~~ shell
echo "laokezhu-translation -- translate" > ~/laokezhu_translation_translate
~~~

~~~shell
chmod +x ~/laokezhu_translation_translate
~~~

在把 ~/laokezhu_translation_translate 替换“命令”里的内容

### 配置界面

Linux系统使用系统在带的快捷键管理工具设置快捷键，Win和Mac使用配置界面的快捷键设置

### 特别说明

本软件没有在MacOS上测试过，不能保证稳定使用，通常来说，Windows和MacOS系统上是使用其官方提供的软件

### 免费额度

| 引擎名称 | 免费额度                       |
| -------- | ------------------------------ |
| 百度     | 每月100万字符，需个人认证      |
| 有道     | 无，只有新用户赠送的免费体验金 |
| 腾讯     | 每月500万字符                  |
| 微软     | 每月200万字符                  |
| 阿里     | 每月100万字符                  |
| 华为     | 每月100万字符                  |
| 火山     | 每月200万字符                  |
| 谷歌     | 每月50万字符                   |
| DeepL    | 每月50万字符                   |

这里仅供参考，具体在官网查询。

### 捐赠
<img src="public/wx.jpg" alt="wx.jpg" style="zoom:25%;" /><img src="public/zfb.jpg" alt="zfb.jpg" style="zoom:25%;" />