#[macro_use] 
extern crate lazy_static;

pub mod parsing;

#[cfg(test)]
mod tests{
	use parsing::encode;
	use parsing::decode;
	use parsing:: parse;

	#[test]
	fn it_works(){
		//Basic URL
		let y = "http://www.google.com".to_string();
		let mut x : String = encode::encode_url_string(y);
		println!("------------------------------");
		assert_eq!("http%3A%2F%2Fwww.google.com", x);
		x = decode::decode_url_string(x);
		assert_eq!("http://www.google.com", x);

		//Spaces
		let space_test = "asdf asdf".to_string();
		let mut enc_space : String = encode::encode_url_string(space_test);
		assert_eq!("asdf+asdf", enc_space);
		enc_space = decode::decode_url_string(enc_space);
		assert_eq!("asdf asdf", enc_space);

		//Url Parser
		let w_scheme: String = String::from("http://www.google.com");
		let wo_scheme: String = String::from("www.google.com");
		let wparams: String = String::from("www.google.com/seeme");


		parse::url_parse(w_scheme);
		parse::url_parse(wo_scheme);
		parse::url_parse(wparams);

		//Temp so test prints
		assert_eq!(1,2);
	}
}