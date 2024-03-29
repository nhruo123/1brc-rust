use std::{
    fs::File,
    process::exit,
    thread::{self, available_parallelism},
};

use rustc_hash::FxHashMap;

struct StationData {
    min_report: i64,
    max_report: i64,
    report_sum: i64,
    report_count: usize,
}

const MEASUREMENTS_FILE_PATH: &str = "../1brc/measurements.txt";
const MAX_GROUP_NAME_LEN: usize = 100;
const MAX_GROUPS: usize = 512;

fn main() {
    let file = File::open(MEASUREMENTS_FILE_PATH).unwrap();
    let task_count: usize = available_parallelism().unwrap().into();

    unsafe {
        let mmap = memmap2::MmapOptions::new()
            .map_copy_read_only(&file)
            .unwrap();

        let file_size = mmap.len();
        let segment_size = file_size / task_count;

        let result_map = thread::scope(|s| {
            let buffer = &mmap;
            let mut tasks: Vec<_> = (1..task_count - 1)
                .map(|index| s.spawn(move || compute(buffer, index * segment_size, segment_size)))
                .collect();

            tasks.push(s.spawn(move || {
                compute(
                    buffer,
                    (task_count - 1) * segment_size,
                    segment_size + file_size % task_count,
                )
            }));

            let result_map = compute(buffer, 0, segment_size);

            tasks.into_iter().fold(result_map, |mut result_map, next_result| {
                let result = next_result.join().unwrap();
                for (key, value) in result {
                    let entry = match result_map.entry(key) {
                        std::collections::hash_map::Entry::Occupied(o) => o.into_mut(),
                        std::collections::hash_map::Entry::Vacant(v) => v.insert(StationData {
                            min_report: i64::MAX,
                            max_report: i64::MIN,
                            report_sum: 0,
                            report_count: 0,
                        }),
                    };

                    entry.max_report = entry.max_report.max(value.max_report);
                    entry.min_report = entry.min_report.min(value.min_report);
                    entry.report_sum += value.report_sum;
                    entry.report_count += value.report_count;
                }

                result_map
            })
        });

        let mut reports_array: Vec<_>= result_map.into_iter().collect();
        reports_array.sort_unstable_by_key(|v| v.0);
        print_result(reports_array);

        exit(0);
    }
}

unsafe fn compute(text: &[u8], start_offset: usize, len: usize) -> FxHashMap<&str, StationData> {
    let mut reports_map = FxHashMap::<&str, StationData>::default();
    reports_map.reserve(MAX_GROUPS * 4);

    let mut current_index = start_offset;

    if current_index > 0 {
        while *text.get_unchecked(current_index) != b'\n' {
            current_index += 1;
        }
        current_index += 1;
    }

    let mut start_of_line_index = current_index;
    let mut separator_index = current_index;

    while current_index < start_offset + len
        || (start_offset + len < text.len() && *text.get_unchecked(current_index - 1) != b'\n')
    {
        match text.get_unchecked(current_index) {
            b'\n' => {
                let name = std::str::from_utf8_unchecked(std::slice::from_raw_parts(
                    text.as_ptr().offset(start_of_line_index as _),
                    separator_index - start_of_line_index,
                ));

                let is_neg = *text.get_unchecked(separator_index + 1) == b'-';
                let num_len = current_index - separator_index - 3 - is_neg as usize;
                let mut report_value = parse_2_digit_number(
                    text.as_ptr()
                        .offset((separator_index + 1 - (2 - num_len) + is_neg as usize) as _),
                    num_len,
                ) as i64
                    * 10;

                report_value += convert_from_ascii(*text.get_unchecked(current_index - 1));
                report_value *= !is_neg as i64 * 1 + is_neg as i64 * -1;

                let entry = match reports_map.entry(name) {
                    std::collections::hash_map::Entry::Occupied(o) => o.into_mut(),
                    std::collections::hash_map::Entry::Vacant(v) => v.insert(StationData {
                        min_report: i64::MAX,
                        max_report: i64::MIN,
                        report_sum: 0,
                        report_count: 0,
                    }),
                };

                entry.max_report = entry.max_report.max(report_value);
                entry.min_report = entry.min_report.min(report_value);
                entry.report_sum += report_value;
                entry.report_count += 1;

                start_of_line_index = current_index + 1;
            }
            b';' => {
                separator_index = current_index;
            }
            _ => (),
        }
        current_index += 1;
    }

    reports_map
}

fn print_result(reports_array: Vec<(&str, StationData)>) {
    let mut output_str = String::with_capacity((MAX_GROUP_NAME_LEN + (3 * 4)) * MAX_GROUPS + 2);
    output_str.push('{');

    let mut add_comma = false;

    for (name, report) in reports_array {
        if add_comma {
            output_str.push_str(", ");
        } else {
            add_comma = true;
        }

        let avg =
            (report.report_sum as f64 / 10.0 / report.report_count as f64 * 10.0).round() / 10.0;
        output_str.push_str(
            format!(
                "{}={:.1}/{:.1}/{:.1}",
                name,
                report.min_report as f64 / 10.0,
                avg,
                report.max_report as f64 / 10.0
            )
            .as_str(),
        );
    }

    output_str.push('}');

    println!("{}", output_str);
}

#[inline(always)]
unsafe fn parse_2_digit_number(ptr: *const u8, len: usize) -> u16 {
    const ZERO_MASK: u16 = 0x3030;

    let general_mask: u16 = !(0xFFFF_u32 >> (len as u32 * 8)) as u16;

    let mut number: u16 = (ptr as *const u16).read_unaligned();
    number &= general_mask;

    number -= ZERO_MASK & general_mask;

    let lower_digit = (number >> 8) & 0xFF;
    let upper_digit = ((number) & 0xFF) * 10;

    let result = lower_digit + upper_digit;

    result
}

#[inline(always)]
fn convert_from_ascii(char: u8) -> i64 {
    const ASCII_OFFSET: u8 = b'0';
    (char.wrapping_sub(ASCII_OFFSET)) as _
}
