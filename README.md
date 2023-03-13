# GUI for [Psh](https://github.com/uvizhe/psh) Password Manager library

GUI targets Android (Cordova) and Desktop (Tauri).

*WARNING: Consider this a beta software.
The code compiles into Web Assembly and is very slow right now.
Some sensitive data is NOT wiped from memory on app exit.*

![](/psh-gui.png "")

## Prebuilt binary

Is available for Android only on Google Play Market: [Psh Password Manager](https://play.google.com/store/apps/details?id=im.uvizhe.psh)

## Building from source

### Android

Assuming all prerequisites for Android development have been installed

```sh
$ cd src-cordova
$ cordova platform add android
$ cordova run android  # This will build debug apk version and install on connected Android device or emulator
```

### Tauri

Assuming all Tauri prerequisites have been installed

```sh
$ cargo tauri build
```

## TODO

* [Hide](https://security.stackexchange.com/a/179346) sensitive data from browser and deal with it in Rust code exclusively (which allows zeroizing of memory)
* Improve Argon2 hashing performance (by waiting for further Wasm and Rust Argon2 optimizations :)) [Wasm in Chrome Android is very slow ATM :(]

## Thanks to

* 💛[@leann-fraoigh](https://github.com/leann-fraoigh) for helping me navigate the darkest places of Web Standards rabbit hole.
