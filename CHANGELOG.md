# Changelog

## 0.3.0 (Upcoming)

### Improvements

* **BREAKING:** Tetra now has its own `Key` and `MouseButton` types, instead of exporting the ones from the `sdl2` crate. Some key names have been changed to make them more obvious/consistent.
* **BREAKING:** The deprecated `build_matrix` method has been removed from `DrawParams`.

## 0.2.7 (January 23rd, 2019)

### Improvements

* We now use the [`hashbrown`](https://github.com/Amanieu/hashbrown) implementation of `HashMap`/`HashSet` instead of the `fnv` hasher. The hope was that this would give a performance boost, but it didn't seem to have any real observable impact :( That said, several of Tetra's dependencies use `hashbrown`, so in the interests of keeping the dependency tree light, we're switching anyway. ([@17cupsofcoffee](https://github.com/17cupsofcoffee), in [#91](https://github.com/17cupsofcoffee/tetra/pull/91))

### Bug Fixes

* A race condition between Rodio and SDL has been fixed. ([@17cupsofcoffee](https://github.com/17cupsofcoffee), in [#90](https://github.com/17cupsofcoffee/tetra/pull/90))
* While testing `hashbrown` integration, it was found that the benchmark numbers in the FAQ were slightly inaccurate - this has now been fixed. ([@17cupsofcoffee](https://github.com/17cupsofcoffee), in [#91](https://github.com/17cupsofcoffee/tetra/pull/91))

## 0.2.6 (January 20th, 2019)

### New Features

* Audio playback has been added, using [Rodio](https://github.com/tomaka/rodio) as the backend! ([@17cupsofcoffee](https://github.com/17cupsofcoffee), in [#84](https://github.com/17cupsofcoffee/tetra/pull/84))
* A port of the popular 'BunnyMark' benchmark has been added, which can be helpful for comparing relative performance of different versions/configurations of Tetra. ([@17cupsofcoffee](https://github.com/17cupsofcoffee), in [#77](https://github.com/17cupsofcoffee/tetra/pull/77))
* The documentation has been updated to detail the `sdl2_bundled` and `sdl2_static_link` features.

### Improvements

* The code that handles sprite transformations has been rewritten, and is now around 10 times faster than 0.2.5 in debug mode, and twice as fast as 0.2.5 in release mode. ([@17cupsofcoffee](https://github.com/17cupsofcoffee), in [#79](https://github.com/17cupsofcoffee/tetra/pull/79))

### Bug Fixes

* The `build_matrix` method on `DrawParams` was meant to be an internal utility, not a part of the public API. Tetra no longer uses it, so it has been deprecated, and will be removed in 0.3. ([@17cupsofcoffee](https://github.com/17cupsofcoffee), in [#79](https://github.com/17cupsofcoffee/tetra/pull/79))

## 0.2.5 (Janurary 6th, 2019)

### New Features

* Custom shaders can now be used for rendering! ([@17cupsofcoffee](https://github.com/17cupsofcoffee), in [#75](https://github.com/17cupsofcoffee/tetra/pull/75))

### Bug Fixes

* The parameters contained within an instance of `DrawParams` are now publicly accessible - without these, it wasn't possible to write a proper custom implementation of `Drawable`. ([@17cupsofcoffee](https://github.com/17cupsofcoffee), in [#73](https://github.com/17cupsofcoffee/tetra/pull/73))
* Shaders now bind their outputs explicitly - this should help with compatability. ([@17cupsofcoffee](https://github.com/17cupsofcoffee), in [#75](https://github.com/17cupsofcoffee/tetra/pull/75))

## 0.2.4 (Janurary 4th, 2019)

### Bug Fixes

* Fixed an issue where the OpenGL context would fail to initialize when using NVidia's proprietary Linux drivers. ([@17cupsofcoffee](https://github.com/17cupsofcoffee), in [#72](https://github.com/17cupsofcoffee/tetra/pull/72))

## 0.2.3 (January 3rd, 2019)

### New Features

* Tetra now has support for gamepads! The API is roughly the same as that of keyboard and mouse, aside from having to specify which gamepad's state you're querying. ([@17cupsofcoffee](https://github.com/17cupsofcoffee), in [#62](https://github.com/17cupsofcoffee/tetra/pull/62) and [#63](https://github.com/17cupsofcoffee/tetra/pull/63))

### Improvements

* Text is now drawn using the same shader as everything else - this likely won't be noticable now, but it will make things a lot easier once custom shaders get added! ([@17cupsofcoffee](https://github.com/17cupsofcoffee), in [#68](https://github.com/17cupsofcoffee/tetra/pull/68))

### Bug Fixes

* Some subtle issues around font cache invalidation have been fixed - in general we now let `glyph-brush` handle that side of things. ([@17cupsofcoffee](https://github.com/17cupsofcoffee), in [#61](https://github.com/17cupsofcoffee/tetra/pull/61) and [#68](https://github.com/17cupsofcoffee/tetra/pull/68))
* Texture flipping was broken in 2.0 - this has now been fixed. ([@17cupsofcoffee](https://github.com/17cupsofcoffee), in [#65](https://github.com/17cupsofcoffee/tetra/pull/65))
* The OpenGL context now explicitly requests a 32 bit color buffer and double buffering. ([@17cupsofcoffee](https://github.com/17cupsofcoffee), in [#66](https://github.com/17cupsofcoffee/tetra/pull/66))
* Shaders now bind their texture sampler explicitly, which should avoid black screens on some drivers. ([@17cupsofcoffee](https://github.com/17cupsofcoffee), in [#69](https://github.com/17cupsofcoffee/tetra/pull/69))

## 0.2.2 (December 24, 2018)

### New Features

* Tetra now has a [website](https://tetra.seventeencups.net), with a tutorial on how to get started using it. ([@17cupsofcoffee](https://github.com/17cupsofcoffee), in [#55](https://github.com/17cupsofcoffee/tetra/pull/55))
* `run_with` is now less restrictive about what kinds of closure it will accept. ([@17cupsofcoffee](https://github.com/17cupsofcoffee), in [#57](https://github.com/17cupsofcoffee/tetra/pull/57))

### Bug Fixes

* We now always request an OpenGL 3.2 core profile context - this is required for us to support MacOS. ([@17cupsofcoffee](https://github.com/17cupsofcoffee), in [#56](https://github.com/17cupsofcoffee/tetra/pull/56))

## 0.2.1 (December 22, 2018)

### New Features

* Shader errors are now properly reported via `TetraError::OpenGl`. ([@17cupsofcoffee](https://github.com/17cupsofcoffee), in [#50](https://github.com/17cupsofcoffee/tetra/pull/50))

### Bug Fixes

* The shader attribute order is now explicitly defined - this fixes an issue with black screens on some drivers. ([@17cupsofcoffee](https://github.com/17cupsofcoffee), in  [#52](https://github.com/17cupsofcoffee/tetra/pull/52))

## 0.2.0 (December 21, 2018)

### Breaking Changes

* The library has been upgraded to the 2018 edition of Rust.
* `ContextBuilder::new` now takes the title and size as parameters. The old behavior of the function can be replicated by using `ContextBuilder::default` instead.
* `run` is now a method on `Context`, instead of a free function. 
* The `update` and `draw` methods on `State` now return `tetra::Result`, allowing errors to be returned (or propagated via the `?` operator). Any errors returned from these methods will stop the game - your main method can then handle the error (e.g. log it out). ([#29](https://github.com/17cupsofcoffee/tetra/issues/29))
* The `scale` option on `ContextBuilder` has been renamed to `window_scale`, to better reflect its behavior.
* `Shader::from_file` is now called `Shader::new`, and `Shader::new` is now called `Shader::from_string`. This is more consistent with the other constructors.
* Tick rates are now specified in ticks per second ([#40](https://github.com/17cupsofcoffee/tetra/issues/40)).
* The `ContextBuilder` no longer consumes itself when called - this is more flexible for e.g. calling methods inside a conditional.
* `quit` has been moved to the `window` module.
* `set_tick_rate` has been moved to the `time` module.
* The functions for getting the game's internal width/height have been renamed to disambiguate them from the functions for getting the window width/height.
* Matching on `TetraError` will now force you to add a wildcard arm. This will prevent the addition of new error types from being a breaking change.
* `Shader::from_string` now returns `Result`, as proper error handling will be added to to it eventually ([#43](https://github.com/17cupsofcoffee/tetra/issues/43)).

### New Features

* `Texture` now has methods to get the width and height ([#31](https://github.com/17cupsofcoffee/tetra/issues/31)).
* The `bundled` and `static-link` features from the `sdl2` crate can now be used through Tetra by enabling the `sdl2_bundled` and `sdl2_static_link` features ([#33](https://github.com/17cupsofcoffee/tetra/pull/33), by [@VictorKoenders](https://github.com/VictorKoenders)).
* New methods have been added to allow iterating over down/pressed/released keys on the keyboard ([#35](https://github.com/17cupsofcoffee/tetra/pull/35), by [@VictorKoenders](https://github.com/VictorKoenders)):
    * `input::get_keys_down`
    * `input::get_keys_pressed`
    * `input::get_keys_released`
* Text input typed by the user can now be retrieved using the `input::get_text_input` function ([#36](https://github.com/17cupsofcoffee/tetra/pull/36), by [@VictorKoenders](https://github.com/VictorKoenders)).
* `Text` now has a method for efficiently calculating (and caching) the outer bounds of the text ([#41](https://github.com/17cupsofcoffee/tetra/pull/41), by [@VictorKoenders](https://github.com/VictorKoenders)).
* New methods have been added to `Animation`, allowing it to be modified after it is initially created ([#48](https://github.com/17cupsofcoffee/tetra/pull/48), by [@VictorKoenders](https://github.com/VictorKoenders))
* There are now numerous different `ScreenScaling` types that can be chosen from ([#21](https://github.com/17cupsofcoffee/tetra/issues/21)).
* Extra options have been added to the `ContextBuilder`, allowing you to start the window in various different states (e.g. fullscreen) ([#28](https://github.com/17cupsofcoffee/tetra/issues/28)).
* There are now many new methods for manipulating the window/game loop in the `window` module.
* The `update` and `draw` methods on `State` are now both optional.
* The `graphics` module now re-exports `Vec2`.
* In addition to the normal `run` method, there is now also a `run_with` method that uses a closure to construct the `State`. This is handy when method chaining - see the examples for how it can be used.
* Public types now implement `Debug` and `Clone` where appropriate.
* `TetraError` now implements the standard library `Error` trait ([#46](https://github.com/17cupsofcoffee/tetra/issues/46)).

### Bug Fixes

* The model matrix is now calculated once per `Drawable`, instead of once per vertex. This should speed up rendering ([#26](https://github.com/17cupsofcoffee/tetra/issues/26)).
* The top left corner of a `NineSlice` no longer gets distorted if the x and y of the `fill_rect` aren't equal.
* The renderer now automatically flushes instead of panicking if it hits capacity ([#30](https://github.com/17cupsofcoffee/tetra/issues/30)).
* The renderer will now batch up to 2048 sprites, instead of 1024.
* The default shaders have been rewritten in an older/more compatible syntax, in order to fix some issues with black screens on Mesa drivers ([#14](https://github.com/17cupsofcoffee/tetra/issues/14)).
* The `is_mouse_button_pressed` and `is_mouse_button_released` functions now work correctly. 

## 0.1.6 (December 9, 2018)

### New Features

* The `Font` and `Text` types have been added, allowing you to render out text using a TTF font ([#17](https://github.com/17cupsofcoffee/tetra/issues/17)).
* Inspired by FNA, the `TETRA_OPENGL_FORCE_CORE_PROFILE` environment variable can now be set to force the application to run using the 3.2 core profile. This might end up getting removed in favour of a more robust solution later on, but it's handy for testing (e.g. Renderdoc requires the core profile to be enabled).

### Bug Fixes

* The internal framebuffer is now an RGB texture instead of an RGBA texture - this was causing some strange issues with blending.

## 0.1.5 (December 8, 2018)

### Bug Fixes

* The batcher was performing a flush after texture switches occured, not before.

## 0.1.4 (December 8, 2018)

### New Features

* Graphics can now be rotated using the `rotation` method on `DrawParams` ([#24](https://github.com/17cupsofcoffee/tetra/issues/24)).

### Bug Fixes

* The calculation of how many elements to render when flushing was broken, which could lead to geometry persisting between frames even when the associated graphic was no longer active.

## 0.1.3 (December 7, 2018)

### New Features

* The `NineSlice` type has been added, allowing you to easily create dialog boxes from small textures ([#23](https://github.com/17cupsofcoffee/tetra/issues/23)).
* The window size can now be set explicitly. This will take precedence over the scale setting ([#19](https://github.com/17cupsofcoffee/tetra/issues/19)).
* `tetra::error::Result` and `tetra::error::TetraError` are now re-exported in the root of the crate. This allows you to write `tetra::Result` in your function signatures, which aligns a bit better with other custom `Result` types like `io::Result` ([#18](https://github.com/17cupsofcoffee/tetra/issues/18)).
* [An example of how to use the `Animation` type has been added](https://github.com/17cupsofcoffee/tetra/blob/master/examples/animation.rs)  ([#16](https://github.com/17cupsofcoffee/tetra/issues/16)).


## 0.1.2 (December 3, 2018)

### Bug Fixes

* Quick fix to the docs for the mouse button methods.

## 0.1.1 (December 3, 2018)

### New Features

* Functions for checking the state of the mouse buttons have been added.
    * `input::is_mouse_button_down`
    * `input::is_mouse_button_up`
    * `input::is_mouse_button_pressed`
    * `input::is_mouse_button_released`

### Bug Fixes

* Scaling is now applied relative to the origin ([#12](https://github.com/17cupsofcoffee/tetra/issues/12)).
* Mouse positions now take into account letterboxing ([#13](https://github.com/17cupsofcoffee/tetra/issues/13)).
* Various fixes to the documentation and crate metadata.

## 0.1.0 (December 2, 2018)

Initial release!
