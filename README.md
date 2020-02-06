# rusty-theme
A theme maker inspired by other tools to cutomize OS themes on the fly leveraging .Xdefaults system

## The plan:
Customizing my personal linux distro with different window managers and compnents of a desktop environment is what inspired me to get interested in system development. A lot of the time, getting the individual components, like the windows, status bar, and lock screen is simplified by having them retrieve a color pallet from a single source so they all sync up aesthetically. The Xdefaults system does this in the Xorg display system.
My goal would be to write a rust program that can manage and implement differet color palletes, to create the ability for the user to change color schemes on the fly easily.
## Features desired
- Have a single location where different color "config" files are stored where the app can read in the color values and update the appropriate config files depending on the OS tool being updated.
- Add the ability to generate color palletes randomly, but try and adjust it after so they don't "clash" *Perhaps normalize is a better term not sure* 
- Add the ability to genereate a color pallete from a supplied image file.
  - I'm thinking this will involved reading in the bytes of the image individually to construct a color pallete
  - I know there are algorithms used by tools like GIMP and photoshop that possibly could be useful that I could implement for my needs
  This might be a good oppurtunity to learn more about working with threads, and concurrent programming.
  
- Add feature that can connect and retrieve/update a git repo with your colorschemes. The idea being that the app and retrive your favorite schemes quickly on a new distro install or computer.
  - Maybe find a way to interact with github API
  - Else just evoke shell with git commands from within rust to pull remote repos
