use crypto::md5::Md5;
use crypto::digest::Digest;


pub fn task1() -> String {
    let data = "cxdnnyjw";

    let mut password = String::new();
    let mut md5 = Md5::new();
    for i in 0..u64::max_value() {
        md5.input(data.as_bytes());
        md5.input(i.to_string().as_bytes());

        let mut output = [0; 16];
        md5.result(&mut output);

        let first_five_characters = output[0] as i32 + output[1] as i32 + (output[2] >> 4) as i32;
        if first_five_characters == 0 {
            password.push_str(&format!("{:x}", output[2]));

            if password.len() >= 8 {
                break;
            }
        }

        md5.reset();
    }
    password
}
