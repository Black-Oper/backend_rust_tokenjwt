use crate::utils::codificar::separa_string_binaria;

fn converte_base64_bin(input: &str) -> String {
    let str_b64 = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut str_bin = String::new();

    for caracter in input.chars() {
        if caracter == '=' {
            continue;
        }
        let mut i = 0;
        for c in str_b64.chars() {
            i += 1;
            if caracter == c {
                str_bin.push_str(&format!("{:06b}", i - 1));
                break;
            }
        }
    }
    str_bin
}

fn binario_para_texto(bin: &str) -> String {
    bin.split_whitespace()
       .filter_map(|byte| {
           if byte.len() == 8 {
               u8::from_str_radix(byte, 2).ok().map(|b| b as char)
           } else {
               None
           }
       })
       .collect()
}

pub fn decode_base64(input: &str) -> String {
    let s_bin = converte_base64_bin(input);
    let sbin_sep = separa_string_binaria(&s_bin, 8);
    binario_para_texto(&sbin_sep)
}
