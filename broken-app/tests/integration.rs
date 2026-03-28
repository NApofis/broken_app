use std::thread;
use std::time::Duration;
use broken_app::{algo, leak_buffer, normalize, average_positive, sum_even, use_after_free};
use broken_app::concurrency::{race_increment, reset_counter, read_after_sleep, COUNTER};

#[test]
fn sums_even_numbers() {
    // Разный набор данных
    let nums = [1, 2, 3, 4];
    assert_eq!(sum_even(&nums), 6);

    // Только не четные что бы ничего не сумировалось
    let nums = [1, 3, 5, 7];
    assert_eq!(sum_even(&nums), 0);

    // Пустой срез
    let nums = [];
    assert_eq!(sum_even(&nums), 0);

    // Только четные что бы сумировалось все что прислали
    let nums = [2, 4, 6, 8];
    assert_eq!(sum_even(&nums), 20);
}

#[test]
fn counts_non_zero_bytes() {
    // Разный набор данных
    let data = [0_u8, 1, 0, 2, 3];
    assert_eq!(leak_buffer(&data), 3);

    // Только нули что бы ничего не подсчитывалось
    let data = [0_u8, 0, 0, 0];
    assert_eq!(leak_buffer(&data), 0);

    // Пустой срез
    let data = [];
    assert_eq!(leak_buffer(&data), 0);

    // Единичный срез
    let data = [1];
    assert_eq!(leak_buffer(&data), 1);
}

#[test]
fn dedup_preserves_uniques() {
    let uniq = algo::slow_dedup(&[5, 5, 1, 2, 2, 3]);
    assert_eq!(uniq, vec![1, 2, 3, 5]); // порядок и состав важны
}

#[test]
fn fib_small_numbers() {
    assert_eq!(algo::slow_fib(10), 55);
}

#[test]
fn normalize_simple() {
    assert_eq!(normalize(" Hello World "), "helloworld");
}

#[test]
fn averages_only_positive() {
    let nums = [-5, 5, 15];
    assert!((average_positive(&nums) - 10.0).abs() < f64::EPSILON);

    let nums = [1, 2, 3, 4];
    assert_eq!(average_positive(&nums), 2.5);

    let nums = [-10, 0, 5, 15];
    assert_eq!(average_positive(&nums), 10.0);

    let nums = [-1, -2, -3];
    assert_eq!(average_positive(&nums), 0.0);

    let nums: [i64; 0] = [];
    assert_eq!(average_positive(&nums), 0.0);

    let nums = [1_000_000, 2_000_000, -5];
    assert_eq!(average_positive(&nums), 1_500_000.0);
}

#[test]
fn race_increment_is_correct() {
    let total = race_increment(1_000, 4);
    assert_eq!(total, 4_000);

    let _ = race_increment(100, 2);
    reset_counter();
    assert_eq!(read_after_sleep(), 0);


    reset_counter();
    let handle = thread::spawn(|| {
        thread::sleep(Duration::from_millis(1));
        COUNTER.store(777, std::sync::atomic::Ordering::Release);
    });

    let result = read_after_sleep();
    let _ = handle.join();
    assert!(result == 0 || result == 777);

}

#[test]
fn test_free_raw() {
    let result = unsafe {
        use_after_free(42)
    };
    assert_eq!(result, 42);

    let result = unsafe {
        use_after_free(0)
    };
    assert_eq!(result, 0);

    let result = unsafe {
        use_after_free(-1)
    };
    assert_eq!(result, -1);
}