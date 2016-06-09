use parsing::hex::ToHex;
use parsing::regex::Regex;

/// Percent-encodes a URL string
///
/// #Example
/// ```
/// use rurl::parsing::encode;
/// let mut enc_url : String= "http://www.google.com".to_string();
/// 
/// enc_url = encode::encode_url_string(enc_url);
/// ```
pub fn encode_url_string(s: String) -> String{
	let mut ret_string = "".to_string();

	lazy_static!{
		//Check for more character possibilities
		static ref RE: Regex = Regex::new(r"[-a-zA-Z1-9._~]").unwrap();
	}
	
	for c in s.chars(){
		if RE.is_match(&c.to_string()){
			ret_string.push(c);
		}
		//Handles spaces which are encoded, but not percent-encoded
		else if c == ' '{
			ret_string.push('+');
		}
		else {
			ret_string.push_str(&format!("%{}", c.to_string().to_hex().to_uppercase()));
		}
	}

	ret_string
}