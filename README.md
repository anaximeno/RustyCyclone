# Rusty Cyclone

This is an implementation of a basic game physics engine on Rust.

The gif below is an example of a physic simulation made with this engine of a ball being thrown in the air, which falls by the gravitational force until it reaches the ground.

<img src="./images/fallball-example01.gif" alt="Falling ball gif">

### Dependencies

Before testing it, some dependencies must be installed on your computer, if you are on a GNU/Linux-based operating system you can install them using the following commands depending on your package manager:

**dnf (or yum) package manager**
```bash
sudo dnf install alsa-lib-devel mesa-libGL-devel libX11-devel libXrandr-devel libXi-devel libXcursor-devel libXinerama-devel cmake
```

**apt package manager**
```bash
sudo apt install libasound2-dev mesa-common-dev libx11-dev libxrandr-dev libxi-dev xorg-dev libgl1-mesa-dev libglu1-mesa-dev cmake
```

<!--
## Main references:
TODO: finish this reference section!
-->
