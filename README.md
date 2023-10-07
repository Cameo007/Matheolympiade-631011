# Matheolympiade-631011

Dieses Repository beinhaltet den Code der zum Ausrechnen der Quersummen (für Teil a & b) und Darstellen der Anzahl der Quersummen in Abhängigkeit von `s` (für Teil b) der 1. Aufgabe, der 1. Runde, der 63. Mathematikolympiade für die 9.- und 10. Klasse benutzt wurde.

## Anforderungen
- [Rust-Installation](https://rustup.rs/) (alternativ mit `nix-shell` für NixOS)

## Ausrechnen
#### berechnet alle Quersummen von `s=1` bis `s=100` nach dem vorgegebenen Muster und schreibt diese in `../Data/result_1-100.txt`

 1. `cargo run 1 100`

## Darstellen der Anzahl der Quersummen in Abhängigkeit von `s`
#### zählt die Anzahl Quersummen von `s=1` bis `s=100` (`../Data/result_counter_1-100.json`) und stellt sie in einem Koordinatensystem dar (`../Data/result_counter_1-100.png`)


1. `cargo run --count 1 100`

# TODO
- Plots in Rust generieren
- `.expect("TODO")` durch genauere Fehlermeldungen ersetzen
- [Multithreading](https://rust-lang-de.github.io/rustbook-de/ch16-01-threads.html)
