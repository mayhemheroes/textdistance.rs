#![no_main]

use libfuzzer_sys::fuzz_target;
use std::str;
use textdistance::{
    Algorithm, 
    Bag, Cosine, DamerauLevenshtein, EntropyNCD, Hamming, Jaccard, Jaro, JaroWinkler,
    LCSSeq, LCSStr, Length, Levenshtein, Overlap, Prefix, RatcliffObershelp, Roberts,
    Sift4Simple, SmithWaterman, SorensenDice, Suffix, Tversky, YujianBo, LIG3, MLIPNS,
};

fuzz_target!(|data: &[u8]| {
    if data.len() > 3 {
        let opt = data[0];
        let str1_bytes = &data[1..data.len() / 2];
        let str2_bytes = &data[data.len() / 2 + 1..];
        match str::from_utf8(&str1_bytes) {
            Ok(str1) => match str::from_utf8(&str2_bytes) {
                Ok(str2) => match opt {
                    0 => {
                        Bag::default().for_str(str1, str2);
                    }
                    1 => {
                        Cosine::default().for_str(str1, str2);
                    }
                    2 => {
                        DamerauLevenshtein::default().for_str(str1, str2);
                    }
                    3 => {
                        EntropyNCD::default().for_str(str1, str2);
                    }
                    4 => {
                        Hamming::default().for_str(str1, str2);
                    }
                    5 => {
                        Jaccard::default().for_str(str1, str2);
                    }
                    6 => {
                        Jaro::default().for_str(str1, str2);
                    }
                    7 => {
                        JaroWinkler::default().for_str(str1, str2);
                    }
                    8 => {
                        LCSSeq::default().for_str(str1, str2);
                    }
                    9 => {
                        LCSStr::default().for_str(str1, str2);
                    }
                    10 => {
                        Length::default().for_str(str1, str2);
                    }
                    11 => {
                        Levenshtein::default().for_str(str1, str2);
                    }
                    12 => {
                        Overlap::default().for_str(str1, str2);
                    }
                    13 => {
                        Prefix::default().for_str(str1, str2);
                    }
                    14 => {
                        RatcliffObershelp::default().for_str(str1, str2);
                    }
                    15 => {
                        Roberts::default().for_str(str1, str2);
                    }
                    16 => {
                        Sift4Simple::default().for_str(str1, str2);
                    }
                    17 => {
                        SmithWaterman::default().for_str(str1, str2);
                    }
                    18 => {
                        SorensenDice::default().for_str(str1, str2);
                    }
                    19 => {
                        Suffix::default().for_str(str1, str2);
                    }
                    20 => {
                        Tversky::default().for_str(str1, str2);
                    }
                    21 => {
                        YujianBo::default().for_str(str1, str2);
                    }
                    22 => {
                        LIG3::default().for_str(str1, str2);
                    }
                    23 => {
                        MLIPNS::default().for_str(str1, str2);
                    }
                    _ => (),
                },
                _ => (),
            },
            _ => (),
        }
    }
});
