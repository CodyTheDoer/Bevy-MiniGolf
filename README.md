![Screenshot of the first iteration of the rotating menu hub](images/Minigolf%20007.png)

# MiniGolf Simulator - The Daily Bonk

This project is my personal learning space for building a functional online enabled game in the bevy engine. It is a work-in-progress where I experiment and build knowledge.
![Screenshot of the first iteraction of the main menu](images/Minigolf%20001.png)

Deeply inspired by 3-D Ultra Minigolf from the late nineties.
    - Thanks Dad for letting me hog your PC endlessly to put balls in holes.

![Link to playthrough of 3-D Ultra-Minigolf](https://www.youtube.com/watch?v=8EPrQjw1210)

## Features

![Screenshot of State Focused development screen](images/Minigolf%20005.png)
The Daily Bonk is a modern ECS driven Golf Game
- A Modern Game Engine: Sound, UI, and Animations built with Bevy's ECS (Entity Component System).
- Multiple camera modes:
    - Live Ball Camera
    - Cup Orbit Camera
    - Free Pan Orbit Camera for those really tricky shots!
- Physics driven by Rapier
![Screenshot of Rapier Integration during development](images/Minigolf%20000.png)
- Online and local play with up to 6 Friends, or if you don't have any. We can also supply AI golf buddies.
- 18 holes of golf, 4 Modes of play...
    - Whole Corse
    - Front Nine
    - Back Nine
    - Select A Hole
- Custom Golf Balls
- Leaderboards...
- 3D Interactable Menus
![Screenshot of online menu scene development in blender](images/Minigolf%20002.png)

### Tech Stack

```
ECS: Bevy

Language:
    Rust

The Cargo Manifest:
    bevy = {version = "0.14.2", features = ["mp3"]}
    bevy_easy_vec_ui = { path = "D:/bevy/donut/bevy_easy_vec_ui" }
    bevy_editor_pls = "0.9"
    bevy_matchbox = "0.10"
    bevy_mod_raycast = "0.18.0"
    bevy-tokio-tasks = "0.14.0"
    bevy_rapier3d = "0.27.0"
    bevy_render = "0.14.2"
    dotenv = "0.15.0"
    time = { version = "0.3.36", features = [ "local-offset", "serde" ] }
    serde = { version = "1.0.215", features = ["derive"] }
    rmp-serde = "1.3.0"
    rusqlite = { version = "0.29.0", features = ["bundled"] }
    uuid = { version = "1.11.0",  features = [ "v4",  "v7", "fast-rng", "macro-diagnostics" ] }
    regex = "1.11.1"
    serde_json = "1.0.133"

CAD: 
    Blender
IDE:
    VSCode
Database Manager:
    DBeaver
Sound: 
    Audacity
```

![Screenshot of local menu scene and state monitoring during development](images/Minigolf%20004.png)

Also the main menu is absolutely on a floating golfball
![Screenshot of floating golfball menu from out of frame](images/Minigolf%20006.png)