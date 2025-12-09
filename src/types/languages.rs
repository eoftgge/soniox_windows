use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub enum LanguageHint {
    #[serde(rename = "af")]
    Afrikaans,
    #[serde(rename = "sq")]
    Albanian,
    #[serde(rename = "ar")]
    Arabic,
    #[serde(rename = "az")]
    Azerbaijani,
    #[serde(rename = "eu")]
    Basque,
    #[serde(rename = "be")]
    Belarusian,
    #[serde(rename = "bn")]
    Bengali,
    #[serde(rename = "bs")]
    Bosnian,
    #[serde(rename = "bg")]
    Bulgarian,
    #[serde(rename = "ca")]
    Catalan,
    #[serde(rename = "zh")]
    Chinese,
    #[serde(rename = "hr")]
    Croatian,
    #[serde(rename = "cs")]
    Czech,
    #[serde(rename = "da")]
    Danish,
    #[serde(rename = "nl")]
    Dutch,
    #[serde(rename = "en")]
    English,
    #[serde(rename = "et")]
    Estonian,
    #[serde(rename = "fi")]
    Finnish,
    #[serde(rename = "fr")]
    French,
    #[serde(rename = "gl")]
    Galician,
    #[serde(rename = "de")]
    German,
    #[serde(rename = "el")]
    Greek,
    #[serde(rename = "gu")]
    Gujarati,
    #[serde(rename = "he")]
    Hebrew,
    #[serde(rename = "hi")]
    Hindi,
    #[serde(rename = "hu")]
    Hungarian,
    #[serde(rename = "id")]
    Indonesian,
    #[serde(rename = "it")]
    Italian,
    #[serde(rename = "ja")]
    Japanese,
    #[serde(rename = "kn")]
    Kannada,
    #[serde(rename = "kk")]
    Kazakh,
    #[serde(rename = "ko")]
    Korean,
    #[serde(rename = "lv")]
    Latvian,
    #[serde(rename = "lt")]
    Lithuanian,
    #[serde(rename = "mk")]
    Macedonian,
    #[serde(rename = "ms")]
    Malay,
    #[serde(rename = "ml")]
    Malayalam,
    #[serde(rename = "mr")]
    Marathi,
    #[serde(rename = "no")]
    Norwegian,
    #[serde(rename = "fa")]
    Persian,
    #[serde(rename = "pl")]
    Polish,
    #[serde(rename = "pt")]
    Portuguese,
    #[serde(rename = "pa")]
    Punjabi,
    #[serde(rename = "ro")]
    Romanian,
    #[serde(rename = "ru")]
    Russian,
    #[serde(rename = "sr")]
    Serbian,
    #[serde(rename = "sk")]
    Slovak,
    #[serde(rename = "sl")]
    Slovenian,
    #[serde(rename = "es")]
    Spanish,
    #[serde(rename = "sw")]
    Swahili,
    #[serde(rename = "sv")]
    Swedish,
    #[serde(rename = "tl")]
    Tagalog,
    #[serde(rename = "ta")]
    Tamil,
    #[serde(rename = "te")]
    Telugu,
    #[serde(rename = "th")]
    Thai,
    #[serde(rename = "tr")]
    Turkish,
    #[serde(rename = "uk")]
    Ukrainian,
    #[serde(rename = "ur")]
    Urdu,
    #[serde(rename = "vi")]
    Vietnamese,
    #[serde(rename = "cy")]
    Welsh,
}