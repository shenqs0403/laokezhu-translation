## Translation 

**This document is translated by translation software**

Current software version: 1.0.0

Currently only Youdao and Baidu are supported. Each minor version upgrade will add at least one translation engine, e.g., upgrade to 1.0.1.

### Linux Wayland

Since the Linux Wayland environment does not support global mouse and keyboard events, text selection translation is not available in Wayland.Shortcut settings can be configured in: System Settings → Keyboard → View and Customize Shortcuts → Custom Shortcuts (This is for Debian 13 Gnome Desktop).
**Name**: Custom, you can set it freely.Command:

**Command:** 

~~~shell
laokezhu-translation --translate
~~~

**Shortcut:** Custom, set as you like.
**Tip** 

If the direct command does not work, write the command to a file, grant execution permission, then use this executable file as the command.For **example:** Write the command to ~/laokezhu_translation_translate using the following steps:

~~~shell
echo "laokezhu-translation --translate" > ~/laokezhu_translation_translate
~~~

~~~shell
chmod +x ~/laokezhu_translation_translate
~~~

Then replace the command field with ~/laokezhu_translation_translate.

### On Linux X11, Windows, MacOS

Shortcuts and text selection can be set directly in the settings interface.

### Special Note

This software has not been tested on MacOS, so stable operation cannot be guaranteed. Generally, official system-provided software is recommended for Windows and MacOS.

### Free Quota

| Engine Name | Free Quota                                                   |
| ----------- | ------------------------------------------------------------ |
| Baidu       | 1,000,000 characters per month, personal authentication required |
| Youdao      | None, only free trial credits for new users                  |
| Tencent     | 5,000,000 characters per month                               |
| Microsoft   | 2,000,000 characters per month                               |
| Alibaba     | 1,000,000 characters per month                               |
| Huawei      | 1,000,000 characters per month                               |
| Volcano     | 2,000,000 characters per month                               |
| Google      | 500,000 characters per month                                 |
| DeepL       | 500,000 characters per month                                 |

For reference only, please check the official website for details.

### Support and Donation
<img src="public/wx.jpg" alt="wx.jpg" style="zoom:25%;" /><img src="public/zfb.jpg" alt="zfb.jpg" style="zoom:25%;" />