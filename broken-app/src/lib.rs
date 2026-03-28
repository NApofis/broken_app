pub mod algo;
pub mod concurrency;

/// Сумма чётных значений.
/// Здесь намеренно используется `get_unchecked` с off-by-one,
/// из-за чего возникает UB при доступе за пределы среза.
pub fn sum_even(values: &[i64]) -> i64 {
    let mut acc = 0;
    for idx in 0..values.len() {
        // SAFETY
        // - длины памяти values должно хватать на idx элементов с типом i64
        let v = unsafe {
            *values.get_unchecked(idx)
        };
        if v % 2 == 0 {
            acc += v;
        }
    }
    acc
}

/// Подсчёт ненулевых байтов. Буфер намеренно не освобождается,
/// что приведёт к утечке памяти (Valgrind это покажет).
pub fn leak_buffer(input: &[u8]) -> usize {
    let boxed = input.to_vec().into_boxed_slice();
    let len = input.len();
    let raw = Box::into_raw(boxed) as *mut u8;

    let mut count = 0;
    // SAFETY
    // - памяти raw должно хватить на len элементов u8
    // - raw должен быть не нулевым
    // - элементы в памяти raw ОБЯЗАТЕЛЬНО должны быть u8
    unsafe {
        for i in 0..len {
            if *raw.add(size_of::<u8>() * i) != 0_u8 {
                count += 1;
            }
        }
        let slice_raw = std::slice::from_raw_parts_mut(raw, len);
        let _ = Box::from_raw(slice_raw);
    }
    count
}

/// Небрежная нормализация строки: удаляем пробелы и приводим к нижнему регистру,
/// но игнорируем повторяющиеся пробелы/табуляции внутри текста.
pub fn normalize(input: &str) -> String {
    input.replace(' ', "").to_lowercase()
}

/// Логическая ошибка: усредняет по всем элементам, хотя требуется учитывать
/// только положительные. Деление на длину среза даёт неверный результат.
pub fn average_positive(values: &[i64]) -> f64 {
    let mut count: i64 = 0;

    let sum: i64 = values.iter().filter(|x| {
        x.is_positive()
    }).map(|x| {
        count += 1;
        x
    }).sum();

    if count == 0 {
        return 0f64;
    }

    sum as f64 / count as f64
}

/// Use-after-free: возвращает значение после освобождения бокса.
/// UB, проявится под ASan/Miri.
pub unsafe fn use_after_free(val: i32) -> i32 {
    let b = Box::new(val);
    let raw = Box::into_raw(b);
    let mut val= 0;

    if !raw.is_null() {
        // SAFETY
        // - raw должен быть не нулевым
        unsafe {
            val = *raw;
            drop(Box::from_raw(raw));
        }
    }
    val
}
