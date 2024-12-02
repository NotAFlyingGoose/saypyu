# SaypYu in Rust

[SaypYu ("Spell as you pronounce, universally")](https://en.wikipedia.org/wiki/SaypYu) is a phonetic alphabet of 24 letters to spell languages, including English. It allows one to very easily determine how to pronounce words by spelling them as they are pronounced.

This crate allows one to convert the mighty and complex [International Phonetic Alphabet (IPA)](https://en.wikipedia.org/wiki/International_Phonetic_Alphabet) into something far more readible and usable by the average person. This was originally a part of my other project, [The Everything Dictionary](https://everythingdictionary.com), in which I used it to allow non-linguists to learn the pronunciations of words without being forced to consult with complicated pronunciation keys.

Learning how to pronounce words should be simple, fast, and accurate.

I decided to make the code open-source since there's next to no resources online on how to convert IPA to SaypYu. Even the section about it on the SaypYu Wikipedia page is very incomplete. And this happens to be a very useful thing when it's been programmed well.

## Example

```rust
// let's learn how to pronounce crustacean
let ipa = "krʌˈsteɪʃən";
let result = saypyu::ipa_to_saypyu(ipa);
assert_eq!(result, "krɘsteyshɘn");
```

In case you're wondering, 'ɘ', called schwa, is the only special letter you need to know in SaypYu. It's pronounced like the first letter in the words "ago" and "about". Compare that to the IPA just above it and you'll see just how simple SaypYu is!

The file `[ipa_to_saypyu.test](./ipa_to_saypyu.test)` is used for testing but also contains a good many different English words with their IPA and SaypYu pronunciations

## Notes

There were three primary sources used in creating the map from IPA to SaypYu. These were all extrememly useful but even official sources were sometimes contradictory or illogical.
This crate has been made to be as consistent and rational as possible and as a result might disagree with some SaypYu found in the wild. Even so, if you find something wrong, or just aren't certain about something, please make an issue! I'd love to make this crate as good as possible 😄

This crate does use a few reforms to SaypYu suggested by Phillip West as they greatly help in making SaypYu even more easily understandable and immediately usable to someone who knows nothing about it. If someone needs to learn what each pattern of letters means before they can use the alphabet, that defeats the entire purpose.

Sources:

- [saypyu.com](https://web.archive.org/web/20160402175957/http://saypyu.com/index.php)
- [Converting English Words into SaypYu by Phillip West](https://www.scribd.com/document/196958386/SaypYu-Conversions)
- [Necessary Reforms to SayPyu : The Next Steps by Phillip West](https://www.scribd.com/document/196958655/Reforming-SaypYu-Next-Steps)

## License

This crate is distributed under the terms of the GNU GPLv3 license. See [LICENSE](./LICENSE) for details.
