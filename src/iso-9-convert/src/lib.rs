#[macro_use]
extern crate clap;
extern crate unicode_normalization;

use std::cmp;
use std::collections::HashMap;
use std::iter::FromIterator;
use unicode_normalization::UnicodeNormalization;

pub struct Converter {
    replace_map: HashMap<String, String>,
    key_len_max: usize,
}

arg_enum!{
    pub enum HardSoftSign {
        Capital,
        Small
    }
}

impl Default for HardSoftSign {
    fn default() -> Self {
        HardSoftSign::Small
    }
}

arg_enum!{
    pub enum HookToLeft {
        Cedilla,
        CommaBelow
    }
}

impl Default for HookToLeft {
    fn default() -> Self {
        HookToLeft::Cedilla
    }
}

impl Converter {
    fn with_replace_map(map: HashMap<&str, &str>) -> Self {
        let replace_map: HashMap<String, String> = map.into_iter()
            .map(|(k, v)| (k.nfc().collect(), v.nfc().collect()))
            .collect();
        let key_len_max = replace_map.keys()
            .map(|k| k.chars().count())
            .max().expect("replace map is empty");
        Self {
            key_len_max,
            replace_map,
        }
    }

    pub fn to_cyrillic(hard_soft_sign: HardSoftSign) -> Self {
        let mut map: HashMap<&str, &str> = TO_CYRILLIC.into_iter().map(|&t| t)
            .collect();
        match hard_soft_sign {
            HardSoftSign::Capital => map.extend([
                ("ʹ", "Ь"),
                ("ʺ", "Ъ"),
            ].into_iter().map(|&t| t)),
            HardSoftSign::Small => map.extend([
                ("ʹ", "ь"),
                ("ʺ", "ъ"),
            ].into_iter().map(|&t| t)),
        };
        Self::with_replace_map(map)
    }

    pub fn to_latin(hook_to_left: HookToLeft) -> Self {
        let mut map: HashMap<&str, &str> = TO_CYRILLIC.iter()
            .map(|&(k, v)| (v, k)).collect();
        map.extend([
            ("Ъ", "ʺ"),
            ("Ь", "ʹ"),
        ].into_iter().map(|&t| t));
        match hook_to_left {
            HookToLeft::Cedilla => map.extend([
                ("Җ", "Ž̧"), ("җ", "ž̧"),
                ("Қ", "Ķ"), ("қ", "ķ"),
                ("Ԡ", "Ļ"), ("ԡ", "ļ"),
                ("Ң", "Ņ"), ("ң", "ņ"),
                ("Ҫ", "Ş"), ("ҫ", "ş"),
                ("Ҭ", "Ţ"), ("ҭ", "ţ"),
                ("Ҳ", "Ḩ"), ("ҳ", "ḩ"),
                ("Ҷ", "Ç"), ("ҷ", "ç"),
            ].into_iter().map(|&t| t)),
            HookToLeft::CommaBelow => map.extend([
                ("Җ", "Ž̦"), ("җ", "ž̦"),
                ("Қ", "K̦"), ("қ", "k̦"),
                ("Ԡ", "L̦"), ("ԡ", "l̦"),
                ("Ң", "N̦"), ("ң", "n̦"),
                ("Ҫ", "Ș"), ("ҫ", "ș"),
                ("Ҭ", "Ț"), ("ҭ", "ț"),
                ("Ҳ", "H̦"), ("ҳ", "h̦"),
                ("Ҷ", "C̦"), ("ҷ", "c̦"),
            ].into_iter().map(|&t| t)),
        };
        Self::with_replace_map(map)
    }

    pub fn convert(&self, string: &str) -> String {
        let mut converted = String::new();
        let mut chars = string.nfc();
        loop {
            let len_max = cmp::min(chars.clone().count(), self.key_len_max);
            if len_max == 0 {
                break;
            }
            let mut replace = String::from_iter(chars.clone().take(len_max));
            let mut replaced = 0;
            // iterating from longest to shortest, so that if a has b as its
            // suffix but is longer than b, a takes precedence
            while !replace.is_empty() {
                if let Some(with) = self.replace_map.get(replace.as_str()) {
                    converted.push_str(with);
                    replaced = replace.chars().count();
                    break;
                }
                replace.pop();
            }
            if replaced == 0 {
                converted.push(chars.next()
                               // this should never happen
                               .expect("chars is empty"));
                continue;
            }
            for _ in 0..replaced {
                chars.next()
                    // this should never happen
                    .expect("chars is empty");
            }
        }
        converted
    }
}

