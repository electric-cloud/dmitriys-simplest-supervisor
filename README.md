# Dmitriy's Simplest Supervisor

## Description

Since perl supervisors are not usable for windows containers due to missing fork syscall, we have to use something native that can do good multithreading.
Here is Dmitriy's simplest supervisor, written in Rust.

## Build

Install Rust 1.23+

And: 

```
cargo build --release
```

It will generate dss.exe

## Usage

```
CMD "dss.exe C:\path\to\services"
```

Services is folder with one folder per service.
For example, if one need to run efagent and websphere, folders structure should be something like:

```
├── efagent
│   └── run.ps1
└── websphere
    └── run

```

If you need to call it using ec-perl, or ec-groovy, just use shebang:

```
#!ec-perl
```
