# Mosaic

Stitches together images in the `tiles` directory (put your own images there) into a mosaic. The mosaic is built from an input image, where we apply the following process:

* downsize all tile images to 140x140 pixels
* reduce colors of the input image to 128 colors
* for each pixel of the input image, find the closest-looking tile image
* replace each pixel of the input image with the tile found

The result is a mosaic of tile images that resembles the input image.

## Usage

```sh
./mosaic ./input.png ./output.png
```

it's much faster when compiled in release mode :)

## Note

This is crappy night-time-coding code, so don't expect production-level genius. It's just a fun little project because I was curious on how to stream-write PNG files.
