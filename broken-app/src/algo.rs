use std::collections::BTreeSet;

/// Преобразуем к красно черному дереву заодно отсеев дубликаты и отсортировав
pub fn slow_dedup(values: &[u64]) -> Vec<u64> {
    let out = BTreeSet::from_iter(values.into_iter().cloned());
    out.into_iter().collect::<Vec<u64>>()
}

/// Классическая экспоненциальная реализация без мемоизации — будет медленной на больших n.
pub fn slow_fib(n: u64) -> u64 {
    let mut i0 = 0;
    let mut i1 = 1;
    let mut result = 1;
    if n < 2 {
        return n;
    }
    for _ in 2..=n {
        result = i1 + i0;
        i0 = i1;
        i1 = result
    }
    result
}
