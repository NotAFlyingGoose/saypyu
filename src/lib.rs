// note: "done" means that it was double checked against either the Phillip West reference or saypyu.com

/// A map of certain combinations of IPA/OED/CED characters to their SaypYu versions.
/// This list is checked in order, and similar to a `match`, more complex patterns should be placed
/// before simpler patterns
const IPA_TO_SAYPYU_MULT: &[(&[&str], &str)] = &[
    (&["e:", "eː"], "ee"), // wiki
    (&["i:", "iː"], "ii"), // done 1 (OED & CED)
    (&["u:", "uː"], "uu"), // done (OED & CED)
    (&["eɪ"], "ey"),       // done
    (&["aɪ", "ʌɪ"], "ai"), // done 1 (IPA) 2 (OED)
    (&["ɔɪ"], "oy"),       // done
    (&["æʊ"], "aw"),
    (&["aʊ"], "ou"),       // done
    (&["oʊ", "əʊ"], "ow"), // done 1 (IPA) 2 (OED & CED)
    (&["əʊ"], "oh"),
    (&["ui"], "uy"),
    (
        &[
            "ɜr", // (IPA)
            "ɜ:r", "ɜːr", // custom (see the testing file. without this, stirring
            // becomes sturring [double R])
            "ɜ:", "ɜː", // (CED)
            "ɘ:", "ɘː", // (OED)
        ],
        "ur",
    ), // done all
    (
        &[
            "oʊr", // (IPA)
            "ɔ:r", "ɔːr", // (OED & CED)
        ],
        "oor",
    ), // done all
    (&["ɪər", "ɪə"], "iir"), // done 1 (IPA) 2 (OED)
    // note: the site actually uses "ir" instead of "iir", but this doesn't make much sense
    // SaypYu /i/ is like the first i in ice cream, and
    // SaypYu /ii/ is like the i in pizza.
    // clearly for words like beer, /biir/ makes much more sense than /bir/
    // for hero, /hiirow/ makes more sense than /hirow/
    // for ear, /iir/ makes more sense than /ir/
    //
    // this is also what SaypYu.com uses for "sere" (see ipa_to_saypyu.txt)
    (
        &[
            "ɛər", "εər", "eər", // (IPA [although technically IPA uses epsilon])
            "εə", "ɛə", "eə", // (CED [although technically CED uses e])
        ],
        "ayr",
    ), // done all
    // note: for this one above, manual research will reveal that OED uses "ər" for "air"
    // but elsewhere will use "ər" for "father". For this reason "ər" => "ayr" has been left
    // out
    (&["ʊə"], "ur"),            // done (CED & OED)
    (&["aɪər", "ʌɪə"], "aiɘr"), // done 1 (IPA) 2 (OED)
    (&["aɪər", "aʊə"], "ouɘr"), // done 1 (IPA) 2 (OED)
    (&["tʃ"], "tsh"),           // done
    (&["dʒ", "ʤ"], "j"),        // done

                                //(&["ju"], "yu"),            // custom. this was done to ensure "cube" IPA /kjuːb/ stays
                                // SaypYu /kyub/ instead of SaypYu /kyuub/
                                // todo: does this break anything, maybe should this
                                // match for IPA /ju:/ instead of IPA /ju/
];

