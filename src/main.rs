//! lithicfont - 一個字體處理庫
//!
//! Copyright (c) 2026 ghimist
//! This Source Code Form is subject to the terms of the Mozilla Public
//! License, v. 2.0. If a copy of the MPL was not distributed with this
//! file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! ...
// 【編譯器自動注入的聲明（顯式展開）】
// extern crate core;
// extern crate std;
// use std::prelude::rust_2021::*;

use base64::Engine;
use sha2::Digest;
use std::env;
use std::fs;
use std::io::Write;
use std::time::SystemTime;

fn main() {
    // 獲取微秒級/納秒級時間戳（程序啓動瞬閒）
    let time_start: SystemTime = SystemTime::now();
    let duration_start: std::time::Duration = match time_start.duration_since(std::time::UNIX_EPOCH)
    {
        std::result::Result::Ok(d) => d,
        std::result::Result::Err(_) => std::panic!("系統時間早於 UNIX_EPOCH"),
    };
    let start_nanos: u128 = duration_start.as_nanos();

    // 顯式處理命令行參數
    let mut args_iterator: std::env::Args = std::env::args();
    let _executable_name: std::option::Option<std::string::String> = args_iterator.next();

    let arg_input_file: std::option::Option<std::string::String> = args_iterator.next();
    let arg_mode_flag: std::option::Option<std::string::String> = args_iterator.next();

    let input_path: std::string::String = match arg_input_file {
        std::option::Option::Some(path) => path,
        std::option::Option::None => {
            std::println!("用法: main <font_file.ttf> [--raw]");
            std::process::exit(1);
        }
    };

    let is_raw_mode: bool = match arg_mode_flag {
        std::option::Option::Some(flag) => match flag.as_str() {
            "--raw" => true,
            _ => false,
        },
        std::option::Option::None => false,
    };

    // 讀取二進制文件內容
    let file_bytes: std::vec::Vec<u8> = match std::fs::read(&input_path) {
        std::result::Result::Ok(bytes) => bytes,
        std::result::Result::Err(e) => {
            std::println!("讀取文件失敗: {}", e);
            std::process::exit(1);
        }
    };

    // 計算 SHA-256 哈希
    let mut hasher: sha2::Sha256 = sha2::Sha256::new();
    hasher.update(&file_bytes);
    let hash_result: sha2::digest::Output<sha2::Sha256> = hasher.finalize();
    let mut hash_string: std::string::String = std::string::String::new();

    // 拒絕 for 語法醣，使用顯式 loop 與 Iterator
    let mut hash_iter: std::slice::Iter<u8> = hash_result.as_slice().iter();
    loop {
        match hash_iter.next() {
            std::option::Option::Some(byte) => {
                let byte_hex: std::string::String = std::format!("{:02x}", byte);
                hash_string.push_str(&byte_hex);
            }
            std::option::Option::None => break,
        }
    }

    // 執行 Base64 編碼
    let base64_string: std::string::String =
        base64::engine::general_purpose::STANDARD.encode(&file_bytes);

    // 解析字體以提取 Unicode 範圍和 Family Name
    let font_face: ttf_parser::Face = match ttf_parser::Face::parse(&file_bytes, 0) {
        std::result::Result::Ok(face) => face,
        std::result::Result::Err(e) => {
            std::println!("解析字體結構失敗: {:?}", e);
            std::process::exit(1);
        }
    };

    // 獲取 Font Family Name (Name ID 1)
    let mut font_family: std::string::String = std::string::String::from("UnknownFont");
    let mut names_iter = font_face.names().into_iter();
    loop {
        match names_iter.next() {
            std::option::Option::Some(name) => {
                if name.name_id == ttf_parser::name_id::FAMILY {
                    match name.to_string() {
                        std::option::Option::Some(parsed_name) => {
                            font_family = parsed_name;
                            break;
                        }
                        std::option::Option::None => {}
                    }
                }
            }
            std::option::Option::None => break,
        }
    }

    // 計算連續的 Unicode 範圍
    let mut unicode_ranges: std::vec::Vec<(u32, u32)> = std::vec::Vec::new();
    let mut current_start: std::option::Option<u32> = std::option::Option::None;
    let mut current_end: std::option::Option<u32> = std::option::Option::None;
    let mut cp: u32 = 0;

    // 遍歷所有可能的 Unicode 碼位 (0 至 0x10FFFF)
    loop {
        if cp > 0x10FFFF {
            // 處理最後一段
            match current_start {
                std::option::Option::Some(start) => match current_end {
                    std::option::Option::Some(end) => {
                        unicode_ranges.push((start, end));
                    }
                    std::option::Option::None => {}
                },
                std::option::Option::None => {}
            }
            break;
        }

        let char_opt: std::option::Option<char> = std::char::from_u32(cp);
        let mut is_mapped: bool = false;

        match char_opt {
            std::option::Option::Some(c) => match font_face.glyph_index(c) {
                std::option::Option::Some(_) => {
                    is_mapped = true;
                }
                std::option::Option::None => {}
            },
            std::option::Option::None => {}
        }

        if is_mapped {
            match current_start {
                std::option::Option::None => {
                    current_start = std::option::Option::Some(cp);
                    current_end = std::option::Option::Some(cp);
                }
                std::option::Option::Some(_) => {
                    current_end = std::option::Option::Some(cp);
                }
            }
        } else {
            match current_start {
                std::option::Option::Some(start) => match current_end {
                    std::option::Option::Some(end) => {
                        unicode_ranges.push((start, end));
                        current_start = std::option::Option::None;
                        current_end = std::option::Option::None;
                    }
                    std::option::Option::None => {}
                },
                std::option::Option::None => {}
            }
        }
        cp += 1;
    }

    // 格式化 Unicode Range 字符串
    let mut unicode_range_str: std::string::String = std::string::String::new();
    let mut range_iter: std::slice::Iter<(u32, u32)> = unicode_ranges.iter();
    let mut is_first_range: bool = true;

    loop {
        match range_iter.next() {
            std::option::Option::Some(&(start, end)) => {
                if !is_first_range {
                    unicode_range_str.push_str(",\n        ");
                }
                is_first_range = false;
                if start == end {
                    unicode_range_str.push_str(&std::format!("U+{:04X}", start));
                } else {
                    unicode_range_str.push_str(&std::format!("U+{:04X}-{:04X}", start, end));
                }
            }
            std::option::Option::None => break,
        }
    }

    // 構建輸出內容與文件名
    let output_content: std::string::String;
    let output_filename: std::string::String;

    if is_raw_mode {
        output_content = base64_string;
        output_filename = std::format!("{}.raw.b64", input_path);
    } else {
        // 推斷格式 (woff2, woff, truetype, opentype)
        let format_hint: &str;
        if input_path.ends_with(".woff2") {
            format_hint = "woff2";
        } else if input_path.ends_with(".woff") {
            format_hint = "woff";
        } else if input_path.ends_with(".otf") {
            format_hint = "opentype";
        } else {
            format_hint = "truetype";
        }

        output_filename = std::format!("{}.b64.css", input_path);

        let time_output_start: SystemTime = SystemTime::now();
        let end_nanos: u128 = match time_output_start.duration_since(std::time::UNIX_EPOCH) {
            std::result::Result::Ok(d) => d.as_nanos(),
            std::result::Result::Err(_) => 0,
        };

        output_content = std::format!(
            "/* [LITHIC FONT EMBEDDER LOG] */\n\
             /* Start Timestamp (ns): {} */\n\
             /* Output Timestamp (ns): {} */\n\
             /* Source Hash (SHA-256): {} */\n\
             @font-face {{\n\
             \x20\x20font-family: '{}';\n\
             \x20\x20font-style: normal;\n\
             \x20\x20font-weight: 400;\n\
             \x20\x20font-display: swap;\n\
             \x20\x20font-feature-settings: \"liga\" 1, \"ccmp\" 1;\n\
             \x20\x20size-adjust: 100%;\n\
             \x20\x20src: url('data:font/{};base64,{}') format('{}');\n\
             \x20\x20unicode-range:\n\
             \x20\x20\x20\x20{};\n\
             }}\n",
            start_nanos,
            end_nanos,
            hash_string,
            font_family,
            format_hint,
            base64_string,
            format_hint,
            unicode_range_str
        );
    }

    // 寫入文件
    let mut out_file: std::fs::File = match std::fs::File::create(&output_filename) {
        std::result::Result::Ok(f) => f,
        std::result::Result::Err(e) => {
            std::println!("創建輸出文件失敗: {}", e);
            std::process::exit(1);
        }
    };

    match out_file.write_all(output_content.as_bytes()) {
        std::result::Result::Ok(_) => {
            std::println!("成功輸出至: {}", output_filename);
        }
        std::result::Result::Err(e) => {
            std::println!("寫入文件失敗: {}", e);
            std::process::exit(1);
        }
    }
}
