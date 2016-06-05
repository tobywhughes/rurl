use parsing::hex::FromHex;

pub fn decode_url_string(s: String) -> String{
	let mut ret_string = "".to_string();
	let mut percent_counter: u8 = 0;
	let mut percent_string = "".to_string();

	for c in s.chars(){
		if c != '%' && percent_counter == 0{
			if c != '+' {
				ret_string.push(c);
			} 
			else{
				ret_string.push(' ');
			}
		}
		else if c == '%' {
			percent_counter = 2;
		}
		else {
			if percent_counter == 2 {
				percent_counter -= 1;
				percent_string.push(c);
			}
			else {
				percent_counter -= 1;
				percent_string.push(c);
				let v: Vec<u8> = Vec::from_hex(percent_string).unwrap();
				let s = String::from_utf8(v).unwrap();
				ret_string.push_str(&s);
				percent_string = "".to_string();
			}
		}

	}

	ret_string
}