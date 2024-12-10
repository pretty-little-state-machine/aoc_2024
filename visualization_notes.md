# Example FFMPEG Usage for PNG to GIF

Build the Palette from the visualization images
```bash
ffmpeg -i %06d.png  -vf palettegen palette.png
```

Now create the GIF itself from the base files and the palette
```bash
ffmpeg -y -i %06d.png -i palette.png -filter_complex paletteuse -r 32 output.gif
```
