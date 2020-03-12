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
# Project Name
- rusty-theme
# Author
- Alec Greenaway
- aag3@pdx.edu
# Useful Info
- [Method in which i3 reads colors (anything loaded into the X resource database)](https://i3wm.org/docs/userguide.html#xresources)
- [Xresources](https://wiki.archlinux.org/index.php/x_resources)
# Sources:
  - [Pywal (python)](https://github.com/dylanaraps/pywal)
  - [Java Example of Median Cut](http://jcs.mobile-utopia.com/jcs/16423_ColorCutQuantizer.html)
  - [Calculating Luminance from rgb values](https://stackoverflow.com/questions/596216/formula-to-determine-brightness-of-rgb-color)
  - [Median Cut -- Java](https://github.com/biometrics/imagingbook/blob/master/src/color/MedianCutQuantizer.java)
# Dependencies
- Need to have X installed on your system
- Have applications that use .Xresources to define colors (xterm, st terminal, i3)

# Quick Run
- Run ```rusty-theme -i [image_file] -s [desired colorscheme name] ```
	- OR
- Run ```cargo run -- -i image_file.jpg -s aagColorscheme``` Saves a colorscheme generated image_file to the file aagColorscheme.

### Reload .Xresources on-demand, through ```xrdb```
- Add ```#include "/path/to/colorscheme_file"``` to your Xresources file and comment out the existing colors if needed.
	- OR
- Use ```-r``` To use the default .Xresources from $HOME.
	- ```-n``` command following generating a new colorscheme

# Further Use
- Xresource colors can be defined in programs like i3 using the existing system colors. Colors that can be found with this tool.
- For my testing, I used st terminal which has the option to use the colors defined by Xresources, Xterm can also be used but this hasnt been thouroughly tested.
# Options
- ```-h, --help```           Display Help
- ```-i, --image <file> ```   Use supplied file for colorscheme
- ```-s  --save <name> ```    Use supplied name for colorscheme file generated
- ```-r ```                   Reload the default .Xresources file cannot use with -n
- ```-n --now ```            Reload Xresources with generated colorscheme
- ```-c --colorscheme ```     Load the provided colorscheme file made with the tool in xrdb

# How it Works
- When you run the app with the -i option followed by a jpeg image, the most common 16 colors are grabbed from the image. This color pallete is saved to a text file that follows the syntax for defining hexadecimal colors as outlined by the Xresource system. Mainly, it adds the \* wildcard identifier followed by a color[n] from n = (0-15).
	- The optional -s flag allows the user to enter a filename to save the colorscheme to.
	- After which, they can be sourced to the users .Xresource file by the user with one line: ``` #include "/path/to/colorscheme_file" ```
- To help make sure that the foreground and background colors are as reasonable as they can be, the color pallete has its luminance calculated and the darkest color is assigned to the background, and the brighest color to the foreground.
- The algorithm for the Median Cut is well-documented on the internet, the best documentee ones tended to be in Java.
- Median Cut works but repeately splitting boxes that contain the colors and the volume of the colors in the image provided. We split the boxes until we get 16, along the way sorting the colors in descending order so we can ensure the split happens at distinct values. The end result is averaged at the end and the pallette is returned in the form of seperate vector of colorChannels, the data strcuture to hold the "pixels".

# Issues
- Usefulness as a colorscheme for your terminal is varied based on the source image. An image with not many contrasting colors will generate a pallete where most of the colors are the same.
- A big issue was figuring ways to deal with converting the incoming vector of 8-bit integers representing the rgb values. To do the Median Mean Cut Quantization [Median Cut](https://en.wikipedia.org/wiki/Median_cut) I needed to used 32-bit values, so the conversion involved iterating through the 8-bit vector and building them as 32-bit integers, making sure to acknowledge that the resulting array is a quarter of the length.
- The other major obstactle was getting convertable with bit-wise operations, mainly the bit-wise AND (&). I had to do research on my own to figure out how to use them. However, I found useful ways to utilize them to break apart individual rgb values from a single rgb integer. Many of the resources I used to read up on the algorithm had implementations with bit-wise operations and I could not find a way to do it without using them. But after this project I definitely have a better understanding of them.
- Formatting strings into a form accepted by the Xresources file took some time as well, but I found that the format! macro was a life-saver for sure.
- Comparing float values for the luminance values of the generated colors required an outside library ```float-cmp``` since the compiler was giving me trouble for testing for equality among float values.
# Testing
- Finding ways to test this in the code was a big problem since the end result is based on a visual product, and the fact that the quantized colors are averages of orginal values passed inside, so I went to online pallete generators and compared by pallete with theres until I got consistent and satisfactory results.
- [Online color grabber 1](https://superdevresources.com/tools/color-extractor)
- [Online color grabber 2](https://labs.tineye.com/color/cfe365d6bf120f52b757156b1fea15b3b2299643?ignore_background=True&width=250&color_format=hex&ignore_interior_background=True&height=140)
- The app displays the generated colorscheme after it is done and has options to retreive the actual colors be used by the Xsystem. So this can be verified by using the tool or running xrdb yourself.
# Future Plans
- Add abilty to **Randomly** select color values for colors 0-15 for the colorcheme.
	- This is to address some images not being suitable for all around colorschemes
	- I would like to be able to cycle repeatley on input until a desired pallette is reached
- Add ability to create an organized colorcheme directory that we can search in subsequent runs of the program
