

pub fn parselatin(string: String) -> String {

    let mut text: Vec<char> = Vec::new();
                        
    for c in string.chars()    {
        match c {
            '|' => text.push('Ё'),
            '^' => text.push('Ъ'),
            'Q' => text.push('Я'),
            'W' => text.push('Ш'),
            'E' => text.push('Е'),
            'R' => text.push('Р'),
            'T' => text.push('Т'),
            'Y' => text.push('Ы'),
            'U' => text.push('У'),
            'I' => text.push('И'),
            'O' => text.push('О'),
            'P' => text.push('П'),
            'é' => text.push('Ю'),
            '*' => text.push('Щ'),
            'A' => text.push('А'),
            'S' => text.push('С'),
            'D' => text.push('Д'),
            'F' => text.push('Ф'),
            'G' => text.push('Г'),
            'H' => text.push('Ч'),
            'J' => text.push('Й'),
            'K' => text.push('К'),
            'L' => text.push('Л'),
            'ç' => text.push('Ь'),
            '°' => text.push('Ж'),
            '§' => text.push('Э'),
            'Z' => text.push('З'),
            'X' => text.push('Х'),
            'C' => text.push('Ц'),
            'V' => text.push('В'),
            'B' => text.push('Б'),
            'N' => text.push('Н'),
            'M' => text.push('М'),

            '\\' => text.push('ё'),
            'ì' => text.push('ъ'),
            'q' => text.push('я'),
            'w' => text.push('ш'),
            'e' => text.push('е'),
            'r' => text.push('р'),
            't' => text.push('т'),
            'y' => text.push('ы'),
            'u' => text.push('у'),
            'i' => text.push('и'),
            'o' => text.push('о'),
            'p' => text.push('п'),
            'è' => text.push('ю'),
            '+' => text.push('щ'),
            'a' => text.push('а'),
            's' => text.push('с'),
            'd' => text.push('д'),
            'f' => text.push('ф'),
            'g' => text.push('г'),
            'h' => text.push('ч'),
            'j' => text.push('й'),
            'k' => text.push('к'),
            'l' => text.push('л'),
            'ò' => text.push('ь'),
            'à' => text.push('ж'),
            'ù' => text.push('э'),
            'z' => text.push('з'),
            'x' => text.push('х'),
            'c' => text.push('ц'),
            'v' => text.push('в'),
            'b' => text.push('б'),
            'n' => text.push('н'),
            'm' => text.push('м'),

            _ => text.push(c),
        }
    }

    let s: String = text.into_iter().collect();

    s
}

