# part_viewer
Tool for taking screenshots of 3D parts.

## Usage

Build the crate and move the `part_viewer` binary somewhere where it will be visible on your PATH. Then run the following:

```
part_viewer <INPUT_PATH> <OUTPUT_PATH> <OUTPUT_WIDTH> <OUTPUT_HEIGHT>
```

`<OUTPUT_PATH>` should be an STL file. A PNG will be created at the `<OUTPUT_PATH>`.

## Credits

I started this project by following the excellent Learn Wgpu tutorial, so some of the code in here is copy-pasted from that tutorial. The repo for the Learn Wgpu tutorial is [here](https://github.com/sotrh/learn-wgpu).
