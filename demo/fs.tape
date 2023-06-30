Output demo/fs.gif

Set FontSize 12
Set FontFamily "FiraCode Nerd Font"
Set Width 800
Set Height 500
Set TypingSpeed 75ms
Set Theme "Catppuccin Mocha"
Set Padding 0
Set Margin 0

Hide
Type@0ms "zellij --config .vhs/config.kdl"
Enter
Show
Sleep 500ms

Ctrl+P
Type "w"
Type "adb logcat -s RustApp"
Enter
Ctrl+P
Right
Ctrl+P
Type "adb shell touch /storage/emulated/0/Documents/test.txt"
Enter
Sleep 1
Type "adb shell 'echo Test >/storage/emulated/0/Documents/test.txt'"
Enter
Sleep 1
Type "adb shell rm /storage/emulated/0/Documents/test.txt"
Enter
Sleep 3
Ctrl+P
Type "w"
Sleep 1
Type "curl http://myphone:3000/fs"
Enter
Sleep 5
