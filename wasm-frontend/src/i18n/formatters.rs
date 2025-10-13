pub fn to_persian_digits(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            '0' => '۰', '1' => '۱', '2' => '۲', '3' => '۳', '4' => '۴',
            '5' => '۵', '6' => '۶', '7' => '۷', '8' => '۸', '9' => '۹',
            _ => c,
        })
        .collect()
}

pub fn format_number_persian(n: i64) -> String {
    let s = n.to_string();
    let mut result = String::new();
    
    for (i, c) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 && c != '-' {
            result.insert(0, '٬');
        }
        result.insert(0, c);
    }
    
    to_persian_digits(&result)
}

pub fn format_date_jalali(year: i32, month: u32, day: u32) -> String {
    let month_names = [
        "فروردین", "اردیبهشت", "خرداد", "تیر", "مرداد", "شهریور",
        "مهر", "آبان", "آذر", "دی", "بهمن", "اسفند",
    ];
    
    format!(
        "{} {} {}",
        to_persian_digits(&day.to_string()),
        month_names[(month - 1) as usize],
        to_persian_digits(&year.to_string())
    )
}

