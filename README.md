# Soniox: Subtitle Live
**An advanced AI-based Soniox application that draws subtitles on the screen.
It supports only the Windows operating system, there can be no question of supporting other systems.**

**Advantages**
* It does not work as a window, but as a purely disguised and privileged window with text, and you can even click through it with the mouse. Very convenient.
* Uninterrupted operation. If the connection is disconnected, it makes a repeat request to the server.
* Minimal memory consumption.

**Disadvantages**
* A quite inconvenient setup for inexperienced users. However, you can figure it out quickly, I think.

### TODO
1. Add translate as field to application
2. Add binary-execution files
3. Add the separate window for settings application (In the distant future)
4. Add mark speaker ID for conversation
5. Fix exit application (in some instances)

### Launch
```terminaloutput
>>> git clone https://github.com/eoftgge/soniox_windows.git
>>> cd soniox_windows
>>> cargo build --release
// in directory target/release appear file .exe, and move it file to the your directory
// and you should to create new configuration file (soniox.toml), example in repository.
// and run normally execution file. maybe, the antivirus will complain. it's normal
```

If you have any problems, don't hesitate to ask in the Issues section
