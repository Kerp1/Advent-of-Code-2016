use crypto::md5::Md5;
use crypto::digest::Digest;

pub fn is_valid_hash(output : &[u8]) -> bool {
    output[0] as i32 + output[1] as i32 + (output[2] >> 4) as i32 == 0
}

pub fn dec_to_hex_str(dec: u8) -> String {
    format!("{:x}", dec).to_owned()
}


pub fn task1() -> String {
    let data = "cxdnnyjw";

    let mut password = String::new();
    let mut md5 = Md5::new();
    for i in 0..u64::max_value() {
        md5.input(data.as_bytes());
        md5.input(i.to_string().as_bytes());

        let mut output = [0; 16];
        md5.result(&mut output);

        if is_valid_hash(&output)  {
            password.push_str(&dec_to_hex_str(output[2]));
            if password.len() >= 8 {
                break;
            }
        }
        md5.reset();
    }
    password
}

pub fn task2() -> String {
    let data = "cxdnnyjw";

    let mut password = String::from("00000000");
    let mut md5 = Md5::new();
    let mut count = 1;

    for i in 0..u64::max_value() {
        md5.input(data.as_bytes());
        md5.input(i.to_string().as_bytes());

        let mut output = [0; 16];
        md5.result(&mut output);

        if is_valid_hash(&output)  {
            let position = match dec_to_hex_str(output[2]).parse::<usize>() {
                Ok(v) => v,
                Err(_) => {
                    md5.reset();
                    continue;
                }
            };
            if position < 8 {
                if count & 2 << position == 0 {
                    password = password.chars().enumerate().map(|(i, c)| {
                        if position == i {
                            dec_to_hex_str(output[3]).chars().next().unwrap()
                        } else {
                            c
                        }
                    }).collect();
                    count += 2 << position;
                    println!("password = {:?}", password);
                }

                if count == 511 {
                    break;
                }
            }
        }
        md5.reset();
    }
    password.replace("k", "")
}
