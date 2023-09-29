pub fn encode(instr: &str) -> String {
    let mut retval = String::default();
    for byt in instr.as_bytes() {
        //note utf-8 multi-byte code points consist entirely of bytes greater than 127, so matching alphanumeric bytes will be safe
        match byt {
            b'.' | b'0'..=b'9' | b'A'..=b'Z' | b'a'..=b'z' => {
                retval.push(*byt as char);
            }
            _ => {
                retval.push('_');
                retval.push_str(&format!("{:02x}", *byt));
            }
        }
    }
    retval
}

pub fn decode(instr: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut retval = Vec::<u8>::new();
    let mut bytes = instr.bytes();
    loop {
        let chr = bytes.next();
        let chr = match chr {
            None => {
                break;
            }
            Some(inner) => inner,
        };
        match chr {
            b'.' | b'0'..=b'9' | b'A'..=b'Z' | b'a'..=b'z' => {
                retval.push(chr);
            }
            b'_' => {
                let b1 = bytes.next().ok_or("Input is invalid.")?;
                let b2 = bytes.next().ok_or("Input is invalid.")?;
                let byt = u8::from_str_radix(std::str::from_utf8(&[b1, b2])?, 16)?;
                retval.push(byt);
            }
            _ => {
                return Err("Unexpected byte.".into());
            }
        }
    }
    let retval = String::from_utf8(retval)?;
    Ok(retval)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(encode("qwe.qwe"), "qwe.qwe".to_owned());
        assert_eq!(encode("qwe._qwe"), "qwe._5fqwe".to_owned());
        assert_eq!(encode("q@we._qwe"), "q_40we._5fqwe".to_owned());
        assert_eq!(decode("qwe.qwe").unwrap(), "qwe.qwe".to_owned());
        assert_eq!(decode("qwe._5fqwe").unwrap(), "qwe._qwe".to_owned());
        assert_eq!(decode("q_40we._5fqwe").unwrap(), "q@we._qwe".to_owned());
    }
}