/// Converts the given [IPA](https://en.wikipedia.org/wiki/International_Phonetic_Alphabet) pronounciation into [SaypYu](https://en.wikipedia.org/wiki/SaypYu)
///
/// Special support is also included for the pronounciations found in the OED and CED, and as a
/// result most IPA-like pronounciations found anywhere should work.
///
/// (although note that I haven't tested the CED yet)
///
/// ## Example
///
/// ```rust
/// // let's learn how to pronounce crustacean
/// let ipa = "krʌˈsteɪʃən";
/// let result = saypyu::ipa_to_saypyu(ipa);
/// assert_eq!(result, "krɘsteyshɘn");
/// ```
pub fn ipa_to_saypyu(ipa: &str) -> String {
    let mut res = String::with_capacity((ipa.len() as f32 * 1.5) as usize);

    let mut skip_until = 0;
    'main_loop: for (byte_idx, ch) in ipa.char_indices() {
        if byte_idx < skip_until {
            continue;
        }

        // this is a string over the current and remaining characters in the string
        let rest = &ipa[byte_idx..];
        for (tests, replace) in IPA_TO_SAYPYU_MULT {
            for test in *tests {
                if rest.starts_with(test) {
                    skip_until = byte_idx + test.len();
                    res.push_str(replace);
                    continue 'main_loop;
                }
            }
        }

        // references:
        // https://www.scribd.com/document/196954585/How-to-use-SaypYu-for-English-Speakers
        let new = match ch {
            'ō' => "oh",
            'ā' => "ay",
            'a' | 'æ' => "a",       // done 1 (OED), 2 (IPA)
            'ɑ' | 'ä' => "aa",      // done 1
            'e' | 'ε' | 'ɛ' => "e", // done 1 (OED), 2 (IPA), 3 (IPA)
            'ɪ' => "i",             // done
            'i' => "ii",            // done
            'o' | 'ɒ' => "o",       // done 2
            'ɔ' => "aw",            // done
            'ʊ' => "u",             // done 2
            'u' => "uu",            // done (IPA)
            'y' => "uy",
            'ə' | 'ʌ' => "ɘ",       // done 1, 2 // schwa
            'ø' | 'œ' => "ur",      // note: in original SaypYu this is /ɘɘ/
            'j' | 'ʎ' | 'ʝ' => "y", // done 1
            'w' => "w",
            'ʍ' => "hw",
            'ɥ' => "w",
            'b' => "b",
            'd' => "d",
            'f' => "f",
            'g' | 'ɡ' => "g",
            'h' => "h",
            'k' => "k",
            'x' => "kh", // done
            'l' => "l",
            'm' => "m",
            'n' => "n",
            'p' => "p",
            'q' => "q", // note: according to saypyu.com, english q is k, but ipa
            // q is q
            'r' | 'ɾ' | 'ɹ' => "r",
            's' => "s",
            't' => "t",
            'v' => "v",
            'z' => "z",
            'θ' => "th", // done
            'ð' => "dh", // done
            'ʃ' => "sh", // done
            'ʒ' => "j",  // done
            'ŋ' => "ng", // done
            '(' => "(",
            ')' => ")",
            ' ' => " ",
            _ => continue,
        };
        res.push_str(new);
    }

    res.shrink_to_fit();

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ipa_to_saypyu_file() {
        let text = std::fs::read_to_string("ipa_to_saypyu.test").unwrap();

        // I did this instead of printing as the loop went along bc I wanted pretty
        // formatting
        struct Failure<'a> {
            english_word: &'a str,
            ipa: &'a str,
            actual: String,
            expected: &'a str,
            line_no: usize,
        }

        let mut max_english_len = 0;
        let mut max_ipa_len = 0;
        let mut max_actual_len = 0;
        let mut max_expected_len = 0;

        let mut failures = Vec::new();
        let mut total = 0;

        for (line_no, line) in text.lines().enumerate() {
            if line.starts_with('#') {
                continue;
            }
            total += 1;

            let mut words = line.split_whitespace();
            let english_word = words.next().unwrap();

            while let Some(ipa) = words.next() {
                if ipa.starts_with('(') {
                    continue;
                }
                if ipa.starts_with('#') {
                    break;
                }

                let expected = words.next().unwrap();
                let actual = ipa_to_saypyu(ipa);

                if expected.contains('ə') {
                    println!(
                        "WARNING SaypYu /{expected}/ contains ə instead of ɘ! :{}",
                        line_no + 1
                    );
                }

                if expected != actual {
                    let english_len = english_word.chars().count();
                    if english_len > max_english_len {
                        max_english_len = english_len;
                    }

                    let ipa_len = ipa.chars().count();
                    if ipa_len > max_ipa_len {
                        max_ipa_len = ipa_len;
                    }

                    let actual_len = actual.chars().count();
                    if actual_len > max_actual_len {
                        max_actual_len = actual_len;
                    }

                    let expected_len = expected.chars().count();
                    if expected_len > max_expected_len {
                        max_expected_len = expected_len;
                    }

                    failures.push(Failure {
                        english_word,
                        ipa,
                        actual,
                        expected,
                        line_no: line_no + 1,
                    });
                }
            }
        }

        let failed = failures.len();

        if failed >= 1 {
            for Failure {
                english_word,
                ipa,
                actual,
                expected,
                line_no,
            } in failures
            {
                println!(
                "FAILED {english_word}{} IPA /{ipa}/{}=> /{actual}/{}(expected /{expected}/){}:{line_no}",
                " ".repeat(max_english_len.saturating_sub(english_word.chars().count())),
                " ".repeat((max_ipa_len + 1).saturating_sub(ipa.chars().count())),
                " ".repeat((max_actual_len + 1).saturating_sub(actual.chars().count())),
                " ".repeat((max_expected_len + 1).saturating_sub(expected.chars().count()))
            );
            }

            panic!(
                "{failed:>3} / {total:>3} tests failed ({:.1}%)",
                failed as f32 / total as f32 * 100.0
            );
        }
    }
}
