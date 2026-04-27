export interface LanguageOption {
    label: string,
    value: string
}

export interface Languages {
    [key: string]: LanguageOption[]
}

export const languages: Languages = {
    baidu: [
        { label: "Auto", value: "auto" },
        { label: "中文", value: "zh" },
        { label: "English", value: "en" },
        { label: "粵語", value: "yue" },
        { label: "文言文", value: "wyw" },
        { label: "日本語", value: "jp" },
        { label: "한국어", value: "kor" },
        { label: "Français", value: "fra" },
        { label: "Español", value: "spa" },
        { label: "ภาษาไทย", value: "th" },
        { label: "العربية", value: "ara" },
        { label: "Русский", value: "ru" },
        { label: "Português", value: "pt" },
        { label: "Deutsch", value: "de" },
        { label: "Italiano", value: "it" },
        { label: "Ελληνικά", value: "el" },
        { label: "Nederlands", value: "nl" },
        { label: "Polski", value: "pl" },
        { label: "Български", value: "bul" },
        { label: "Eesti", value: "est" },
        { label: "Dansk", value: "dan" },
        { label: "Suomi", value: "fin" },
        { label: "Čeština", value: "cs" },
        { label: "Română", value: "rom" },
        { label: "Slovenščina", value: "slo" },
        { label: "Svenska", value: "swe" },
        { label: "Magyar", value: "hu" },
        { label: "繁體中文", value: "cht" },
        { label: "Tiếng Việt", value: "vie" }
    ]
}