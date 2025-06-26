## Compiling for raspberry pi

```bash
brew install llvm

export PATH="/opt/homebrew/opt/llvm/bin:$PATH"
export LDFLAGS="-L/opt/homebrew/opt/llvm/lib"
export CPPFLAGS="-I/opt/homebrew/opt/llvm/include"
export TARGET_CC=$(which clang)

brew install arm-linux-gnueabihf-binutils

cargo install cross@0.2.4
CROSS_CONTAINER_OPTS="--platform linux/amd64" cross build --target armv7-unknown-linux-musleabihf
```
