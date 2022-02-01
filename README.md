d88dmp
=======
![](https://github.com/ORYZAPAO/d88dmp/workflows/Rust/badge.svg)
[![CircleCI](https://circleci.com/gh/ORYZAPAO/d88dmp/tree/main.svg?style=svg)](https://circleci.com/gh/ORYZAPAO/d88dmp/tree/main)

[日本語(Japanese)](/README_J.md)

Introduction
-----------
d88dmp is a report tool that disk image format file * .d88 for retro PC emulators.   
d88 is disk Image file format for Japanese 8bit Retro PC, [NEC PC-8801 Series](https://en.wikipedia.org/wiki/PC-8800_series), [Sharp X1 Series](https://en.wikipedia.org/wiki/Sharp_X1), [MSX](https://en.wikipedia.org/wiki/MSX) etc..

Output 
--------
The Output of d88dmp are shown below. 
### Floppy Disk Header
  + Disk Name
  + Write Protect
  + Floppy Disk Type(2D/2DD/2HD)
  + Floppy Disk Size(as byte)
### Track Table
  + Offset to any Track
### Sector Header
  + Cylinder Number(Zero Start...)
  + Side (0:surface、1:back) 
  + Sector Number(One Start...)
  + Sector Size(as byte)
  + Number Of Sector in Track
  + Density(D:Single, DD:Double-Density, HD:High-Density)
  + Delete Flag
  + Status
  + Data Size of Sector(as byte)
### Sector Data
  + Byte Dump Data


Install
---------------------
```
cargo install d88dmp
```

Source Code 
------------
--> ![Download](https://github.com/ORYZAPAO/d88dmp/releases)

I use ArchLinux, Intel Mac.

How to Use
------
```
 $ d88dmp <D88 Disk Image FIle>
```

## usage 
```
USAGE:
    d88dmp [OPTIONS] <*.D88>

ARGS:
    <*.D88>    D88 Disk Image

OPTIONS:
    -h, --help       Show help message 
    -n, --no-info    No detail Info.
        --no-color   Disable color
    -V, --version    Version info.

```

Example) CP/M formated Disk Image(Sharp X1turbo, turbo CP/M)

```
$ .\d88dmp testdata/turboCP_M_formated_disk.d88 
```
![example](https://github.com/ORYZAPAO/d88dmp/blob/main/image/d88info_img.png?raw=true)


Build
---------------------
```
$ git clone git@github.com:ORYZAPAO/d88dmp.git
$ cargo build --release
```

Licence
----------
 MIT Licens

Change Log
----------
+ [Change Log](https://github.com/ORYZAPAO/d88dmp/CHANGELOG.md)

See also
---------------------
+ [.D88形式のフロッピーディスクイメージフォーマット - （仮）](https://gra4.hatenadiary.jp/entry/20171108/1510096429)
+ [D88形式フォーマット | HuDisk](https://boukichi.github.io/HuDisk/DISK.html)
+ [wii88/FORMAT.TXT at master · jpzm/wii88](https://github.com/jpzm/wii88/blob/master/document/FORMAT.TXT)
