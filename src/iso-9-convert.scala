package io.github.brkalmar.iso9convert

import java.text
import scala.io

/** Convert stdin Cyrillic to Latin, print to stdout.
  */
object ISO9Convert {

  def main(args: Array[String]) {
    val converter = new Converter
    println(
      args.length match {
        case 0 => convertStdIn(converter)
        case _ => convertArgs(converter, args)
      }
    )
  }

  private def convertStdIn(converter: Converter): String = {
    io.Source.stdin.getLines().map(converter.cyrillicToLatin(_)).mkString("\n")
  }

  private def convertArgs(converter: Converter, args: Array[String]): String = {
    args.map(converter.cyrillicToLatin(_)).mkString("\n")
  }

}

/** Convert between Latin and Cyrillic, as defined by ISO-9.
  * 
  * @param hookToLeft decides what to use as the Unicode replacement of the
  * diacritic “hook to left” used in the standard, when converting to Latin
  * (converting from Latin recognizes all replacements); for instance, with
  * HookToLeft.CommaBelow, "ҳ" becomes "h̦", whereas with HookToLeft.Cedilla, "ҳ"
  * becomes "ḩ"
  * @param hardSoftSign decides what version of the hard and the soft sign to
  * replace “double prime” and “prime” with, when converting to Cyrillic; for
  * instance, with HardSoftSign.Small, "ʺ" becomes "ъ", whereas with
  * HardSoftSign.Capital, "ʺ" becomes "Ъ"
  */
class Converter(
  val hookToLeft: Converter.HookToLeft = Converter.HookToLeft.CommaBelow,
  val hardSoftSign: Converter.HardSoftSign = Converter.HardSoftSign.Small
) {

  import Converter._

  private val lToC: Map[String, String] =
    hardSoftSign match {
      case HardSoftSign.Capital => lToCDefault + (
        "ʹ" -> "Ь",
        "ʺ" -> "Ъ"
      )
      case HardSoftSign.Small => lToCDefault + (
        "ʹ" -> "ь",
        "ʺ" -> "ъ"
      )
    }

  private val lToCLongest: Int = lToC.max._1.length

  private val cToL: Map[String, String] =
    hookToLeft match {
      case HookToLeft.Cedilla => cToLDefault + (
        "Җ" -> "Ž̧", "җ" -> "ž̧",
        "Қ" -> "Ķ", "қ" -> "ķ",
        "Ԡ" -> "Ļ", "ԡ" -> "ļ",
        "Ң" -> "Ņ", "ң" -> "ņ",
        "Ҫ" -> "Ş", "ҫ" -> "ş",
        "Ҭ" -> "Ţ", "ҭ" -> "ţ",
        "Ҳ" -> "Ḩ", "ҳ" -> "ḩ",
        "Ҷ" -> "Ç", "ҷ" -> "ç"
      )
      case HookToLeft.CommaBelow => cToLDefault + (
        "Җ" -> "Ž̦", "җ" -> "ž̦",
        "Қ" -> "K̦", "қ" -> "k̦",
        "Ԡ" -> "L̦", "ԡ" -> "l̦",
        "Ң" -> "N̦", "ң" -> "n̦",
        "Ҫ" -> "Ș", "ҫ" -> "ș",
        "Ҭ" -> "Ț", "ҭ" -> "ț",
        "Ҳ" -> "H̦", "ҳ" -> "h̦",
        "Ҷ" -> "C̦", "ҷ" -> "c̦"
      )
    }

  private val cToLLongest: Int = cToL.max._1.length

  /** Replace all Cyrillic character sequences with Latin equivalents in the
    * string, after NFC-normalization.
    * 
    * @param string the string to convert
    * @return the converted string, NFC-normalized
    */
  def cyrillicToLatin(string: String): String = {
    convert(string, cToL, cToLLongest)
  }

  /** Replace all Latin character sequences with Cyrillic equivalents in the
    * string, after NFC-normalization.
    * 
    * @param string the string to convert
    * @return the converted string, NFC-normalized
    */
  def latinToCyrillic(string: String): String = {
    convert(string, lToC, lToCLongest)
  }

  /** NFC-normalize the string, and replace substrings which are keys in the given
    * map with their associated values.
    * 
    * @param string the string to replace substrings of
    * @param map the map containing strings and their replacements
    * @param longestKey the length of the longest key in the map
    * @return the string with the substrings replaced
    */
  private def convert(
    string: String, map: Map[String, String], longestKey: Int
  ): String = {
    val s = text.Normalizer.normalize(string, text.Normalizer.Form.NFC)

    /** recurisvely replace all substrings */
    def replace(index: Int): String = {

      /** recursively find shortest replacement at the current index, or the single
        * character at the current index if no replacement is found
        */
      def findReplacement(length: Int): String = {
        if (length > longestKey || index + length > s.length) {
          // no replacement in the map, take one char unaltered
          s.substring(index, index + 1)
        } else {
          val subOld = s.substring(index, index + length)
          map get subOld match {
            case Some(subNew) => subNew
            case None => findReplacement(length + 1)
          }
        }
      }

      if (index >= s.length) {
        ""
      } else {
        val replacement = findReplacement(1)
        replacement + replace(index + replacement.length)
      }
    }

    replace(0)
  }

}

