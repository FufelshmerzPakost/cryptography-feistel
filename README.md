# Реализация сетей фейстеля на Rust

## Генерация ключа
Раундовые ключи генерируются примитивным переставлением байт в векторе

## Операции f()
- xor часть блока + раундовый ключ
- Инверсия этого дела
- Побитовый сдвиг влево

## Общие параметры 
 - 10 раундов
 - блок 64 бит(8 байт)
 - ключ 32 бит(4 байта)

