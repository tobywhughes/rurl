
pub mod parsing;

#[cfg(test)]
mod tests{
	use parsing::encode;
	#[test]
	fn it_works(){

		let x: String = encode::to_code('A');
		println!("------------------------------");
		assert_eq!("65", x);
	}
}