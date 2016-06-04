use parsing::hex::ToHex;
use parsing::regex::Regex;

pub fn encode_url_string(s: String) -> String{
	let mut ret_string = String::from("");
	lazy_static!{
		//Check for more character possibilities
		static ref RE: Regex = Regex::new(r"[-a-zA-Z1-9._~]").unwrap();
	}
	for c in s.chars(){
		if RE.is_match(&c.to_string()){
			ret_string.push(c);
		}
		else {
			ret_string.push_str(&format!("%{}", c.to_string().to_hex().to_uppercase()));
		}
	}

	ret_string
}