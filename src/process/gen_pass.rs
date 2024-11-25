use rand::seq::SliceRandom;

const UPPER: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ"; // 大写字母（去除容易混淆的字母）
const LOWER: &[u8] = b"abcdefghijkmnopqrstuvwxyz"; // 小写字母（去除容易混淆的字母）
const NUMBER: &[u8] = b"123456789"; // 数字（去除0）
const SYMBOL: &[u8] = b"!@#$%^&*_"; // 符号

#[warn(unused_variables)]
pub fn process_genpass(
    length: u8,
    upper: bool,
    lower: bool,
    number: bool,
    symbol: bool,
) -> anyhow::Result<()> {
    if length == 0 {
        return Err(anyhow::anyhow!("密码长度必须大于0"));
    }

    if !upper && !lower && !number && !symbol {
        return Err(anyhow::anyhow!("至少选择一种字符类型"));
    }

    let mut rng = rand::thread_rng();
    let mut password = Vec::new();
    let mut chars = Vec::new();

    if upper {
        chars.extend_from_slice(UPPER);
        password.push(*UPPER.choose(&mut rng).expect("UPPER won't be empty"));
    }
    if lower {
        chars.extend_from_slice(LOWER);
        password.push(*LOWER.choose(&mut rng).expect("LOWER won't be empty"));
    }
    if number {
        chars.extend_from_slice(NUMBER);
        password.push(*NUMBER.choose(&mut rng).expect("NUMBER won't be empty"));
    }
    if symbol {
        chars.extend_from_slice(SYMBOL);
        password.push(*SYMBOL.choose(&mut rng).expect("SYMBOL won't be empty"));
    }

    // 计算剩余的密码长度
    let remaining_length  = if length as usize >= password.len(){
        length as usize - password.len()
    } else{
        return Err(anyhow::anyhow!("密码长度必须大于已选择的字符类型数量"))
    };
    if remaining_length > 0 {
        for _ in 0..remaining_length {
            let c = chars
                .choose(&mut rng)
                .expect("chars won't be empty in this context");
            password.push(*c);
        }
    }

    password.shuffle(&mut rng);

    println!("Password: {}", String::from_utf8(password)?);

    Ok(())
}
