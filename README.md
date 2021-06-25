# Loading Egui's texture with Luminance

This repo renders Egui's texture the exact same way, same shaders, same rendering flow, on both the GL and WebGL2 backends of Luminance.

The rendering for both occurs in `texture.rs`.

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


