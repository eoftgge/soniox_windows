# Soniox: Subtitles Live
**An advanced AI-based Soniox application that draws subtitles on the screen.
It supports only the Windows operating system, there can be no question of supporting other systems.**

**Advantages**
* It does not work as a window, but as a purely disguised and privileged window with text, and you can even click through it with the mouse. Quite convenient.
* Uninterrupted operation. If the connection is disconnected, it makes a repeat request to the server.
* Minimal memory consumption.
* And the most important thing is that it receives sound directly from Windows, without any third-party virtual channels. I'll say it's very convenient.

**Disadvantages**
* A quite inconvenient setup for inexperienced users. However, you can figure it out quickly, I think.

### TODO
1. [ ] Add translate as field to application
2. [x] Add binary-execution files
3. [ ] Add the separate window for settings application (In the distant future)
4. [ ] Add mark speaker ID for conversation
5. [ ] Fix exit application (in some instances)
6. [ ] Add display messages errors

### Launch
For build and start, you need [Rust Compiler](https://rust-lang.org/tools/install/)
```terminaloutput
>>> git clone https://github.com/eoftgge/soniox_windows.git
>>> cd soniox_windows
>>> cargo build --release
// in directory target/release appear file .exe, and move it file to the your directory
// and you should to create new configuration file (soniox.toml), example in repository.
// and run normally execution file. maybe, the antivirus will complain. it's normal
```

And you can also use binaries from [GitHub Releases](https://github.com/eoftgge/soniox_windows/releases) <br>
To run, you need a corresponding configuration file, an example is in the repository. Create it and fill in the config.
And run. Enjoy :)



If you have any problems, don't hesitate to ask in the Issues section
