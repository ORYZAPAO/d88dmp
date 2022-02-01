d88dmp
=======
![](https://github.com/ORYZAPAO/d88dmp/workflows/Rust/badge.svg)
[![CircleCI](https://circleci.com/gh/ORYZAPAO/d88dmp/tree/main.svg?style=svg)](https://circleci.com/gh/ORYZAPAO/d88dmp/tree/main)

[English](/README.md)

はじめに
--------

日本のレトロPCエミュレータ用のディスクイメージ形式*.d88形式のファイルを、byte単位でテキストでダンプするツールです。  
d88ファイル解析およびRust勉強用として、個人的に作成したものです。  

出力内容
--------

下記情報を表示します。  
### ディスクヘッダ情報
  + ディスク名
  + ライトプロテクトの有無
  + ディスクの種類(2D/2DD/2HD)
  + ディスクサイズ(byte単位)
### 各トラックへのオフセットテーブル
### セクタヘッダ情報
  + シリンダ番号(0開始〜)
  + サイド情報(0:表、1:裏) 
  + セクタ番号(1開始〜) 
  + セクタサイズ(byte単位)
  + トラック中のセクタ数
  + 記録密度(単密度/倍密度/高密度)
  + 削除フラグ
  + ステータス
  + セクタのデータサイズ(byte単位)
### セクタのデータ
  + データをbyte単位でダンプ  

インストール方法
---------------------
rustのcargoコマンドから
```
cargo install d88dmp
```

動作環境/Download
------------
--> ![Download](https://github.com/ORYZAPAO/d88dmp/releases)

開発はLinux環境メイン（+ MacOS(Intel)、64bit版Windows10）で行っています。  
他の環境では試していません。

使い方
------
基本的な使い方は、コマンドライン等からD88形式のファイル名を指定するだけ。
```
 $ d88dmp <D88形式のファイル名>
```

## usage 
```
USAGE:
    d88dmp [OPTIONS] <*.D88>

ARGS:
    <*.D88>    D88 Disk Image

OPTIONS:
    -h, --help       ヘルプメッセージを表示
    -n, --no-info    ディスクやセクタのヘッダ情報を表示しない
        --no-color   カラー表示の無効化
    -V, --version    バージョン情報を表示

```

実行例）X1 turbo用のturbo CP/Mでファーマットしたディスクイメージ(CPM_data.d88)を表示  

```
$ .\d88dmp testdata/turboCP_M_formated_disk.d88 
```
![example](https://github.com/ORYZAPAO/d88dmp/blob/main/image/d88info_img.png?raw=true)

ビルド、コンパイル方法
---------------------

ソースコードからのコンパイルは [Rust](https://www.rust-lang.org) が必要です。  
GitHub から git clone したあと、ビルド方法は普通にcargoコマンドを叩くだけです。  
```
$ git clone git@github.com:ORYZAPAO/d88dmp.git
$ cargo build --release
```

ライセンス
----------
 MIT Licens

更新履歴
----------
+ [更新履歴](/CHANGELOG.md)

参考情報
---------------------

開発にあたり、こちらの情報を参考にさせていただきました。

+ [.D88形式のフロッピーディスクイメージフォーマット - （仮）](https://gra4.hatenadiary.jp/entry/20171108/1510096429)
+ [D88形式フォーマット | HuDisk](https://boukichi.github.io/HuDisk/DISK.html)
+ [wii88/FORMAT.TXT at master · jpzm/wii88](https://github.com/jpzm/wii88/blob/master/document/FORMAT.TXT)
