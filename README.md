# Uruchamianie

Trzeba jako argument podać endpoint do pobierania danych z websocketów, np.
- `cargo run -- ws://rekrutacja.westeurope.cloudapp.azure.com/maze1`

Będzie coś tam drukować, ale na końcu powinno pokazać: `Number of least turns at exit: 4`

# Zasada działania

Przeszukujemy labirynt metodą BFS. Jest kolejka FIFO, na którą wpychamy pola labiryntu i przy zdejmowaniu jakiegoś punktu aktualizujemy optymalną liczbę skrętów dla jego sąsiadów (założenie jest, że gdy wyciągamy punkt z kolejki, to doszliśmy do tego punktu wykonując możliwie najmniej zakrętów).

Pobieranie sąsiadów z websocketa odbywa się synchronicznie. Pewnie jest pole do popisu z robieniem tego asynchronicznie i jakimś zrównolegleniem pytania, ale już dostatecznie się zmęczyłem robiąc to zadanie, więc odpuściłem :P