extern crate hex;

use hex::ToHex;

pub fn to_code(c: char) -> String{
	c.to_hex()
}