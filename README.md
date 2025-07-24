# DA-PAC

This repository contains the Peripheral Access Crates (PACs) for Dialog Semiconductor / Renesas Connectivity Devices.

The crate is generated from the SVD files in [packs](https://www.keil.arm.com/packs) using [svd2pac](https://github.com/Infineon/svd2pac).

It serves as the foundation for the Hardware Abstraction Layer (HAL) in Rust for Renesas Wireless Connectivity Devices.

## Supported Devices

- DA14531
- DA14580
- DA14581
- DA14582
- DA14583
- DA14585
- DA14586
- DA14680
- DA14681
- DA14682
- DA14683
- DA14691
- DA14695
- DA14697
- DA14699
- DA1469x

## Target Architectures

Each device corresponds to a specific architecture target:

- **thumbv6m-none-eabi** (Cortex-M0+/Cortex-M0):
  - DA14531
  - DA14580 / DA14581 / DA14583
  - DA14585 / DA14586
  - DA14680 / DA14681
  - DA14682 / DA14683

- **thumbv8m.main-none-eabi** (Cortex-M33):
  - DA14691
  - DA14695
  - DA14697
  - DA14699
  - DA1470x

## License

This crate is licensed under either the MIT License or the Apache License, Version 2.0.

The contents of this crate are auto-generated and licensed under the following proprietary terms of Renesas Electronics Corporation:
This software is supplied by Renesas Electronics Corporation and is only intended for use with Renesas products. No other uses are authorized. This software is owned by Renesas Electronics Corporation and is protected under all applicable laws, including copyright laws.

This software is provided "AS IS" without warranties of any kind, and Renesas makes no warranties regarding this software, including but not limited to
warranties of merchantability, fitness for a particular purpose, and non-infringement.

Renesas reserves the right, without notice, to make changes to this software and to discontinue the availability of this software. By using this software,
you agree to the additional terms and conditions found at:
[Renesas Disclaimer](http://www.renesas.com/disclaimer)
