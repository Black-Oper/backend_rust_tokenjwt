pub fn converter_binario(string: &str) -> String {
    string.bytes()
        .map(|b| format!("{:08b}", b))
        .collect::<Vec<String>>()
        .join("")
}

pub fn separa_string_binaria(string: &str, num: i32) -> String {
    let mut str_bin_separada = String::new();
    let mut i: i32 = 0;

    for caractere in string.chars() {
        str_bin_separada.push(caractere);
        i += 1;

        if i == num {
            i = 0;
            str_bin_separada.push(' ');
        }
    }

    str_bin_separada
}

pub fn converte_bin_base64(string: &str) -> String {
    let str_b64 = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut contador = 0;
    let mut i = 0;
    let mut string_b64 = String::new();

    for caractere in string.chars() {
        if caractere == ' ' {
            string_b64.push(str_b64.chars().nth(contador).unwrap());
            contador = 0;
            i = 0;
        } else {
            i += 1;
            if caractere == '1' {
                match i {
                    1 => contador += 32,
                    2 => contador += 16,
                    3 => contador += 8,
                    4 => contador += 4,
                    5 => contador += 2,
                    6 => contador += 1,
                    _ => unreachable!("Erro inesperado no match"),
                }
            }
        }
    }

    if i > 0 {
        string_b64.push(str_b64.chars().nth(contador).unwrap());
    }

    let pad = (4 - (string_b64.len() % 4)) % 4;
    for _ in 0..pad {
        string_b64.push('=');
    }

    string_b64
}

pub fn converter_string_base64(input: &str) -> String{

    let s = converter_binario(&input);
    let sbin = separa_string_binaria(&s, 6);
    let sb64 = converte_bin_base64(&sbin);

    sb64
}
