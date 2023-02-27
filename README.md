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
