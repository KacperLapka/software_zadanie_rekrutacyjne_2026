# Co zostało zrobione
 - Wystrzeliwanie rakiety z łagodnym lądowaniem sa pomocą skryptu w pythonie w `main.py`
 - przepisanie najważniejszych rzeczy z biblioteki communication-library(dekodowanie i kodowanie pakietów)
 - kod w rust nasłuchuje i wyświetla wartości wysyłane przez rakietę robi to asynchronicznie wyświetlanie jest osobnym wątkiem

# Co nie zostało zrobione
 - Sekwencja startu z poziomu kodu rusta - nie zdążyłem :(
 - zarządzanie pakietami za pomocą piorytetu - tu nie dokońca rozumiem czy software w ogóle ich używa czy to jest tylko dla rakiety, jeśli software też to są 2 osobne kolejki jedna dla `HIGH` druga dla `LOW`?

# Plany
Jeśli chodzi o to jak implementacja sekwencji startu miała by wyglądać dla kodu w rust to planowałem zrobić taki miniframework tak jak to było w kodzie `main.py` chodzi mi ti o klasy `ServoManager`, `RelaysManager` i enum `Toggle`.
