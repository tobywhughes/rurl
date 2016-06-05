use parsing::regex::Regex;

pub fn url_parse(url : String) -> ParsedUrl{
	let mut purl = ParsedUrl {scheme: "".to_string(), net_loc:"".to_string(), path: "".to_string(), 
							  params: "".to_string(), query: "".to_string(), frag: "".to_string()};

	purl.scheme = get_scheme(&url);

	purl.net_loc = get_net_loc(&url, purl.scheme.len());

	//So that the compiler doesn't complain while implementing
	purl.path = purl.path;
	purl.params = purl.params;
	purl.query = purl.query;
	purl.frag = purl.frag;

	//Quick tests
	println!("{}", purl.scheme);
	println!("{}", purl.net_loc);
	purl
}

fn get_scheme(url: &String) -> String {
	let mut scheme_string: String = "".to_string();
	let mut col_flag : bool = false;

	for c in url.chars(){
		if col_flag == false{
			if c == ':' {
				col_flag = true;
			}
			else {
				scheme_string.push(c);
			}
		}
		else {
			if c == '/' {
				break;
			}
			//Compliant with RFC 1738 - 3.1
			else {
				scheme_string = "".to_string();
				break;
			}
		}
	}

	if col_flag == false{
		scheme_string = "".to_string();
	}

	scheme_string
}

fn get_net_loc(url: &String, mut scheme_len: usize) -> String{
	if scheme_len != 0{
		scheme_len += 3;
	}

	let mut net_string: String = "".to_string();
	lazy_static!{
		static ref RE: Regex = Regex::new(r"[/;\?#]").unwrap();
	}

	for c in url.chars(){
		if scheme_len != 0 {
			scheme_len -= 1;
		}
		else {
			if RE.is_match(&c.to_string()) == false {
				net_string.push(c);
			} 
			else {
				break;
			}
		}
	}

	net_string
}

pub struct ParsedUrl {
	scheme: String,
	net_loc: String,
	path: String,
	params: String,
	query: String,
	frag: String,
}