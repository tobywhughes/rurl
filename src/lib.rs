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
		//URL Encode/Decode Tests
		let mut enc_url : String= "http://www.google.com".to_string();
		enc_url = encode::encode_url_string(enc_url);
		assert_eq!("http%3A%2F%2Fwww.google.com", enc_url);
		enc_url = decode::decode_url_string(enc_url);
		assert_eq!("http://www.google.com", enc_url);
		enc_url = String::from("http://www.google.com/path;type=a?query#fragment");
		enc_url = encode::encode_url_string(enc_url);
		assert_eq!("http%3A%2F%2Fwww.google.com%2Fpath%3Btype%3Da%3Fquery%23fragment", enc_url);
		enc_url = decode::decode_url_string(enc_url);
		assert_eq!("http://www.google.com/path;type=a?query#fragment", enc_url);


		//Spaces
		let space_test = "asdf asdf".to_string();
		let mut enc_space : String = encode::encode_url_string(space_test);
		assert_eq!("asdf+asdf", enc_space);
		enc_space = decode::decode_url_string(enc_space);
		assert_eq!("asdf asdf", enc_space);

		//Url Parser
		let w_scheme: String = String::from("http://www.google.com");
		let wo_scheme: String = String::from("www.google.com");
		let wparams: String = String::from("http://www.google.com/path/path");
		let everything : String = String::from("http://www.google.com/path;type=a?query#fragment");

		//Parsed Urls
		let purl1 = parse::url_parse(w_scheme);
		let purl2 = parse::url_parse(wo_scheme);
		let purl3 = parse::url_parse(wparams);
		let purl4 = parse::url_parse(everything);

		//Parsed Tests
		assert_eq!(purl1.scheme, "http");
		assert_eq!(purl2.scheme, "");
		assert_eq!(purl3.net_loc, "www.google.com");
		assert_eq!(purl3.path, "/path/path");
		assert_eq!(purl4.params, "type=a");
		assert_eq!(purl4.query, "query");
		assert_eq!(purl4.frag, "fragment");	

		//Unparsing test
		let unparsed1 : String = parse::url_join_parsed(purl2);
		assert_eq!(unparsed1, "www.google.com");
		let unparsed2 : String = parse::url_join_parsed(purl4);
		assert_eq!(unparsed2, "http://www.google.com/path;type=a?query#fragment");
	}
}