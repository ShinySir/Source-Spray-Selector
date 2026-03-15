# Source Spray Selector <img width="48" height=auto alt="img1" src="https://raw.githubusercontent.com/ShinySir/Source-Spray-Selector/refs/heads/master/assets/icon.png" />
Spray Manager for Linux, Windows and MacOS

[![Rust](https://img.shields.io/badge/rust-1.94-orange?logo=rust&logoColor=white)](https://www.rust-lang.org/)  [![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0) [![GitHub release](https://img.shields.io/github/v/release/ShinySir/Source-Spray-Selector?logo=github)](https://github.com/ShinySir/Source-Spray-Selector/releases/latest)

Source Spray Selector is a simple spray selector desktop application GUI written in Rust using [`egui`](https://www.egui.rs/) for browsing, previewing, and managing `.vtf` spray files used in Source engine games like Team Fortress 2, Garry's Mod, Left 4 Dead 2 etc.

I have made this because I was tired on how clunky the in game UI for selecting of sprays, when you have alot of sprays it gets very tedious to choose, like the file names gets cut off if its long enough

---

## Features

- Previewing Sprays
- Favoriting via Right Click
- Applying Sprays for your Source Engine Games
- Saving and Loading of Game Profiles

---

## Screenshots

windows:

<div style="display:flex; gap:10px;">
  <img src="https://raw.githubusercontent.com/ShinySir/Source-Spray-Selector/refs/heads/master/assets/a1.png" style="width:400px; height:auto;">
  <img src="https://raw.githubusercontent.com/ShinySir/Source-Spray-Selector/refs/heads/master/assets/a2.png" style="width:400px; height:auto;">
  <img src="https://raw.githubusercontent.com/ShinySir/Source-Spray-Selector/refs/heads/master/assets/a3.png" style="width:400px; height:auto;">
</div>

linux wayland and x11 (old version):

<img width="400" height=auto alt="image_2026-03-12_17-06-57" src="https://github.com/user-attachments/assets/f49561ad-671d-4c1c-915b-81a52171d66c" />

<img width="400" height=auto alt="image_2026-03-13_01-41-27" src="https://github.com/user-attachments/assets/cdd3dc9f-dc18-4b5e-b5d9-6437e8120db2" />


---

## Usage

input a file path to the folder that will contain where the game stores sprays on ususally /materials/vgui/logos

example: C:\Program Files (x86)\Steam\steamapps\common\Team Fortress 2\tf\materials\vgui\logos

select a vtf file then select spray

you can add favorites via right clicking

---

## Installation

[Releases](https://github.com/ShinySir/Source-Spray-Selector/releases)

download either the linux binary or windows exe file depending which system you use

Mac is completly untested as I dont own any Mac devices

---

## Compiling

cargo build --release after downloading the source files
