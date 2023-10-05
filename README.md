# Matheolympiade-631011

Dieses Repository beinhaltet den Code der zum Ausrechnen der Quersummen (für Teil a & b) und Darstellen der Anzahl der Quersummen in Abhängigkeit von `s` (für Teil b) der 1. Aufgabe, der 1. Runde, der 63. Mathematikolympiade für die 9.- und 10. Klasse benutzt wurde.

## Ausrechnen

- benötigt eine Rust-Installation (alternativ `shell.nix` für nixOS)

 1. `cargo run 1 100` (berechnet alle Quersummen von `s=1` bis `s=100` nach dem vorgegebenen Muster und schreibt diese in `../Data/result_1-100.txt`)

## Darstellen der Anzahl der Quersummen in Abhängigkeit von `s`

- benötigt eine Python-Installation (alternativ `shell.nix` für nixOS)

1. `cd Calculator`
2. `cargo run --count 1 100` (zählt die Anzahl Quersummen von `s=1` bis `s=100` nach dem vorgegebenen Muster und schreibt diese in `../Data/result_counter_1-100.json`)
3. cd `../Visualizer`
4. pip install -r requirements.txt
5. `python3 main.py ../Data/result_counter_1-100.json` (Stellt die Daten in einem Koordinatensystem dar, welches unter `../Data/result_counter_1-100.png` gespeichert wird)

# TODO
- (Plots in Rust generieren)
