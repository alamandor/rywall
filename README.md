# Rywall
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

### Run on existing colorscheme file
- Run ``` rusty-theme -c [colorscheme file]```
### Reload .Xresources on-demand, through ```xrdb```
- Add ```#include "/path/to/colorscheme_file"``` to your Xresources file and comment out the existing colors if needed.
	- OR
- Use ```-r``` To use the default .Xresources from $HOME.
	- ```-n``` command following generating a new colorscheme

### Shuffle Colorscheme
This shuffles the keys and color values to get a different possibly better pallette from an image. Run repeatly until desired results.
- ```rusty-theme -i image_file -s [desired name] --random ```
# Further Use
- Xresource colors can be defined in programs like i3 using the existing system colors. Colors that can be found with this tool.
	- In Useful Info section there is a link that outlines how to define Xresource variables in your i3 config.
- For my testing, I used st terminal which has the option to use the colors defined by Xresources, Xterm can also be used but this hasnt been thouroughly tested.
# Options
- ```-h, --help```           Display Help
- ```-i, --image <file> ```   Use supplied file for colorscheme
- ```-s  --save <name> ```    Use supplied name for colorscheme file generated
- ```-r ```                   Reload the default .Xresources file cannot use with -n
- ```-n --now ```            Reload Xresources with generated colorscheme
- ```-c --colorscheme ```     Load the provided colorscheme file made with the tool in xrdb
- ```--random```		Shuffle the new pallette to different keys to change how external programs use the new colors.

# How it Works
- When you run the app with the -i option followed by a jpeg image, the most common 16 colors are grabbed from the image. This color pallete is saved to a text file that follows the syntax for defining hexadecimal colors as outlined by the Xresource system. Mainly, it adds the \* wildcard identifier followed by a color[n] from n = (0-15).
	- The optional -s flag allows the user to enter a filename to save the colorscheme to.
	- After which, they can be sourced to the users .Xresource file by the user with one line: ``` #include "/path/to/colorscheme_file" ```
- To help make sure that the foreground and background colors are as reasonable as they can be, the color pallete has its luminance calculated and the darkest color is assigned to the background, and the brighest color to the foreground.
- The random flag (--random) splits up the keys and the hex colors and shuffles them to get a different result, this is to try and deal with the issue of some of the colors not being suitable as a particular color number. Programs use the number following the color to assign that color in predetermined slots, so sometimes moving them around can make an otherwise unsuitable image create a better pallette.
- The algorithm for the Median Cut is well-documented on the internet, the best documentee ones tended to be in Java.
- Median Cut works but repeately splitting boxes that contain the colors and the volume of the colors in the image provided. We split the boxes until we get 16, along the way sorting the colors in descending order so we can ensure the split happens at distinct values. The end result is averaged at the end and the pallette is returned in the form of seperate vector of colorChannels, the data strcuture to hold the "pixels".

# Issues
- Usefulness as a colorscheme for your terminal is varied based on the source image. An image with not many contrasting colors will generate a pallete where most of the colors are the same.
- A big issue was figuring ways to deal with converting the incoming vector of 8-bit integers representing the rgb values. To do the Median Mean Cut Quantization [Median Cut](https://en.wikipedia.org/wiki/Median_cut) I needed to used 32-bit values, so the conversion involved iterating through the 8-bit vector and building them as 32-bit integers, making sure to acknowledge that the resulting array is a quarter of the length.
satisfactory results.


- [Online color grabber 1](https://superdevresources.com/tools/color-extractor)
- [Online color grabber 2](https://labs.tineye.com/color/cfe365d6bf120f52b757156b1fea15b3b2299643?ignore_background=True&width=250&color_format=hex&ignore_interior_background=True&height=140)
- The app displays the generated colorscheme after it is done and has options to retreive the actual colors be used by the Xsystem. So this can be verified by using the tool or running xrdb yourself.
### Integrated Tests
- There are 3 test that one can run with the ``` cargo test -- --nocapture``` since verifying that colors were loaded is done with println! statements.
	- I reccomend choosing them by name to run them, 1 at a time, I had strange behavior runnning the function to retrive from the running Xdatabase becasue I believe the tests can run in parallel.
	- loaded_to_xystem verfies that rusty-theme correctly generates a colorscheme AND loads it into Xsystem by running the xrdb command.
	- random_to_xsystem tests the same as above but tests whether the hex xolors and the names associated with them are randomly shuffled properly
	- save_with_correct_name simple creates a colorscheme from an image and verifies it was correctly created with the correct name with an assert statement.
# Future Plans
- Use an external crate to display colors within the terminal output to speed up the process of comparing colorschemes.
- Add ability to create an organized colorcheme directory that we can search in subsequent runs of the program
- Better handling of errors in current code.
- Isolate functions in main and consolidate them in a library crate
