# Competitive_Template_Rust
AtCoderとかの競技プログラミングでRustでの標準入力を簡単にする関数とマクロです。

## input_line_str関数
標準入力から一行読み込んでStringに変換します。
改行文字は除去されています。
```Rust
let s = input_line_str();
```

## inputマクロ
標準入力から一行読み込み数値に変換します。
```Rust
let a:i32;
let b:i32;
let c:f64;

//標準入力に"4"が入力されると変数aに4が代入されます。
input!(a);

//標準入力に"2 4.3"が入力されると変数bに2,変数cに4.3が代入されます。
input!(b,c);
```

## input_vector2d関数
標準入力から指定した行数を読み込み二次元配列(Vec)に変換します。
```Rust
let a:Vec<Vec<isize>>;
//10行読み込む
a = input_vector2d(10);
```

## input_vector関数
標準入力から一行読み込み配列(Vec)に変換します。
```Rust
let a:Vec<isize>;
a = input_vector();
```

## str2vec関数
文字列を配列(Vec<char>)に変換します。
```Rust
let a = str2vec("abc");
//['a','b','c']

let b = str2vec(&input_line_str());
//標準入力に入力した文字列を配列に変換

```
