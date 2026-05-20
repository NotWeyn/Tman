// Language mapping — single source of truth for source & target language selectors
// Each entry: { code, name, flag, rtl? }

export interface Language {
  code: string;
  name: string;
  flag: string;
  rtl?: boolean;
}

export const languages: Language[] = [
  { code: "auto", name: "Otomatik", flag: "🌐" },
  { code: "af", name: "Afrikaans", flag: "🇿🇦" },
  { code: "ar", name: "العربية", flag: "🇸🇦", rtl: true },
  { code: "az", name: "Azərbaycanca", flag: "🇦🇿" },
  { code: "be", name: "Беларуская", flag: "🇧🇾" },
  { code: "bg", name: "Български", flag: "🇧🇬" },
  { code: "bn", name: "বাংলা", flag: "🇧🇩" },
  { code: "bs", name: "Bosanski", flag: "🇧🇦" },
  { code: "ca", name: "Català", flag: "🇪🇸" },
  { code: "cs", name: "Čeština", flag: "🇨🇿" },
  { code: "cy", name: "Cymraeg", flag: "🏴󠁧󠁢󠁷󠁬󠁳󠁿" },
  { code: "da", name: "Dansk", flag: "🇩🇰" },
  { code: "de", name: "Deutsch", flag: "🇩🇪" },
  { code: "el", name: "Ελληνικά", flag: "🇬🇷" },
  { code: "en", name: "English", flag: "🇬🇧" },
  { code: "es", name: "Español", flag: "🇪🇸" },
  { code: "et", name: "Eesti", flag: "🇪🇪" },
  { code: "fa", name: "فارسی", flag: "🇮🇷", rtl: true },
  { code: "fi", name: "Suomi", flag: "🇫🇮" },
  { code: "fr", name: "Français", flag: "🇫🇷" },
  { code: "ga", name: "Gaeilge", flag: "🇮🇪" },
  { code: "he", name: "עברית", flag: "🇮🇱", rtl: true },
  { code: "hi", name: "हिन्दी", flag: "🇮🇳" },
  { code: "hr", name: "Hrvatski", flag: "🇭🇷" },
  { code: "hu", name: "Magyar", flag: "🇭🇺" },
  { code: "hy", name: "Հայերեն", flag: "🇦🇲" },
  { code: "id", name: "Bahasa Indonesia", flag: "🇮🇩" },
  { code: "is", name: "Íslenska", flag: "🇮🇸" },
  { code: "it", name: "Italiano", flag: "🇮🇹" },
  { code: "ja", name: "日本語", flag: "🇯🇵" },
  { code: "ka", name: "ქართული", flag: "🇬🇪" },
  { code: "kk", name: "Қазақша", flag: "🇰🇿" },
  { code: "km", name: "ខ្មែរ", flag: "🇰🇭" },
  { code: "ko", name: "한국어", flag: "🇰🇷" },
  { code: "la", name: "Latina", flag: "🏛️" },
  { code: "lt", name: "Lietuvių", flag: "🇱🇹" },
  { code: "lv", name: "Latviešu", flag: "🇱🇻" },
  { code: "mk", name: "Македонски", flag: "🇲🇰" },
  { code: "mn", name: "Монгол", flag: "🇲🇳" },
  { code: "ms", name: "Bahasa Melayu", flag: "🇲🇾" },
  { code: "my", name: "ဗမာစာ", flag: "🇲🇲" },
  { code: "nb", name: "Norsk Bokmål", flag: "🇳🇴" },
  { code: "nl", name: "Nederlands", flag: "🇳🇱" },
  { code: "pl", name: "Polski", flag: "🇵🇱" },
  { code: "pt", name: "Português", flag: "🇧🇷" },
  { code: "ro", name: "Română", flag: "🇷🇴" },
  { code: "ru", name: "Русский", flag: "🇷🇺" },
  { code: "sk", name: "Slovenčina", flag: "🇸🇰" },
  { code: "sl", name: "Slovenščina", flag: "🇸🇮" },
  { code: "sq", name: "Shqip", flag: "🇦🇱" },
  { code: "sr", name: "Српски", flag: "🇷🇸" },
  { code: "sv", name: "Svenska", flag: "🇸🇪" },
  { code: "sw", name: "Kiswahili", flag: "🇹🇿" },
  { code: "ta", name: "தமிழ்", flag: "🇮🇳" },
  { code: "te", name: "తెలుగు", flag: "🇮🇳" },
  { code: "th", name: "ไทย", flag: "🇹🇭" },
  { code: "tl", name: "Filipino", flag: "🇵🇭" },
  { code: "tr", name: "Türkçe", flag: "🇹🇷" },
  { code: "uk", name: "Українська", flag: "🇺🇦" },
  { code: "ur", name: "اردو", flag: "🇵🇰", rtl: true },
  { code: "uz", name: "Oʻzbek", flag: "🇺🇿" },
  { code: "vi", name: "Tiếng Việt", flag: "🇻🇳" },
  { code: "zh", name: "中文", flag: "🇨🇳" },
  { code: "zh-TW", name: "繁體中文", flag: "🇹🇼" },
];

/** Get a language by code, returns undefined if not found */
export function getLanguage(code: string): Language | undefined {
  return languages.find(l => l.code === code);
}

/** Get flag emoji for a language code */
export function getFlag(code: string): string {
  return getLanguage(code)?.flag ?? "🏳️";
}

/** All languages except "auto" — for target language picker */
export const targetLanguages = languages.filter(l => l.code !== "auto");
