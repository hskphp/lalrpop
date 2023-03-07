## やってみたいワークショップ一覧

## 初級者向けワークショップ

- 四則演算ワークショップ
- ラムダ計算ワークショップ
- みんな大好きフィボナッチが動くラムダ計算ワークショップ
- スタックマシンコンパイラワークショップ
- レジスタマシンワークショップ

## 中級者向けワークショップ


## 上級者向けワークショップ


## 言語別ワークショップ

## 自作言語ワークショップ


Syntax 

    x   ∈ Variables
    eff ∈ Effects

    v ::= x | h | λx.e | ef
    e ::= v | e e | let x = e in e
            | inst () | with v handle e
            | performe e e
    h ::= handler v (val x -> e) ((x, k) -> e)

    F ::= e □ | □ v | let x = □ in e
        | with v handle □ | performe □ e | perfome v □
    s ::= [] | F :: s

Figure 1: the syntax of λeff

Semantics

    flatfn []     = λx.x
    flatfn (F::s) = λx.flatfn s (F [x])

Figure 2: utility function flatfn

    <F[e]; s; es> |-> <e; F :: s; es>                        (Push)
    <v; F::s; es> |-> <F[v]; s; es>                          (Pop)
    <v; []; es>   |-> <v; []; es>                            (Result)
    <λx.e; (□ v)::s; es> |-> <e[x=v]; s; es>                 (Apply)
    <inst(); s; es> |-> <eff; s; es>                         (Instanciate)
    <perform eff v; F::s; es> |-> <perform eff v; s; F::es>  (Rethrow)
    <perform eff v; F::s; es> |-> <e_eff[x=v,k=flatfn es]; F::s; []> (HandleEff)
    where F = with h handle □
          h = handler eff (val x -> ev)((x,k) -> eeff)
    <v; F::s; es> |-> <ev[x = v]; s; es>                     (HandleV)
    where F = with h handle □
          h = handler eff (val x -> ev)((x,k) -> eeff)
    <perform eff v; []; es> |- abort                         (Leak)

Figure 3: the semantics of λeff
