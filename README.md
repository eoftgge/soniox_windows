# Soniox Live: Subtitles

**The cross-platform advanced AI-based Soniox application that draws subtitles on the screen.**

#### Features
* Transparent overlay: Subtitles are displayed on top of all windows without interfering with the work.
* "Always on Top" mode: The window always remains visible.
* Click-through: In overlay mode, the program passes mouse clicks through itself (Mouse Passthrough).
* High performance: Written in Rust using RAII for resource management.
* Customization: Adjust the font size, text color to suit your needs.

### Launch

For build and start, you need [Rust Compiler](https://rust-lang.org/tools/install/)

```terminaloutput
>>> git clone https://github.com/eoftgge/soniox_live.git
>>> cd soniox_live
>>> cargo build --release
// in directory target/release appear file .exe, and move it file to the your directory
// and you should to create new configuration file (soniox.toml), example in repository.
// and run normally execution file. maybe, the antivirus will complain. it's normal
```

And you can also use binaries from [GitHub Releases](https://github.com/eoftgge/soniox_windows/releases) <br>
To run, you need a corresponding configuration file, an example is in the repository. Create it and fill in the config.
And run. Enjoy :)

If you have any problems, don't hesitate to ask in the Issues section
