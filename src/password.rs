use rand::{thread_rng, Rng};

pub struct PasswordParameters {
    password_length: i32,
    include_special_characters: bool,
    use_numbers: bool,
    use_upper_case: bool,
    use_underlines: bool,
}

impl PasswordParameters {
    pub fn new(
        password_length: i32,
        include_special_characters: bool,
        use_numbers: bool,
        use_upper_case: bool,
        use_underlines: bool,
    ) -> PasswordParameters 
    {
        PasswordParameters {
            password_length: password_length,
            include_special_characters: include_special_characters,
            use_numbers: use_numbers,
            use_upper_case: use_upper_case,
            use_underlines: use_underlines,
        }
    }
}

pub fn generate_password(passwd_parameters: PasswordParameters) -> String {
    let mut password = String::new();

    let mut allowed_characters = String::from("qwertyuiopasdfghjklzxcvbnm");

    if passwd_parameters.include_special_characters {
        allowed_characters += "!\"$#%'()*+`-./:;<=>?@[\\]^{|}~&";
    }
    if passwd_parameters.use_numbers {
        allowed_characters += "1234567890";
    }
    if passwd_parameters.use_upper_case {
        allowed_characters += "QWERTYUIOPASDFGHJKLZXCVBNM";
    }
    if passwd_parameters.use_underlines {
        allowed_characters += "_";
    }

    let mut rng = thread_rng();
    for _ in 0..passwd_parameters.password_length {
        let rand_num = rng.gen_range(0..allowed_characters.len());

        password.push(allowed_characters.as_bytes()[rand_num] as char);
    }

    password
}