# egui_cable

[![GitHub](https://img.shields.io/badge/GitHub-ryo33/egui__cable-222222)](https://github.com/ryo33/mry)
![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)
[![Crates.io](https://img.shields.io/crates/v/egui_cable)](https://crates.io/crates/egui_cable)
[![docs.rs](https://img.shields.io/docsrs/egui_cable)](https://docs.rs/egui_cable)

A generic and extensible data-oriented widget for connecting ports by cables.

I create this for the visual programming editor of [Hihaheho/Desk](https://github.com/Hihaheho/Desk).

It's good for:

- analog synthesizer-like UI
- node-based UI
- anything you can imagine

The code is good for studying how to write egui widgets.

## Features

- [x] connect ports by data
- [x] dynamic connect and disconnect
- [x] lock connection
- [x] custom plug widget
- [x] custom port widget
- [x] custom cable widget
- [x] multiple connections on a single port.
- [x] on-connect event
- [x] on-disconnect event
- [x] on-hover event
- [x] garbage collection
- [ ] multi-touch support (help me)

## Examples

Click the images to see the source code.

[![Simple example](https://user-images.githubusercontent.com/8780513/170413124-5400d9b4-8f10-4607-bc37-23acc8c6b842.png)](https://github.com/ryo33/egui_cable/blob/main/examples/simple.rs)

[![Connect example](https://user-images.githubusercontent.com/8780513/170408794-ef879b04-9095-4c48-94d6-4773af9437e1.png)](https://github.com/ryo33/egui_cable/blob/main/examples/connect.rs)

