d88info
=======

はじめに
--------

レトロPCエミュレータ用のディスクイメージ形式*.d88形式のファイルを、byte単位でテキストでダンプするコマンドラインのツールです。  
d88ファイル解析およびRust勉強用として、個人的に作成したものです。  

出力内容
--------

下記情報を表示します。  
### ディスクヘッダ情報
  + ディスク名
  + ライトプロテクトの有無
  + ディスクの種類(2D/2DD/2HD)
  + ディスクサイズ(byte単位)
  + トラック情報の先頭へのオフセット
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

動作環境/Download
------------
+ Linux(ArchLinux)
+ Windows(64bit版、Intel)
+ MacOS(64bit版、Intel)
  
開発はLinux環境メインで行っています。  
他の環境では試していません。

Download
------

準備中  
（自分でビルドしてください）


使い方
------

基本的な使い方は、コマンドライン等からD88形式のファイル名を指定するだけ。
```
 $ d88info <D88形式のファイル名>
```
以上。

実行例）X1 turbo用のturbo CP/Mでファーマットしたディスクイメージ(CPM_data.d88)を表示  
```
$ d88info testdata/turboCP_M_formated_disk.d88 
Offset+0 +1 +2 +3 +4 +5 +6 +7 +8 +9 +a +b +c +d +e +f                 
----- -----------------------------------------------                 
00000 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 ................ Name(�����������������), WriteProtect(None), Type(2D Disk), DiskSize(348848 byte)
002b0 00 00 01 01 10 00 00 00 00 00 00 00 00 00 00 01 ................ Track(0), Side(0), Sector(1), Size(256 byte/sec), NumOfSec(16 sec/track), Status(OK), Density(00h), Mark(NORMAL), DataSize(256 byte), 
002c0 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 ................ 
002d0 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 ................ 
002e0 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 ................ 
002f0 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 ................ 
00300 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 ................ 
00310 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 ................ 
00320 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 ................ 
00330 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 ................ 
00340 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 ................ 
00350 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 ................ 
00360 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 ................ 
00370 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 ................ 
00380 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 ................ 
00390 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 ................ 
003a0 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 ................ 
003b0 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 ................ 
003c0 00 00 02 01 10 00 00 00 00 00 00 00 00 00 00 01 ................ Track(0), Side(0), Sector(2), Size(256 byte/sec), NumOfSec(16 sec/track), Status(OK), Density(00h), Mark(NORMAL), DataSize(256 byte), 
003d0 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 ................ 
003e0 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 ................ 
003f0 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 ................ 
00400 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 e5 ................ 
　　:
  (略)
　　:
```

## usage 
```
USAGE:
    d88info [OPTIONS] <*.D88>

ARGS:
    <*.D88>    D88 Disk Image

OPTIONS:
    -h, --help       ヘルプメッセージを表示
    -n, --noinfo     ディスクやセクタのヘッダ情報を表示しない
    -V, --version    バージョン情報を表示

```

ビルド、コンパイル方法
---------------------

ソースコードからのコンパイルは [Rust](https://www.rust-lang.org) が必要です。  
GitHub から git clone したあと、ビルド方法は普通にcargoコマンドを叩くだけです。  
```
$ git clone git@github.com:ORYZAPAO/d88info.git
$ cargo build --release
```
（Rustは開発環境の構築が楽チンで良いデスネ。  
　特にWindowsのC++でライブラリをいろいろ拾ってくるのは、すごく面倒・・・）


ライセンス
----------
 MIT License

更新履歴
----------
+ ver0.11_211212  calp 3.0.0-rc.4対応。Cargo.lockを登録。Cargo.lockを登録しておかないと、新たにビルドしたとき、新旧のクレート依存で失敗するのね。
+ ver0.1_211129  初版


参考情報
---------------------

開発にあたり、こちらの情報を参考にさせていただきました。

+ [.D88形式のフロッピーディスクイメージフォーマット - （仮）](https://gra4.hatenadiary.jp/entry/20171108/1510096429)
+ [D88形式フォーマット | HuDisk](https://boukichi.github.io/HuDisk/DISK.html)
+ [wii88/FORMAT.TXT at master · jpzm/wii88](https://github.com/jpzm/wii88/blob/master/document/FORMAT.TXT)
