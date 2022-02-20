F88FileIO
=======
![](https://github.com/ORYZAPAO/d88dmp/workflows/Rust/badge.svg)
[![CircleCI](https://circleci.com/gh/ORYZAPAO/d88dmp/tree/main.svg?style=svg)](https://circleci.com/gh/ORYZAPAO/d88dmp/tree/main)


Introduction
-----------
F88FileIO is a simple Library that D88 file access library.
D88 file is disk Image file for Japanese 8bit Retro PC, [NEC PC-8801 Series](https://en.wikipedia.org/wiki/PC-8800_series), [Sharp X1 Series](https://en.wikipedia.org/wiki/Sharp_X1), [MSX](https://en.wikipedia.org/wiki/MSX) etc..

Example
-----------
```
  use D88FileIO::fileio::D88FileIO;
  use D88FileIO::disk::{Sector, Track};
  use D88FileIO::format::{D88_SectorHdr};

  let d88fileio = D88FileIO::open("./ABC.d88");
  
  println!("{:?}", d88fileio.disk.header);

  for track in self.d88fileio.disk.track_tbl.iter() {
    for sector in track.sector_tbl.iter(){
      println!("{:?}", sector.header);
      println!("{:?}", sector.data);
    }
  }

```

See also
---------------------
+ [d88dmp(crates.io)](https://crates.io/crates/d88dmp)

+ [.D88形式のフロッピーディスクイメージフォーマット - （仮）](https://gra4.hatenadiary.jp/entry/20171108/1510096429)
+ [D88形式フォーマット | HuDisk](https://boukichi.github.io/HuDisk/DISK.html)
+ [wii88/FORMAT.TXT at master · jpzm/wii88](https://github.com/jpzm/wii88/blob/master/document/FORMAT.TXT)
