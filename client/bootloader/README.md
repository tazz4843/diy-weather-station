# bootloader

Flash the bootloader by running this command in the same directory as this README:

```
cargo flash --release --chip RP2040 --all-features
```

After this is run, the chip will automatically reset every 10 seconds thanks to the watchdog timeout,
until actual firmware is flashed.
Ten seconds is roughly enough time to flash 268KiB of firmware, so it should be enough to flash the
complementary firmware in `../app`.
