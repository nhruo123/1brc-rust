use std::fs::File;

use rustc_hash::FxHashMap;

struct StationData {
    min_report: f64,
    max_report: f64,
    report_sum: f64,
    report_count: usize,
}

const MEASUREMENTS_FILE_PATH: &str = "../1brc/measurements.txt";

fn main() {
    let file = File::open(MEASUREMENTS_FILE_PATH).unwrap();
    unsafe {
        let mmap = memmap2::MmapOptions::new()
            .map_copy_read_only(&file)
            .unwrap();

        compute(&mmap);
    }
}

unsafe fn compute(text: &[u8]) {
    let mut reports_map = FxHashMap::<&str, StationData>::default();

    let mut start_of_line_index = 0;
    let mut separator_index = 0;
    let mut current_index = 0;

    while current_index < text.len() {
        match &text[current_index] {
            b'\n' => {
                let name =
                    std::str::from_utf8_unchecked(&text[start_of_line_index..separator_index]);
                let mut reported_value =
                    parse_fixed_point_number(text.as_ptr().wrapping_add(current_index - 1));
                let is_neg = *text.get_unchecked(separator_index + 1) == b'-';
                reported_value *= !is_neg as i64 * 1 + is_neg as i64 * -1;
                let report_value = reported_value as f64 / 10.0;

                let entry = match reports_map.entry(name) {
                    std::collections::hash_map::Entry::Occupied(o) => o.into_mut(),
                    std::collections::hash_map::Entry::Vacant(v) => v.insert(StationData {
                        min_report: f64::MAX,
                        max_report: f64::MIN,
                        report_sum: 0.0,
                        report_count: 0,
                    }),
                };

                entry.max_report = entry.max_report.max(report_value);
                entry.min_report = entry.min_report.min(report_value);
                entry.report_sum += report_value;
                entry.report_count += 1;

                current_index += 1;
                start_of_line_index = current_index;
            }
            b';' => {
                separator_index = current_index;
            }
            _ => (),
        }
        current_index += 1;
    }
    let mut reports_array = reports_map.into_iter().collect::<Vec<_>>();
    reports_array.sort_by(|a, b| a.0.cmp(b.0));

    let mut output_str = String::new();
    output_str.push('{');

    let mut add_comma = false;

    for (name, report) in reports_array {
        if add_comma {
            output_str.push_str(", ");
        } else {
            add_comma = true;
        }

        let avg = (report.report_sum / report.report_count as f64 * 10.0).round() / 10.0;
        output_str.push_str(
            format!(
                "{}={:.1}/{:.1}/{:.1}",
                name, report.min_report, avg, report.max_report
            )
            .as_str(),
        );
    }

    output_str.push('}');

    println!("{}", output_str);
}

#[inline(always)]
unsafe fn parse_fixed_point_number(end_of_num_char: *const u8) -> i64 {
    let mut number = 0;
    let mut position = 1;

    for index in 0..=4 {
        let char = *(end_of_num_char.wrapping_sub(index));
        let is_good = char >= b'0' && char <= b'9';
        number += convert_from_ascii(char) as i64 * is_good as i64 * position;
        position *= (1 * !is_good as i64) + (10 * is_good as i64);
    }

    number
}

#[inline(always)]
fn convert_from_ascii(char: u8) -> i64 {
    const ASCII_OFFSET: u8 = b'0';
    (char.wrapping_sub(ASCII_OFFSET)) as _
}
