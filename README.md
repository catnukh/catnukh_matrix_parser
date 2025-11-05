# catnukh_matrix_parser
`catnukh_matrix_parser` - парсер для виконання операцій над матрицями, такими як: додавання, віднімання, множення на скаляр та множення двох матриць між собою

## Технічний опис
Початково дані читаються з файлу `.txt` за допомого парсера `pest`, який використовує правила граматики визначені в `src/matrix.pest`

### Етапи парсингу
1. Розпінавання тексту з файлу на конкретну дію, тобто визначення матриці, додавання матриць і так далі. Це відбувається за допомогою `pest`, що будує "дерево" ідентифікуючи команди, які необхідно виконати (add, subtract, mult...) та ігногуючи `COMMENT` і `WHITESPACE`
2. Код в `src/lib.rs` проходить по цьому дереву та перетворює його на структурований список команд `Vec<Command>`

### Використання результатів
1. CLI-додаток, тобто `src/main.rs` отримує `Vec<Command>`
2. Створюється `HashMap<String, Matrix>` для зберігання всіх визначених матриць за їхїніми іменами
3. Програма по черзі виконує команди з `Vec<Command>` звертаючись до `HashMap` для отримання матриць та виводу результату в консоль

### Як кнонувати собі репозиторій
1. Ввести команду `git clone` у консолі
   ```rust
   git clone
   ```
2. Перейдіть у щойно створену папку
   ```rust
   cd catnukh_matrix_parser
   ```
3. Запуск тестів. Для цього необхідно виконати команду `cargo test`
   ```rust
   cargo test
   ```

## Граматика (`src/matrix.pest`)
```rust
WHITESPACE = _{ " " | "\n" | "\t" | "\r" }
COMMENT = _{ "#" ~ (!"\n" ~ ANY)* }
number = { "-"? ~ ('0'..'9')+ ~ ("." ~ ('0'..'9')+)? ~ !(ASCII_ALPHA | ".") }
name_of_matrix = { (ASCII_ALPHA | "_") ~ (ASCII_ALPHANUMERIC | "_")* }
row = { "[" ~ number ~ ("," ~ number)* ~ "]" }
matrix = { "[" ~ row ~ ("," ~ row)* ~ "]" }
mat_def = { "mat" ~ name_of_matrix ~ "=" ~ matrix }
add = { "add" ~ name_of_matrix ~ "," ~ name_of_matrix }
subtract = { "sub" ~ name_of_matrix ~ "," ~ name_of_matrix }
mult = { "mul" ~ name_of_matrix ~ "," ~ name_of_matrix }
scale = { "scale" ~ name_of_matrix ~ "," ~ number }
operation = { mat_def | add | subtract | mult | scale }
file = { SOI ~ operation* ~ EOI }
```
