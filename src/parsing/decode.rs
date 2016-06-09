use parsing::hex::FromHex;

/// Decodes a percent-encoded URL string
///
/// #Example
/// ```
/// use rurl::parsing::decode;
/// let mut enc_url : String= "http%3A%2F%2Fwww.google.com".to_string();
/// 
/// enc_url = decode::decode_url_string(enc_url);
/// ```
pub fn decode_url_string(s: String) -> String{
	let mut ret_string = "".to_string();
	let mut percent_counter: u8 = 0;
	let mut percent_string = "".to_string();

	for c in s.chars(){
		if c != '%' && percent_counter == 0{
			if c != '+' {
				ret_string.push(c);
			}
			//Handles spaces, which are encoded, but not percent encoded 
			else{
				ret_string.push(' ');
			}
		}
		else if c == '%' {
			percent_counter = 2;
		}
		//Percent encoded sequence has been found 
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