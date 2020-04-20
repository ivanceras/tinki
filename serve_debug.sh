
#!/bin/bash

set -v

wasm-pack build --target no-modules --dev

basic-http-server ./ -a 0.0.0.0:4000
