## Setup

1. Set _project_name_ in Cargo.toml
2. Set mcu_support crate in Cargo.toml
3. Set a default compilation target. There are four options as mentioned at the bottom of .cargo/config. For the STM32F303VCT6, which has a Cortex-M4F core, we'll pick the thumbv7em-none-eabihf target.
4. Setup .vscode/settings

```m
    "mcu" - target mcu,
    "svd_file" - path to svd file,
    "openocd_config" - path to openocd config,
    "flash_size" - flash size,
    "ram_size" - ram size,
    "exec_name" - name of build file (same as name of folder by default),
    "target" - compiler target (same as .cargo/config)
    "exec_basepath" - path to output directory (dont change),
```

4. Without mcu support crate you must to setup memory.x file
   1. Copy it from selected mcu/<_mcu_> folder to root

## Add mcu

1. Add **.svd** and **memory.x** file in mcu/type folder
2. Add **openocd.cfg** in mcu/type folder
3. Add section to .vscode/settings.json

## Change gdb server

Select jlink or stlink in openocd.cfg file
