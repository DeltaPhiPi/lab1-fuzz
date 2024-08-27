Herramienta de CLI para generar casos de prueba para el primer laboratorio del curso de Programación 2 de FING.
Uso:
```
cargo run -- <{grupo | fecha | persona}> <tamaño>
```
Genera en stdout un caso de prueba para el módulo correspondiente de un tamaño dado, el tamaño no corresponde directamente a la cantidad de comandos ya que se generan de manera aleatoria y para ciertos casos se necesitan varios comandos para que el programa quede en un estado válido.
