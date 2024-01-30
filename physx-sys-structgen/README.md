
This is a minimal crate for the `structgen` tool that can be more-easily built
and bundled for iOS to run under a simulator.

To run:

```bash
git submodule update --init
cd physx-sys
export PHYSX_DIR=$PWD
cd ../physx-sys-structgen
cargo build --target aarch64-apple-ios-sim && \
cargo bundle --target aarch64-apple-ios-sim && \
xcrun simctl install A10C5F74-5969-4894-B99C-4E3F60BBBFA8 ../target/aarch64-apple-ios-sim/debug/bundle/ios/physx-sys-structgen.app/ && \
xcrun simctl launch --console A10C5F74-5969-4894-B99C-4E3F60BBBFA8 physx.rust.structgen
```

(swap the device ID for the ID of a simulator you've created)