const TO_CYRILLIC: [(&str, &str); 269] = [
    ("A", "А"), ("a", "а"),
    ("Ä", "Ӓ"), ("ä", "ӓ"),
    ("Ạ̈", "Ӓ̄"), ("ạ̈", "ӓ̄"),
    ("Ă", "Ӑ"), ("ă", "ӑ"),
    ("Ā", "А̄"), ("ā", "а̄"),
    ("Æ", "Ӕ"), ("æ", "ӕ"),
    ("Á", "А́"), ("á", "а́"),
    ("Å", "А̊"), ("å", "а̊"),
    ("B", "Б"), ("b", "б"),
    ("V", "В"), ("v", "в"),
    ("G", "Г"), ("g", "г"),
    ("Ǵ", "Ѓ"), ("ǵ", "ѓ"),
    ("Ġ", "Ғ"), ("ġ", "ғ"),
    ("Ğ", "Ҕ"), ("ğ", "ҕ"),
    ("Ḥ", "Һ"), ("ḥ", "һ"),
    ("D", "Д"), ("d", "д"),
    ("Đ", "Ђ"), ("đ", "ђ"),
    ("E", "Е"), ("e", "е"),
    ("Ĕ", "Ӗ"), ("ĕ", "ӗ"),
    ("Ë", "Ё"), ("ë", "ё"),
    ("Ê", "Є"), ("ê", "є"),
    ("Ž", "Ж"), ("ž", "ж"),
    ("Ž̦", "Җ"), ("ž̦", "җ"),
    ("Ž̧", "Җ"), ("ž̧", "җ"),
    ("Z̄", "Ӝ"), ("z̄", "ӝ"),
    ("Z̆", "Ӂ"), ("z̆", "ӂ"),
    ("Z", "З"), ("z", "з"),
    ("Z̈", "Ӟ"), ("z̈", "ӟ"),
    ("Ź", "Ӡ"), ("ź", "ӡ"),
    // NOTE: not part of ISO-9
    ("Ð", "Ҙ"), ("ð", "ҙ"),
    ("Ẑ", "Ѕ"), ("ẑ", "ѕ"),
    ("I", "И"), ("i", "и"),
    ("Ī", "Ӣ"), ("ī", "ӣ"),
    ("Í", "И́"), ("í", "и́"),
    ("Î", "Ӥ"), ("î", "ӥ"),
    ("J", "Й"), ("j", "й"),
    ("Ì", "І"), ("ì", "і"),
    ("Ï", "Ї"), ("ï", "ї"),
    ("Ǐ", "І̄"), ("ǐ", "і̄"),
    ("J̌", "Ј"), ("ǰ", "ј"),
    ("J́", "Ј̵"), ("j́", "ј̵"),
    ("K", "К"), ("k", "к"),
    ("Ḱ", "Ќ"), ("ḱ", "ќ"),
    ("Ḳ", "Ӄ"), ("ḳ", "ӄ"),
    ("K̂", "Ҝ"), ("k̂", "ҝ"),
    ("Ǩ", "Ҡ"), ("ǩ", "ҡ"),
    ("K̄", "Ҟ"), ("k̄", "ҟ"),
    ("K̦", "Қ"), ("k̦", "қ"),
    ("Ķ", "Қ"), ("ķ", "қ"),
    ("K̀", "К̨"), ("k̀", "к̨"),
    ("Q", "Ԛ"), ("q", "ԛ"),
    ("L", "Л"), ("l", "л"),
    ("L̂", "Љ"), ("l̂", "љ"),
    ("Ĺ", "Л’"), ("ĺ", "л’"),
    ("L̦", "Ԡ"), ("l̦", "ԡ"),
    ("Ļ", "Ԡ"), ("ļ", "ԡ"),
    ("M", "М"), ("m", "м"),
    ("N", "Н"), ("n", "н"),
    ("N̂", "Њ"), ("n̂", "њ"),
    ("N̦", "Ң"), ("n̦", "ң"),
    ("Ņ", "Ң"), ("ņ", "ң"),
    ("Ṇ", "Ӊ"), ("ṇ", "ӊ"),
    ("Ṅ", "Ҥ"), ("ṅ", "ҥ"),
    ("Ǹ", "Ԋ"), ("ǹ", "ԋ"),
    ("Ń", "Ԣ"), ("ń", "ԣ"),
    ("Ň", "Ӈ"), ("ň", "ӈ"),
    ("N̄", "Н̄"), ("n̄", "н̄"),
    ("O", "О"), ("o", "о"),
    ("Ö", "Ӧ"), ("ö", "ӧ"),
    ("Ô", "Ө"), ("ô", "ө"),
    ("Ő", "Ӫ"), ("ő", "ӫ"),
    ("Ọ̈", "Ӧ̄"), ("ọ̈", "о̄̈"),
    ("Ò", "Ҩ"), ("ò", "ҩ"),
    ("Ó", "О́"), ("ó", "о́"),
    ("Ō", "О̄"), ("ō", "о̄"),
    ("Œ", "Œ"), ("œ", "œ"),
    ("P", "П"), ("p", "п"),
    ("Ṕ", "Ҧ"), ("ṕ", "ҧ"),
    ("P̀", "Ԥ"), ("p̀", "ԥ"),
    ("R", "Р"), ("r", "р"),
    ("S", "С"), ("s", "с"),
    ("Ș", "Ҫ"), ("ș", "ҫ"),
    ("Ş", "Ҫ"), ("ş", "ҫ"),
    ("S̀", "С̀"), ("s̀", "с̀"),
    ("T", "Т"), ("t", "т"),
    ("Ć", "Ћ"), ("ć", "ћ"),
    ("T̀", "Ԏ"), ("t̀", "ԏ"),
    ("Ť", "Т̌"), ("ť", "т̌"),
    ("Ț", "Ҭ"), ("ț", "ҭ"),
    ("Ţ", "Ҭ"), ("ţ", "ҭ"),
    ("U", "У"), ("u", "у"),
    ("Ü", "Ӱ"), ("ü", "ӱ"),
    ("Ū", "Ӯ"), ("ū", "ӯ"),
    ("Ŭ", "Ў"), ("ŭ", "ў"),
    ("Ű", "Ӳ"), ("ű", "ӳ"),
    ("Ú", "У́"), ("ú", "у́"),
    ("Ụ̈", "Ӱ̄"), ("ụ̈", "ӱ̄"),
    ("Ù", "Ү"), ("ù", "ү"),
    ("U̇", "Ұ"), ("u̇", "ұ"),
    ("W", "Ԝ"), ("w", "ԝ"),
    ("F", "Ф"), ("f", "ф"),
    ("H", "Х"), ("h", "х"),
    ("H̦", "Ҳ"), ("h̦", "ҳ"),
    ("Ḩ", "Ҳ"), ("ḩ", "ҳ"),
    ("C", "Ц"), ("c", "ц"),
    ("C̄", "Ҵ"), ("c̄", "ҵ"),
    ("D̂", "Џ"), ("d̂", "џ"),
    ("Č", "Ч"), ("č", "ч"),
    ("C̦", "Ҷ"), ("c̦", "ҷ"),
    ("Ç", "Ҷ"), ("ç", "ҷ"),
    ("C̣", "Ӌ"), ("c̣", "ӌ"),
    ("C̈", "Ӵ"), ("c̈", "ӵ"),
    ("Ĉ", "Ҹ"), ("ĉ", "ҹ"),
    ("C̀", "Ч̀"), ("c̀", "ч̀"),
    ("C̆", "Ҽ"), ("c̆", "ҽ"),
    ("C̨̆", "Ҿ"), ("c̨̆", "ҿ"),
    ("Š", "Ш"), ("š", "ш"),
    ("Ŝ", "Щ"), ("ŝ", "щ"),
    ("ʺ", "ъ"),
    ("Y", "Ы"), ("y", "ы"),
    ("Ÿ", "Ӹ"), ("ÿ", "ӹ"),
    ("Ȳ", "Ы̄"), ("ȳ", "ы̄"),
    ("ʹ", "ь"),
    ("È", "Э"), ("è", "э"),
    ("A̋", "Ә"), ("a̋", "ә"),
    ("À", "Ӛ"), ("à", "ӛ"),
    ("Û", "Ю"), ("û", "ю"),
    ("Û̄", "Ю̄"), ("û̄", "ю̄"),
    ("Â", "Я"), ("â", "я"),
    ("G̀", "Ґ"), ("g̀", "ґ"),
    ("Ě", "Ѣ"), ("ě", "ѣ"),
    ("Ǎ", "Ѫ"), ("ǎ", "ѫ"),
    ("F̀", "Ѳ"), ("f̀", "ѳ"),
    ("Ỳ", "Ѵ"), ("ỳ", "ѵ"),
    ("‡", "Ӏ"),
    ("`", "ʼ"),
    ("¨", "ˮ"),
];

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper that automatically formats the strings as chars and bytes upon
    /// failure
    fn test_convert(converter: &Converter, input: &str, expected: &str) {
        let converted = converter.convert(input);
        assert_eq!(converted, expected, "\n{}\n{}",
                   str_format_chars_and_bytes(&converted),
                   str_format_chars_and_bytes(expected));

        fn str_format_chars_and_bytes(s: &str) -> String {
            let mut chars = String::new();
            let mut bytes = String::new();
            let char_indices: HashMap<usize, char> = s.char_indices().collect();
            for (i, byte) in s.as_bytes().into_iter().enumerate() {
                chars.push_str(
                    &format!("{:>3}", match char_indices.get(&i) {
                        Some(&c) => c,
                        None => ' ',
                    })
                );
                bytes.push_str(&format!(" {:02x}", byte));
            }
            format!("{}\n{}", chars, bytes)
        }
    }

    #[test]
    fn converter_to_cyrillic_capital() {
        let c = Converter::to_cyrillic(HardSoftSign::Capital);
        test_convert(&c, "", "");
        test_convert(&c, "đabŭỤ̈deFḰt123c̀iю", "ђабўӰ̄деФЌт123ч̀ию");
        test_convert(&c, "abĹEi ĺ", "абЛ’Еи л’");
        // abk
        test_convert(&c, "Ac̄h alṕha uouaait.", "Аҵх алҧха уоуааит.");
        // bak
        test_convert(&c, "Ḥeð bašǩortsa ḥôjla̋ša̋ḥegeðme?",
                     "Һеҙ башҡортса һөйләшәһегеҙме?");
        // bel
        test_convert(&c, "Maë sudna na pavetranaj padušcy poŭna vugramì.",
                     "Маё судна на паветранай падушцы поўна вуграмі.");
        // bul
        test_convert(&c,
                     "Korabʺt mi na vʺzdušna vʺzglavnica e pʺlen sʺs zmiorki.",
                     "КорабЪт ми на вЪздушна вЪзглавница е пЪлен сЪс змиорки.");
        // chv
        test_convert(&c, "Tarhasšăn, majĕpenreh kalaşăr.",
                     "Тархасшӑн, майӗпенрех калаҫӑр.");
        test_convert(&c, "Tarhasšăn, majĕpenreh kalașăr.",
                     "Тархасшӑн, майӗпенрех калаҫӑр.");
        // kaz
        test_convert(&c, "Menìņ a̋ue negìz kemesì žylanbalyķpen toltyrylġan.",
                     "Менің әуе негіз кемесі жыланбалықпен толтырылған.");
        test_convert(&c, "Menìn̦ a̋ue negìz kemesì žylanbalyk̦pen toltyrylġan.",
                     "Менің әуе негіз кемесі жыланбалықпен толтырылған.");
        // kir
        test_convert(&c, "Menin aba kemem kurt balyktar menen tolup turat.",
                     "Менин аба кемем курт балыктар менен толуп турат.");
        // mdf
        test_convert(&c, "Kožftodu venežeze gujgalda pâškse.",
                     "Кожфтоду венежезе гуйгалда пяшксе.");
        // mkd
        test_convert(&c, "Moeto letačko vozilo e polno so ǰaguli.",
                     "Моето летачко возило е полно со јагули.");
        // mon
        test_convert(&c, "Minij hôvôgč ongoc mogoj zagasaar dùùrsèn bajna.",
                     "Миний хөвөгч онгоц могой загасаар дүүрсэн байна.");
        // myv
        test_convert(&c, "Monʹ venčesʹ koštonʹ todovso sval pešksè ëzny kalso.",
                     "МонЬ венчесЬ коштонЬ тодовсо свал пешксэ ёзны калсо.");
        // oss
        test_convert(&c, "Æhsyzgon myn u demæ bazongæ uyn.",
                     "Ӕхсызгон мын у демӕ базонгӕ уын.");
        // rus
        test_convert(&c, "Moë sudno na vozdušnoj poduške polno ugrej.",
                     "Моё судно на воздушной подушке полно угрей.");
        // Rusyn
        test_convert(&c, "Êden âzyk neê nig̀da dostatnʹo.",
                     "Єден язык неє ниґда достатнЬо.");
        // sah
        test_convert(&c, "Miigin kytta ùṅkùùlùôhhùtùn bağarağyt duo?",
                     "Миигин кытта үҥкүүлүөххүтүн баҕараҕыт дуо?");
        // srp
        test_convert(&c, "Moǰ hoverkraft ǰe pun ǰegul̂a.",
                     "Мој ховеркрафт је пун јегуља.");
        // tgk
        test_convert(&c, "Az vohuriamon šod ḩastam.",
                     "Аз вохуриамон шод ҳастам.");
        test_convert(&c, "Az vohuriamon šod h̦astam.",
                     "Аз вохуриамон шод ҳастам.");
        // ukr
        test_convert(&c, "Moê sudno na povìtrânìj podušcì napovnene vugrami.",
                     "Моє судно на повітряній подушці наповнене вуграми.");
    }

    #[test]
    fn converter_to_cyrillic_small() {
        let c = Converter::to_cyrillic(HardSoftSign::Small);
        test_convert(&c, "", "");
        test_convert(&c, "đabŭỤ̈deFḰt123c̀iю", "ђабўӰ̄деФЌт123ч̀ию");
        test_convert(&c, "abĹEi ĺ", "абЛ’Еи л’");
        // abk
        test_convert(&c, "Ac̄h alṕha uouaait.", "Аҵх алҧха уоуааит.");
        // bak
        test_convert(&c, "Ḥeð bašǩortsa ḥôjla̋ša̋ḥegeðme?",
                     "Һеҙ башҡортса һөйләшәһегеҙме?");
        // bel
        test_convert(&c, "Maë sudna na pavetranaj padušcy poŭna vugramì.",
                     "Маё судна на паветранай падушцы поўна вуграмі.");
        // bul
        test_convert(&c,
                     "Korabʺt mi na vʺzdušna vʺzglavnica e pʺlen sʺs zmiorki.",
                     "Корабът ми на въздушна възглавница е пълен със змиорки.");
        // chv
        test_convert(&c, "Tarhasšăn, majĕpenreh kalaşăr.",
                     "Тархасшӑн, майӗпенрех калаҫӑр.");
        test_convert(&c, "Tarhasšăn, majĕpenreh kalașăr.",
                     "Тархасшӑн, майӗпенрех калаҫӑр.");
        // kaz
        test_convert(&c, "Menìņ a̋ue negìz kemesì žylanbalyķpen toltyrylġan.",
                     "Менің әуе негіз кемесі жыланбалықпен толтырылған.");
        test_convert(&c, "Menìn̦ a̋ue negìz kemesì žylanbalyk̦pen toltyrylġan.",
                     "Менің әуе негіз кемесі жыланбалықпен толтырылған.");
        // kir
        test_convert(&c, "Menin aba kemem kurt balyktar menen tolup turat.",
                     "Менин аба кемем курт балыктар менен толуп турат.");
        // mdf
        test_convert(&c, "Kožftodu venežeze gujgalda pâškse.",
                     "Кожфтоду венежезе гуйгалда пяшксе.");
        // mkd
        test_convert(&c, "Moeto letačko vozilo e polno so ǰaguli.",
                     "Моето летачко возило е полно со јагули.");
        // mon
        test_convert(&c, "Minij hôvôgč ongoc mogoj zagasaar dùùrsèn bajna.",
                     "Миний хөвөгч онгоц могой загасаар дүүрсэн байна.");
        // myv
        test_convert(&c, "Monʹ venčesʹ koštonʹ todovso sval pešksè ëzny kalso.",
                     "Монь венчесь коштонь тодовсо свал пешксэ ёзны калсо.");
        // oss
        test_convert(&c, "Æhsyzgon myn u demæ bazongæ uyn.",
                     "Ӕхсызгон мын у демӕ базонгӕ уын.");
        // rus
        test_convert(&c, "Moë sudno na vozdušnoj poduške polno ugrej.",
                     "Моё судно на воздушной подушке полно угрей.");
        // Rusyn
        test_convert(&c, "Êden âzyk neê nig̀da dostatnʹo.",
                     "Єден язык неє ниґда достатньо.");
        // sah
        test_convert(&c, "Miigin kytta ùṅkùùlùôhhùtùn bağarağyt duo?",
                     "Миигин кытта үҥкүүлүөххүтүн баҕараҕыт дуо?");
        // srp
        test_convert(&c, "Moǰ hoverkraft ǰe pun ǰegul̂a.",
                     "Мој ховеркрафт је пун јегуља.");
        // tgk
        test_convert(&c, "Az vohuriamon šod ḩastam.",
                     "Аз вохуриамон шод ҳастам.");
        test_convert(&c, "Az vohuriamon šod h̦astam.",
                     "Аз вохуриамон шод ҳастам.");
        // ukr
        test_convert(&c, "Moê sudno na povìtrânìj podušcì napovnene vugrami.",
                     "Моє судно на повітряній подушці наповнене вуграми.");
    }

    #[test]
    fn converter_to_latin_cedilla() {
        let c = Converter::to_latin(HookToLeft::Cedilla);
        test_convert(&c, "", "");
        test_convert(&c, "abcђdӰ̄eFЌт123юhij", "abcđdỤ̈eFḰt123ûhij");
        test_convert(&c, "абЛ’Еи л’", "abĹEi ĺ");
        // abk
        test_convert(&c, "Аҵх алҧха уоуааит.", "Ac̄h alṕha uouaait.");
        // bak
        test_convert(&c, "Һеҙ башҡортса һөйләшәһегеҙме?",
                     "Ḥeð bašǩortsa ḥôjla̋ša̋ḥegeðme?");
        // bel
        test_convert(&c, "Маё судна на паветранай падушцы поўна вуграмі.",
                     "Maë sudna na pavetranaj padušcy poŭna vugramì.");
        // bul
        test_convert(&c,
                     "Корабът ми на въздушна възглавница е пълен със змиорки.",
                     "Korabʺt mi na vʺzdušna vʺzglavnica e pʺlen sʺs zmiorki.");
        // chv
        test_convert(&c, "Тархасшӑн, майӗпенрех калаҫӑр.",
                     "Tarhasšăn, majĕpenreh kalaşăr.");
        // kaz
        test_convert(&c, "Менің әуе негіз кемесі жыланбалықпен толтырылған.",
                     "Menìņ a̋ue negìz kemesì žylanbalyķpen toltyrylġan.");
        // kir
        test_convert(&c, "Менин аба кемем курт балыктар менен толуп турат.",
                     "Menin aba kemem kurt balyktar menen tolup turat.");
        // mdf
        test_convert(&c, "Кожфтоду венежезе гуйгалда пяшксе.",
                     "Kožftodu venežeze gujgalda pâškse.");
        // mkd
        test_convert(&c, "Моето летачко возило е полно со јагули.",
                     "Moeto letačko vozilo e polno so ǰaguli.");
        // mon
        test_convert(&c, "Миний хөвөгч онгоц могой загасаар дүүрсэн байна.",
                     "Minij hôvôgč ongoc mogoj zagasaar dùùrsèn bajna.");
        // myv
        test_convert(&c, "Монь венчесь коштонь тодовсо свал пешксэ ёзны калсо.",
                     "Monʹ venčesʹ koštonʹ todovso sval pešksè ëzny kalso.");
        // oss
        test_convert(&c, "Ӕхсызгон мын у демӕ базонгӕ уын.",
                     "Æhsyzgon myn u demæ bazongæ uyn.");
        // rus
        test_convert(&c, "Моё судно на воздушной подушке полно угрей.",
                     "Moë sudno na vozdušnoj poduške polno ugrej.");
        // Rusyn
        test_convert(&c, "Єден язык неє ниґда достатньо.",
                     "Êden âzyk neê nig̀da dostatnʹo.");
        // sah
        test_convert(&c, "Миигин кытта үҥкүүлүөххүтүн баҕараҕыт дуо?",
                     "Miigin kytta ùṅkùùlùôhhùtùn bağarağyt duo?");
        // srp
        test_convert(&c, "Мој ховеркрафт је пун јегуља.",
                     "Moǰ hoverkraft ǰe pun ǰegul̂a.");
        // tgk
        test_convert(&c, "Аз вохуриамон шод ҳастам.",
                     "Az vohuriamon šod ḩastam.");
        // ukr
        test_convert(&c, "Моє судно на повітряній подушці наповнене вуграми.",
                     "Moê sudno na povìtrânìj podušcì napovnene vugrami.");
    }

    #[test]
    fn converter_to_latin_comma_below() {
        let c = Converter::to_latin(HookToLeft::CommaBelow);
        test_convert(&c, "", "");
        test_convert(&c, "abcђdӰ̄eFЌт123юhij", "abcđdỤ̈eFḰt123ûhij");
        test_convert(&c, "абЛ’Еи л’", "abĹEi ĺ");
        // abk
        test_convert(&c, "Аҵх алҧха уоуааит.", "Ac̄h alṕha uouaait.");
        // bak
        test_convert(&c, "Һеҙ башҡортса һөйләшәһегеҙме?",
                     "Ḥeð bašǩortsa ḥôjla̋ša̋ḥegeðme?");
        // bel
        test_convert(&c, "Маё судна на паветранай падушцы поўна вуграмі.",
                     "Maë sudna na pavetranaj padušcy poŭna vugramì.");
        // bul
        test_convert(&c,
                     "Корабът ми на въздушна възглавница е пълен със змиорки.",
                     "Korabʺt mi na vʺzdušna vʺzglavnica e pʺlen sʺs zmiorki.");
        // chv
        test_convert(&c, "Тархасшӑн, майӗпенрех калаҫӑр.",
                     "Tarhasšăn, majĕpenreh kalașăr.");
        // kaz
        test_convert(&c, "Менің әуе негіз кемесі жыланбалықпен толтырылған.",
                     "Menìn̦ a̋ue negìz kemesì žylanbalyk̦pen toltyrylġan.");
        // kir
        test_convert(&c, "Менин аба кемем курт балыктар менен толуп турат.",
                     "Menin aba kemem kurt balyktar menen tolup turat.");
        // mdf
        test_convert(&c, "Кожфтоду венежезе гуйгалда пяшксе.",
                     "Kožftodu venežeze gujgalda pâškse.");
        // mkd
        test_convert(&c, "Моето летачко возило е полно со јагули.",
                     "Moeto letačko vozilo e polno so ǰaguli.");
        // mon
        test_convert(&c, "Миний хөвөгч онгоц могой загасаар дүүрсэн байна.",
                     "Minij hôvôgč ongoc mogoj zagasaar dùùrsèn bajna.");
        // myv
        test_convert(&c, "Монь венчесь коштонь тодовсо свал пешксэ ёзны калсо.",
                     "Monʹ venčesʹ koštonʹ todovso sval pešksè ëzny kalso.");
        // oss
        test_convert(&c, "Ӕхсызгон мын у демӕ базонгӕ уын.",
                     "Æhsyzgon myn u demæ bazongæ uyn.");
        // rus
        test_convert(&c, "Моё судно на воздушной подушке полно угрей.",
                     "Moë sudno na vozdušnoj poduške polno ugrej.");
        // Rusyn
        test_convert(&c, "Єден язык неє ниґда достатньо.",
                     "Êden âzyk neê nig̀da dostatnʹo.");
        // sah
        test_convert(&c, "Миигин кытта үҥкүүлүөххүтүн баҕараҕыт дуо?",
                     "Miigin kytta ùṅkùùlùôhhùtùn bağarağyt duo?");
        // srp
        test_convert(&c, "Мој ховеркрафт је пун јегуља.",
                     "Moǰ hoverkraft ǰe pun ǰegul̂a.");
        // tgk
        test_convert(&c, "Аз вохуриамон шод ҳастам.",
                     "Az vohuriamon šod h̦astam.");
        // ukr
        test_convert(&c, "Моє судно на повітряній подушці наповнене вуграми.",
                     "Moê sudno na povìtrânìj podušcì napovnene vugrami.");
    }
}
