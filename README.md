# egui-rover-gui

A reimplementation of [RoverGUI](https://github.com/ROMANT21/RoverGUI) in Rust's eframe/egui toolkit.

![The perfect image](https://github.com/ROMANT21/RoverGUI/raw/master/coolguy.png)

## The Plan

Using Rust for a front-end sounds horrifying, but it's actually pretty nice. In general, there are a few considerations to take when creating a GUI in Rust:

1. **Maintainablility**: Use libraries, APIs, and methodologies that will be easy to comprehend and utilize for a long time!
2. **Usability**: Anyone, anywhere, should be able to pick up and use the GUI without prior knowledge.
    - Implement help documentation and give more information in settings
    - Allow users to adjust settings as necessary, on the fly
    - Should work on all platforms (without being a pain)
3. **Documentation**: For future developers, the GUI and its components should be documented.
    - Rust does this automatically through [rustdoc](https://doc.rust-lang.org/rustdoc/what-is-rustdoc.html).
    - It could eventually be a good idea to implement tutorial documentation through an [mdBook](https://rust-lang.github.io/mdBook/). One of these would help future users (long after we're gone) to comprehend the software and its uses.

### Resources

1. [egui](https://github.com/emilk/egui): a GUI library which works on most platforms, including internet browsers through WebAssembly.
2. [eframe](https://github.com/emilk/egui/tree/master/crates/eframe): the egui framework that works to turn the basics of egui into a full ui ecosystem. 
3. [eframe YouTube Tutorial](https://www.youtube.com/watch?v=NtUkr_z7l84): it's YouTube so yeah
4. [Rust WebAssembly Book](https://rustwasm.github.io/docs/book/): the documentation for WebAssembly, which converts Rust code to run in the browser sandbox. It also enables use of egui in the browser!
5. [libzmq-rs Guidebook](https://jean-airoldie.github.io/libzmq-rs/basics/patterns.html)
6. [libzmq-rs docs.rs site](https://docs.rs/libzmq/0.2.5/libzmq/index.html)
7. [image (rust) crates.io page](https://crates.io/crates/image)
8. [libzmq-rs-Based Projects](https://github.com/jean-airoldie/libzmq-rs/network/dependents)
9. [RustWASM Gamepad Docs](https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Gamepad.html)
10. [eframe-template](https://github.com/emilk/eframe_template)

### To Do

- [ ] Finish basic UI
  - [ ] SoRo Logo
  - [ ] Pink background
- [ ] "Video" stream receiver
- [ ] Buttons
  - [ ] Start stream
  - [ ] Stop stream
  - [ ] Change Port
  - [ ] Change IP
  - [ ] Scan for devices (return list)

### Development

This project uses `trunk` to compile to WebAssembly. To run it on your machine's browser, just run `trunk serve --release`, then navigate to `http://127.0.0.1:8080/index.html#dev` or wherever trunk says `📡 server listening at http://place:port/index.html#dev` :)

***
`#dev` is important here, as it allows you to **skip sw.js caching on dev builds**.
***

If you want to run it on your own machine, you can compile it like normal. Just run `cargo build` and you'll be able to to run it like a native app (bc it is...)

#### Problems

You may have some minor problems if you don't have all the requirements.

If you get errors related to your Rust version, just run `rustup update`. That fixes most problems...

Compiling on Linux sometimes requires these packages. You'll probably want them anyways, so give these commands a try for your distro:

Ubuntu/Debian: `sudo apt install libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev libssl-dev` (not all-inclusive, but that's probably most of it)

Fedora/CentOS/RHEL: `sudo dnf install clang clang-devel clang-tools-extra libxkbcommon-devel pkg-config openssl-devel libxcb-devel fontconfig fontconfig-devel binaryen gcc alsa-lib-devel cmake make gcc-c++ freetype-devel expat-devel libX11-devel` (all inclusive if you're on GNOME/Xfce)

Windows users should probably grab Visual Studio, as it typically provides development things. Or just use WSL :)

### Launch on Horrifying Setups

If you're weird like me, you may be using a MacBook running Asahi Linux. Since Asahi doesn't have complete WebGL, browsers don't use it by default. So... the screen will be blank when you launch an eframe application. This can also happen on MUCH older machines.

Run this command to launch Firefox with WebGL, no matter how apalling your setup is:
`MESA_GL_VERSION_OVERRIDE=3.3 MESA_GLSL_VERSION_OVERRIDE=330 MESA_GLES_VERSION_OVERRIDE=3.1 MOZ_ENABLE_WAYLAND=1 firefox`

You can then develop like normal using `trunk serve --release`. You'll actually see something, and it'll look pretty good, too :)
