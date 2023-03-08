# QPSK_modulator

QPSK modulator built in Rust for an assignment for Zirkuitu Digitalen Laborategia

## Run Program - Ubuntu

To start running the program, first you'll need to install Rust and Cargo in your local machine

### Rust Installation

-   Via API
    It doesn't install rustup so it's recommended if the only reason of cloning the repo is to execute the program the program is to execute it

```bash
sudo apt update
sudo apt install rustc
```

-   Via rustup

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Check if rust is install correctly checking the version once rust is installed

```bash
rustc --version
```

### Cargo

```bash
sudo apt update
sudo apt install rustc
```

## Run program

```bash
cargo run
```

## Input values

-   Laginketa maiztasuna (48000 Hz)
-   Modulazioaren maiztasuna (3000 Hz)
-   Sinboloaren maiztasuna (750 sinbol/s)
-   Irteerako bit kopurua (16 bit)
-   Modulatzeko datuak (00011011)

## Debugging

Install rust-analyzer extension in VSCode and you'll be able to debug the program with any more complication

---

© 2023 by Iker Hernandez. This project is licensed under the MIT License.

This project has been written in English, because I have few extensions installed to check spelling in English and a one for Basque doesn't exist
