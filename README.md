# Troubleshooting loading Egui's texture with Luminance

This repo renders Egui's texture the exact same way, same shaders, same rendering flow, on both the GL and WebGL2 backends of Luminance.

## Run app on web

```
$ yarn serve
```

http://localhost:8080

(Don't forget to `yarn`)

Execution begins at `start()` in `web.rs`.

## Run app on native

```
cargo run
```

# Screenshots

Here are screenshots so you don't have to run it.

(there's a clear color of `0.6, 0.6, 0.6, 1.` set on the pipeline so that we know any white is rendered from the shaders)

## Rendered on desktop
![image of texture on native](./native.png "Egui Texture on native")

This window has the dimensions of the texture, 2048x64. The clear color may be affecting it, but you can clearly see the characters Egui generates. The characters are upside-down and backwards, probably just due to a coordinate orientation difference between the window and Egui's texture.

![image of texture on web](./webgl.png "Egui Texture on web")

This is a 800x800 canvas against a dark blue background. You can see it is entirely white. The gray clear color set on the graphics pipeline is not displayed at all, so, this white color is coming from the sampler in the shader which should be returning pixels from the texture.




