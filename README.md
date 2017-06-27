# openby

# What is this?
`openby` is generic open applicaiton command. (like `xdg-open`)  
`openby` has default application each file extensions.


# Sample
```
$ openby rust.pdf
-> open rust.pdf by evince.

$ openby movie.mp4
-> open movie.mp4 by vlc.
```


# TOML Configuration
```toml
version = 0.0

[[tools]]
command = "apvlv"
extentions = [ "pdf" ]

[[tools]]
command = "mirage"
extentions = [ "jpg", "png", "gif" ]

[[tools]]
command = "vlc"
extentions = [ "mp4", "mov", "avi" ]

[[tools]]
command = "vim -R"
extentions = [ "conf" ]
```


# Implementation Process
1. import toml crate
2. design toml format
3. read toml file
4. execute command
5. write toml config file