object Converter {

  /** Possible Unicode replacements of “hook to left”.
    */
  object HookToLeft extends Enumeration {
    val Cedilla, CommaBelow = Value
  }
  type HookToLeft = HookToLeft.Value

  /** Version of the hard and the soft sign to use.
    */
  object HardSoftSign extends Enumeration {
    val Capital, Small = Value
  }
  type HardSoftSign = HardSoftSign.Value

  /** Default mapping from Latin to Cyrillic, NFC-normalized.
    */
  private val lToCDefault: Map[String, String] = Map(
    "A" -> "А", "a" -> "а",
    "Ä" -> "Ӓ", "ä" -> "ӓ",
    "Ạ̈" -> "Ӓ̄", "ạ̈" -> "ӓ̄",
    "Ă" -> "Ӑ", "ă" -> "ӑ",
    "Ā" -> "А̄", "ā" -> "а̄",
    "Æ" -> "Ӕ", "æ" -> "ӕ",
    "Á" -> "А́", "á" -> "а́",
    "Å" -> "А̊", "å" -> "а̊",
    "B" -> "Б", "b" -> "б",
    "V" -> "В", "v" -> "в",
    "G" -> "Г", "g" -> "г",
    "Ǵ" -> "Ѓ", "ǵ" -> "ѓ",
    "Ġ" -> "Ғ", "ġ" -> "ғ",
    "Ğ" -> "Ҕ", "ğ" -> "ҕ",
    "Ḥ" -> "Һ", "ḥ" -> "һ",
    "D" -> "Д", "d" -> "д",
    "Đ" -> "Ђ", "đ" -> "ђ",
    "E" -> "Е", "e" -> "е",
    "Ĕ" -> "Ӗ", "ĕ" -> "ӗ",
    "Ë" -> "Ё", "ë" -> "ё",
    "Ê" -> "Є", "ê" -> "є",
    "Ž" -> "Ж", "ž" -> "ж",
    "Ž̦" -> "Җ", "ž̦" -> "җ",
    "Ž̧" -> "Җ", "ž̧" -> "җ",
    "Z̄" -> "Ӝ", "z̄" -> "ӝ",
    "Z̆" -> "Ӂ", "z̆" -> "ӂ",
    "Z" -> "З", "z" -> "з",
    "Z̈" -> "Ӟ", "z̈" -> "ӟ",
    "Ź" -> "Ӡ", "ź" -> "ӡ",
    "Ẑ" -> "Ѕ", "ẑ" -> "ѕ",
    "I" -> "И", "i" -> "и",
    "Ī" -> "Ӣ", "ī" -> "ӣ",
    "Í" -> "И́", "í" -> "и́",
    "Î" -> "Ӥ", "î" -> "ӥ",
    "J" -> "Й", "j" -> "й",
    "Ì" -> "І", "ì" -> "і",
    "Ï" -> "Ї", "ï" -> "ї",
    "Ǐ" -> "І̄", "ǐ" -> "і̄",
    "J̌" -> "Ј", "ǰ" -> "ј",
    "J́" -> "Ј̵", "j́" -> "ј̵",
    "K" -> "К", "k" -> "к",
    "Ḱ" -> "Ќ", "ḱ" -> "ќ",
    "Ḳ" -> "Ӄ", "ḳ" -> "ӄ",
    "K̂" -> "Ҝ", "k̂" -> "ҝ",
    "Ǩ" -> "Ҡ", "ǩ" -> "ҡ",
    "K̄" -> "Ҟ", "k̄" -> "ҟ",
    "K̦" -> "Қ", "k̦" -> "қ",
    "Ķ" -> "Қ", "ķ" -> "қ",
    "K̀" -> "К̨", "k̀" -> "к̨",
    "Q" -> "Ԛ", "q" -> "ԛ",
    "L" -> "Л", "l" -> "л",
    "L̂" -> "Љ", "l̂" -> "љ",
    "Ĺ" -> "Л’", "ĺ" -> "Л’",
    "L̦" -> "Ԡ", "l̦" -> "ԡ",
    "Ļ" -> "Ԡ", "ļ" -> "ԡ",
    "M" -> "М", "m" -> "м",
    "N" -> "Н", "n" -> "н",
    "N̂" -> "Њ", "n̂" -> "њ",
    "N̦" -> "Ң", "n̦" -> "ң",
    "Ņ" -> "Ң", "ņ" -> "ң",
    "Ṇ" -> "Ӊ", "ṇ" -> "ӊ",
    "Ṅ" -> "Ҥ", "ṅ" -> "ҥ",
    "Ǹ" -> "Ԋ", "ǹ" -> "ԋ",
    "Ń" -> "Ԣ", "ń" -> "ԣ",
    "Ň" -> "Ӈ", "ň" -> "ӈ",
    "N̄" -> "Н̄", "n̄" -> "н̄",
    "O" -> "О", "o" -> "о",
    "Ö" -> "Ӧ", "ö" -> "ӧ",
    "Ô" -> "Ө", "ô" -> "ө",
    "Ő" -> "Ӫ", "ő" -> "ӫ",
    "Ọ̈" -> "Ӧ̄", "ọ̈" -> "о̄̈",
    "Ò" -> "Ҩ", "ò" -> "ҩ",
    "Ó" -> "О́", "ó" -> "о́",
    "Ō" -> "О̄", "ō" -> "о̄",
    "Œ" -> "Œ", "œ" -> "œ",
    "P" -> "П", "p" -> "п",
    "Ṕ" -> "Ҧ", "ṕ" -> "ҧ",
    "P̀" -> "Ԥ", "p̀" -> "ԥ",
    "R" -> "Р", "r" -> "р",
    "S" -> "С", "s" -> "с",
    "Ș" -> "Ҫ", "ș" -> "ҫ",
    "Ş" -> "Ҫ", "ş" -> "ҫ",
    "S̀" -> "С̀", "s̀" -> "с̀",
    "T" -> "Т", "t" -> "т",
    "Ć" -> "Ћ", "ć" -> "ћ",
    "T̀" -> "Ԏ", "t̀" -> "ԏ",
    "Ť" -> "Т̌", "ť" -> "т̌",
    "Ț" -> "Ҭ", "ț" -> "ҭ",
    "Ţ" -> "Ҭ", "ţ" -> "ҭ",
    "U" -> "У", "u" -> "у",
    "Ü" -> "Ӱ", "ü" -> "ӱ",
    "Ū" -> "Ӯ", "ū" -> "ӯ",
    "Ŭ" -> "Ў", "ŭ" -> "ў",
    "Ű" -> "Ӳ", "ű" -> "ӳ",
    "Ú" -> "У́", "ú" -> "у́",
    "Ụ̈" -> "Ӱ̄", "ụ̈" -> "ӱ̄",
    "Ù" -> "Ү", "ù" -> "ү",
    "U̇" -> "Ұ", "u̇" -> "ұ",
    "Ụ̄" -> "Ӱ̄", "ụ̄" -> "ӱ̄",
    "W" -> "Ԝ", "w" -> "ԝ",
    "F" -> "Ф", "f" -> "ф",
    "H" -> "Х", "h" -> "х",
    "H̦" -> "Ҳ", "h̦" -> "ҳ",
    "Ḩ" -> "Ҳ", "ḩ" -> "ҳ",
    "C" -> "Ц", "c" -> "ц",
    "C̄" -> "Ҵ", "c̄" -> "ҵ",
    "D̂" -> "Џ", "d̂" -> "џ",
    "Č" -> "Ч", "č" -> "ч",
    "C̦" -> "Ҷ", "c̦" -> "ҷ",
    "Ç" -> "Ҷ", "ç" -> "ҷ",
    "C̣" -> "Ӌ", "c̣" -> "ӌ",
    "C̈" -> "Ӵ", "c̈" -> "ӵ",
    "Ĉ" -> "Ҹ", "ĉ" -> "ҹ",
    "C̀" -> "Ч̀", "c̀" -> "ч̀",
    "C̆" -> "Ҽ", "c̆" -> "ҽ",
    "C̨̆" -> "Ҿ", "c̨̆" -> "ҿ",
    "Š" -> "Ш", "š" -> "ш",
    "Ŝ" -> "Щ", "ŝ" -> "щ",
    "ʺ" -> "ъ",
    "Y" -> "Ы", "y" -> "ы",
    "Ÿ" -> "Ӹ", "ÿ" -> "ӹ",
    "Ȳ" -> "Ы̄", "ȳ" -> "ы̄",
    "ʹ" -> "ь",
    "È" -> "Э", "è" -> "э",
    "A̋" -> "Ә", "a̋" -> "ә",
    "À" -> "Ӛ", "à" -> "ӛ",
    "Û" -> "Ю", "û" -> "ю",
    "Û̄" -> "Ю̄", "û̄" -> "ю̄",
    "Â" -> "Я", "â" -> "я",
    "G̀" -> "Ґ", "g̀" -> "ґ",
    "Ě" -> "Ѣ", "ě" -> "ѣ",
    "Ǎ" -> "Ѫ", "ǎ" -> "ѫ",
    "F̀" -> "Ѳ", "f̀" -> "ѳ",
    "Ỳ" -> "Ѵ", "ỳ" -> "ѵ",
    "‡" -> "Ӏ",
    "`" -> "ʼ",
    "¨" -> "ˮ"
  )

  /** Default mapping from Cyrillic to Latin, NFC-normalized.
    */
  private val cToLDefault: Map[String, String] =
    lToCDefault.map(_.swap) + (
      "Ъ" -> "ʺ",
      "Ь" -> "ʹ"
    )

}
