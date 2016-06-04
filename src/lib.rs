#[macro_use] 
extern crate lazy_static;

pub mod parsing;

#[cfg(test)]
mod tests{
	use parsing::encode;
	use parsing::decode;
	#[test]
	fn it_works(){
		let y = "http:\\\\www.google.com".to_string();
		let mut x : String = encode::encode_url_string(y);
		println!("------------------------------");
		assert_eq!("http%3A%5C%5Cwww.google.com", x);
		x = decode::decode_url_string(x);
		assert_eq!("http:\\\\www.google.com", x);
	}
}