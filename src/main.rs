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
    let mut reports_map = FxHashMap::<&str, StationData>::default();
    let file = File::open(MEASUREMENTS_FILE_PATH).unwrap();
    let mmap = unsafe {
        memmap2::MmapOptions::new()
            .map_copy_read_only(&file)
            .unwrap()
    };

    let mut start_of_line_index = 0;
    let mut separator_index = 0;
    let mut current_index = 0;

    while current_index < mmap.len() {
        match &mmap[current_index] {
            b'\n' => {
                let name = unsafe {
                    std::str::from_utf8_unchecked(&mmap[start_of_line_index..separator_index])
                };
                let report_value =
                    parse_fixed_point_number(&mmap[separator_index + 1..current_index]) as f64
                        / 10.0;

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
pub fn parse_fixed_point_number(buffer: &[u8]) -> i64 {
    const ASCII_OFFSET: u8 = b'0';
    let is_negative = buffer[0] == b'-';
    let mut number = 0;
    let mut index = is_negative as usize;
    while index < buffer.len() - 2 {
        number *= 10;
        number += (buffer[index] - ASCII_OFFSET) as i64;
        index += 1;
    }

    number *= 10;
    number += (buffer[buffer.len() - 1] - ASCII_OFFSET) as i64;

    // this is branchless even with the if the compiler is smart 
    number * if is_negative { -1 } else { 1 }
}
