sam3x8e
===
Rust crate to support Atmel's SAM3X8E microcontroller. The SAM3X8E is used on the Arduino Due board.
This source was created with the [svd2rust](https://docs.rs/svd2rust/) tool.

When this library was first created, the svd2rust-tool did not support the Atmel-SVD files, since they sometimes miss ResetValues. I forked the tool and fixed it. A PR was created.
My version of the svd2rust tool: https://github.com/ammoniak/svd2rust