# broken_app

Тестовый репозиторий для практики поиска багов и оптимизации rust кода.

Репозиторий содержит два проекта:
1. broken-app - исходные проект с багами
2. reference-app - эталонный проект на который можно было ориентироваться

В ходе работ исходный проект изменялся, код оптимизировался, добавлялись отчеты. Ход изменений можно посмотреть по коммитам. Так же можно ориетироваться на текущий README. В нем записаны все необходимые ссылки на отчеты, код и изменения в коммитах.
Изначальный вид проекта broken-app можно посмотреть [тут](https://github.com/NApofis/broken_app/tree/aece99ecf5272b7e8b385adf890565f2aa5fabb8/broken-app)

## Этап первый

На первом этапе в проекте broken-app необходимо было:
1. Починить тесты
2. Устранить утечки памяти
3. Устранить ошибки работы с памятью

Перед изменениями были подготовлены предварительные отчеты
1. [cargo check](https://github.com/NApofis/broken_app/blob/2556c34043a96c76370d75630ef23b6f039a5cf5/checks/step-1/broken-app-before/cargocheck)
2. [cargo run --bin demo](https://github.com/NApofis/broken_app/blob/2556c34043a96c76370d75630ef23b6f039a5cf5/checks/step-1/broken-app-before/cargorunbin)
3. [cargo test](https://github.com/NApofis/broken_app/blob/2556c34043a96c76370d75630ef23b6f039a5cf5/checks/step-1/broken-app-before/cargotest)
4. [cargo +nightly miri test](https://github.com/NApofis/broken_app/blob/2556c34043a96c76370d75630ef23b6f039a5cf5/checks/step-1/broken-app-before/miri)
5. Результат `valgrind --leak-check=full cargo test` [тут](https://github.com/NApofis/broken_app/blob/2556c34043a96c76370d75630ef23b6f039a5cf5/checks/step-1/broken-app-before/valgrind)
6. Результат санитайзера `RUSTFLAGS="-Zsanitizer=thread" cargo +nightly test -Zbuild-std --target x86_64-unknown-linux-gnu --lib --tests --bins` [тут](https://github.com/NApofis/broken_app/blob/2556c34043a96c76370d75630ef23b6f039a5cf5/checks/step-1/broken-app-before/sanitizers-thread)
7. Результат санитайзера `RUSTFLAGS="-Zsanitizer=address" cargo +nightly test -Zbuild-std --target x86_64-unknown-linux-gnu --lib --tests --bins` [тут](https://github.com/NApofis/broken_app/blob/2556c34043a96c76370d75630ef23b6f039a5cf5/checks/step-1/broken-app-before/sanitizers-address)

Из отчетов видно что тесты падали с ошибкой. Проверка miri и valgrind показали ошибки работы с памятью. Так же не было тестов для метода `race_increment`.
Все изменения в коде для исправления всех ошибок, а так же измененные тесты можно посмотреть [тут](https://github.com/NApofis/broken_app/commit/80d8421630ec5b4743eddb0b72f17efb8d210748#diff-5f2072ea2e8440b350f007f5042d1bcb5acf685a75107fdd055cae6f59aa2e89)

После изменений так же были подготовлены отчеты
1. [cargo check](https://github.com/NApofis/broken_app/blob/80d8421630ec5b4743eddb0b72f17efb8d210748/checks/step-1/broken-app-after/cargocheck)
2. [cargo run --bin demo](https://github.com/NApofis/broken_app/blob/80d8421630ec5b4743eddb0b72f17efb8d210748/checks/step-1/broken-app-after/cargorunbin)
3. [cargo test](https://github.com/NApofis/broken_app/blob/80d8421630ec5b4743eddb0b72f17efb8d210748/checks/step-1/broken-app-after/cargotest)
4. [cargo +nightly miri test](https://github.com/NApofis/broken_app/blob/80d8421630ec5b4743eddb0b72f17efb8d210748/checks/step-1/broken-app-after/miri)
5. Результат `valgrind --leak-check=full cargo test` [тут](https://github.com/NApofis/broken_app/blob/80d8421630ec5b4743eddb0b72f17efb8d210748/checks/step-1/broken-app-after/valgrind)
6. Результат санитайзера `RUSTFLAGS="-Zsanitizer=thread" cargo +nightly test -Zbuild-std --target x86_64-unknown-linux-gnu --lib --tests --bins` [тут](https://github.com/NApofis/broken_app/blob/80d8421630ec5b4743eddb0b72f17efb8d210748/checks/step-1/broken-app-after/sanitizers-thread)
7. Результат санитайзера `RUSTFLAGS="-Zsanitizer=address" cargo +nightly test -Zbuild-std --target x86_64-unknown-linux-gnu --lib --tests --bins` [тут](https://github.com/NApofis/broken_app/blob/80d8421630ec5b4743eddb0b72f17efb8d210748/checks/step-1/broken-app-after/sanitizers-address)

Отчеты эталонного проекта можно посмтерть [тут](https://github.com/NApofis/broken_app/tree/80d8421630ec5b4743eddb0b72f17efb8d210748/checks/step-1/reference-app).  Для эталонного проекта выполнялись такие же команды.

## Этап второй

На этом этапе был проведен анализ работы бинарного проекта [demo](https://github.com/NApofis/broken_app/blob/e0045c317280223fae90bbd219e3c443ff93c39d/broken-app/src/bin/demo.rs). Анализ выполнялся при помощи `perf` и его визуализация через `flamegraph`. Что бы `perf` смог обнаружить проблемные места пришлось добавить многократный вызов методов (цикл for в методе main). Без этого вызываемые методы не попадали в отчет.

Для анализа были выполнены команды:
``` bash
cargo build --release
perf record -F 300 -g ./demo
perf script | /FlameGraph/stackcollapse-perf.pl | /FlameGraph/flamegraph.pl > flame.svg
```
Так же был [изменен](https://github.com/NApofis/broken_app/commit/e0045c317280223fae90bbd219e3c443ff93c39d#diff-6fd8445ca1aa425eae87d3bea7d7f679a49bc334402a39128a46c0a7da927a04) toml файл

Результат анализа
https://napofis.github.io/broken_app//checks/step-2/broken-app/flame.svg

На flame.svg видно что основную часть времени вызова demo отводится на метод  `broken_app::algo::slow_fib`. Из всего времени небольшая часть отводится на `asm_sysvec_apic_timer_interrup`.
Но основная работа все таки происходит в самом методе `slow_fib`. Оптимизация этого метода позволит значительно ускорить работу всего приложения.

Из flame.svg так же видно что рядом с `slow_fib` есть метод `broken_app::normalize`. Основное время работы метода отводится, но овобождение памяти. Оптимизация работу с алокацией и деаллокацие позволит улучшить работы метода и возможно убрать его из отчета flame.svg.

Из flame.svg так же видно что основную часть времени выполнения метода `main` отводится на `print`. То есть оптимизация других методов ни даст никакого прироста. Разве что если убрать вывод в консоль или аккумулировать сообщения перед выводом. Но даже в этом случае оптимизаци не даст большого прироста производительности. 


### Этап третий

Завершающий этап в результате которого необходимо доисправить все ошибки в проекте broken-app. Перед началом исправлений было решено 
1. доработать [тесты](https://github.com/NApofis/broken_app/commit/9feac9e6ea5ea322fcf10658e8d1db7a3d114203#diff-5f2072ea2e8440b350f007f5042d1bcb5acf685a75107fdd055cae6f59aa2e89)
2. доработать criterion бенчмарки что бы вывод был в формате [plot](https://github.com/NApofis/broken_app/commit/9feac9e6ea5ea322fcf10658e8d1db7a3d114203#diff-fceeabab231f8929503f2952ca1302e6c2c73ffcd3ae65a1e22364401cd6d806)
3. подправить [toml](https://github.com/NApofis/broken_app/commit/9feac9e6ea5ea322fcf10658e8d1db7a3d114203#diff-6fd8445ca1aa425eae87d3bea7d7f679a49bc334402a39128a46c0a7da927a04)

Бенчмарки запускались командой
``` bash
cargo bench --bench criterion
```

Предварительные результаты можно посмотреть [тут](https://napofis.github.io/broken_app//checks/step-3/broken-app/before/criterion/report/).
Результаты показали нузкую производительность методов `slow_fib_broken` и `slow_dedup_broken`. Метод `sum_even_broken` отработал нормально.

Что бы исправить найденные проблемы 
1. Был исправлен метод [slow_dedup](https://github.com/NApofis/broken_app/commit/03f8c389e4f98640b4f5924eb2340f7b1e9bf348#diff-ae42d937738c5381924044bd3210e2b219a8d006bbfe29a7be5282232638444eL2). Вся логика метода реализуется в стандартном контейнере BTreeSet который и сортирует и убирает дубликаты
2. Был исправлен метод [slow_fib](https://github.com/NApofis/broken_app/commit/03f8c389e4f98640b4f5924eb2340f7b1e9bf348#diff-ae42d937738c5381924044bd3210e2b219a8d006bbfe29a7be5282232638444eL22). Изменен алогиритм поиска чисел фибаначи из за чего удалось избавиться от рекурсии.
3. Изменен метод [sum_even](https://github.com/NApofis/broken_app/commit/03f8c389e4f98640b4f5924eb2340f7b1e9bf348#diff-eb4943c9ee3ec2beac689bded1b0376c6e6c8ada07e827b042d6d95764b08072L7). Для его работы не требуется unsafe код, а достаточно только фильтрации и сумирования через стандартные методы.
4. Изменен метод [normalize](https://github.com/NApofis/broken_app/commit/03f8c389e4f98640b4f5924eb2340f7b1e9bf348#diff-eb4943c9ee3ec2beac689bded1b0376c6e6c8ada07e827b042d6d95764b08072L48). Вместо двойного копирования строки, из за которой приходилось выделять память и сразу ее освобождать, был сделан один цикл for. Так же было сделано предвыделение памяти что поможет если исходная строка большая. Правда это же изменение сыграет плохую роль если строка будет состоять только из пробелов, но этот маловероятный сценарий.

Итоговые отчеты
1. Повторный результат `cargo bench --bench criterion` [тут](https://napofis.github.io/broken_app//checks/step-3/broken-app/after/criterion/report/)
2. [cargo test](https://github.com/NApofis/broken_app/blob/main/checks/step-3/broken-app/after/cargotest)
4. [cargo +nightly miri test](https://github.com/NApofis/broken_app/blob/main/checks/step-3/broken-app/after/miri)
5. Результат `valgrind --leak-check=full cargo test` [тут](https://github.com/NApofis/broken_app/blob/main/checks/step-3/broken-app/after/valgrind)
6. Результат санитайзера `RUSTFLAGS="-Zsanitizer=thread" cargo +nightly test -Zbuild-std --target x86_64-unknown-linux-gnu --lib --tests --bins` [тут](https://github.com/NApofis/broken_app/blob/main/checks/step-3/broken-app/after/sanitizers-thread)
7. Результат санитайзера `RUSTFLAGS="-Zsanitizer=address" cargo +nightly test -Zbuild-std --target x86_64-unknown-linux-gnu --lib --tests --bins` [тут](https://github.com/NApofis/broken_app/blob/main/checks/step-3/broken-app/after/sanitizers-address)
8. Итоговый `perf + flamegraph` [тут](https://napofis.github.io/broken_app//checks/step-3/broken-app/after/flame.svg)

Бенчмарки эталонного проекта можно посмотреть [тут](https://napofis.github.io/broken_app//checks/step-3/reference-app/criterion/report/)


