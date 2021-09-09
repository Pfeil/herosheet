# Herosheet

[![nightly build](https://github.com/Pfeil/herosheet/actions/workflows/build.yml/badge.svg)](https://github.com/Pfeil/herosheet/actions/workflows/build.yml)

Herosheet is a simple-to-use, unofficial server-side character sheet hoster for pen and paper games following the ["How to be a Hero" Pen and Paper ruleset](https://howtobeahero.de/).

## Current state

It is not yet tested in practice, but the basic functions are available (similar to the PDF features you get in the adobe reader but nowhere else, and without hosting, obviously!). It basically just offers a form, which is mostly a 1:1 copy of the PDF sheet. The forms update button will store the values on the server and refresh the skill calculations on the page. There is currently no client side javascript.

Obviously, this is actually a rather poor "put the paper thing into the computer" approach, but this way it was an easy weekend project for someone who is not a web developer. Ideas on how to evolve this system are welcome :)

Also note that this uses the master branch of the rocket dependency, which is obviously a ridiculous adventure itself.

## User Experience

The idea is currently very simple: You visit the main page, and will be immediately redirected to an empty sheet. The URL encodes the ID of this sheet, so make sure to make a bookmark of it. Use the button on the bottom of the sheet to store/update all information on the server. Doing so will also update the calculations of skill values on the page and give you some feedback.

To load an existing sheet, just click on the bookmark (or remember the url in another way).

## Deployment

Not done yet myself (only ran it on localhost so far). Currently, I think you will need to clone the repo and build the project with nightly rust. If you have rust installed (and also the nightly toolchain), it is a simple `cargo +nightly run` to run the server.

## TODO

- Some paths are relative to the crate manifest, but there should be a configurable basepath etc.
- Make it more interactive and less a paper-clone.
- There is no implicit functionality to note on how many eureka points are already spent. Currently, you will have to do it into the "notes" field.
- I want to host it on uberspace, and I think I can not directly compile it there. Therefore, I will need to make a binary. As they use an older version of libc or something, I will likely have to make a docker container to compile it on the same OS. I already did that for navilink, so this should not be hard (but boring).

## Credits

This project was derived from an example living in the rocket.rs master branch. It also contains the pretty [chota CSS](https://github.com/jenil/chota) ([MIT-License](https://github.com/jenil/chota/blob/master/LICENSE)), which makes life easier.
