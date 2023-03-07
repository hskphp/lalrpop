# lalrpop ワークショップ

## はじめに

これは Rust の lalrpop を使って比較的リーダブルな四則演算を協力して作るワークショップです。
単純な四則演算をするだけのプログラムですが、メインプログラマと協力者に分かれプルリクエストを送ったり受け取ったりしながら1つの言語を作成する経験を通して協力して言語を作る場合のノウハウを身に着けていただきます。

## 目的

一人で作業したほうが楽に作業が進む事が多いと思いますが、複数人での開発を経験をすることが目的ですのでご理解ください。

## 対象者

Rust を使ったワークショップですので参加者の方々は予め cargo をインストールして hello world が表示できるよう準備ください。

## 制作物の著作権

ご自由にお使いください。

## ポリシー(CoC)

迷惑行為はおやめください。

## Cargo.toml

ディレクトリを作り

```bash
$ cargo init
```

Cargo.toml を書き換えます。

```toml
[package]
name = "lalrpop"
version = "0.1.0"
edition = "2021"

build = "build.rs" # <-- We added this and everything after!
# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
lalrpop-util = { version = "0.19.8", features = ["lexer"] }

[build-dependencies]
lalrpop = "0.19.8"
```


```bash
$ cargo run
```

設定したlalrpopがインストールされてビルドされます。

```
:
hello world
```


ast.rs

```rust
use std::fmt::Debug;

#[derive(Clone, Debug)]
pub enum Expr {
    Int(i32),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
}
```
