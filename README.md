Opis

PyroHex to projekt symulacji spalania lasu na siatce heksagonalnej. Symulacja uwzględnia losowe rozmieszczenie drzew i propagację ognia między sąsiednimi komórkami. Program oferuje dwa tryby działania:

    --game: tryb gry z interaktywną wizualizacją w czasie rzeczywistym (z wykorzystaniem macroquad)

    --simulation: tryb automatycznej symulacji z generowaniem wykresu wyników (plotters)

## Instalacja

Wymagane zależności w Cargo.toml:
```
[dependencies]
macroquad = "*"
plotters = "*"
clap = { version = "*", features = ["cargo"] }
rand = "*"
```
## Uruchamianie
Tryb gry
```
cargo run -- --game --grid 30 50
```

![obraz](https://github.com/user-attachments/assets/d2138c05-a64f-4c6f-a5ab-dadef5a2ddee)


Tryb symulacji
```
cargo run -- --simulation --steps 100 --grid 30 50
```
![obraz](https://github.com/user-attachments/assets/57d83ebb-ecf1-482a-bceb-a9bc68b98f35)


## Parametry:

    --grid WIDTH HEIGHT – rozmiar siatki heksagonalnej (domyślnie 25 50)

    --steps – liczba kroków w trybie symulacji (wymagane przy --simulation)

## Architektura

    main.rs – uruchamianie programu, przetwarzanie argumentów CLI

    game.rs – logika trybu gry i rysowanie lasu w czasie rzeczywistym

    grid.rs – implementacja siatki heksagonalnej i operacji na niej

    symulation.rs – algorytm rozprzestrzeniania ognia w czasie

    plot.rs – generowanie wykresów z wynikami symulacji



