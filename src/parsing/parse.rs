use parsing::regex::Regex;

pub fn url_parse(url : String) -> ParsedUrl{
	let mut purl = ParsedUrl {scheme: "".to_string(), net_loc:"".to_string(), path: "".to_string(), 
							  params: "".to_string(), query: "".to_string(), frag: "".to_string()};

	purl.scheme = get_scheme(&url);

	purl.net_loc = get_net_loc(&url, purl.scheme.len());

	//Extra length to scheme if it exits (purl.scheme doesn't account for :// after scheme)
	let mut scheme_extra : usize = 0;
	if purl.scheme.len() != 0{
			scheme_extra = 3;
	}

	//Cuts off the scheme and net_loc
	let cut_url : String = cut_start(&url, purl.scheme.len() + purl.net_loc.len() + scheme_extra);

	purl.path = get_path(&cut_url);
	purl.params = get_params(&cut_url);
	purl.query = get_query(&cut_url);
	purl.frag = get_frag(&cut_url);

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

fn get_path(url : &String) -> String {
	let mut path_flag : bool= false;
	let mut path_string : String = "".to_string();

	lazy_static!{
		static ref RE : Regex = Regex::new(r"[;\?#]").unwrap();
	}

	for c in url.chars(){
		if path_flag == false {
			if c == '/'{
				path_flag = true;
				path_string.push(c);
			}
		}
		else {
			if RE.is_match(&c.to_string()) {
				break;
			}
			else{
				path_string.push(c);
			}
		}
	}

	path_string
}

fn get_params(url : &String) -> String {
	let mut param_flag : bool= false;
	let mut param_string : String = "".to_string();

	lazy_static!{
		static ref RE : Regex = Regex::new(r"[/\?#]").unwrap();
	}

	for c in url.chars(){
		if param_flag == false {
			if c == ';'{
				param_flag = true;
				param_string.push(c);
			}
		}
		else {
			if RE.is_match(&c.to_string()) {
				break;
			}
			else{
				param_string.push(c);
			}
		}
	}

	param_string
}

fn get_query(url : &String) -> String {
	let mut query_flag : bool= false;
	let mut query_string : String = "".to_string();

	lazy_static!{
		static ref RE : Regex = Regex::new(r"[/;#]").unwrap();
	}

	for c in url.chars(){
		if query_flag == false {
			if c == '?'{
				query_flag = true;
				query_string.push(c);
			}
		}
		else {
			if RE.is_match(&c.to_string()) {
				break;
			}
			else{
				query_string.push(c);
			}
		}
	}

	query_string
}

fn get_frag(url : &String) -> String {
	let mut frag_flag : bool= false;
	let mut frag_string : String = "".to_string();

	lazy_static!{
		static ref RE : Regex = Regex::new(r"[/;\?]").unwrap();
	}

	for c in url.chars(){
		if frag_flag == false {
			if c == '#'{
				frag_flag = true;
				frag_string.push(c);
			}
		}
		else {
			if RE.is_match(&c.to_string()) {
				break;
			}
			else{
				frag_string.push(c);
			}
		}
	}

	frag_string
}

fn cut_start(url : &String, mut del_len : usize) -> String{
	let mut cut_string : String = "".to_string();

	for c in url.chars(){
		if del_len != 0 {
			del_len -= 1;
		}
		else {
			cut_string.push(c);
		}
	}

	cut_string
}

pub struct ParsedUrl {
	pub scheme: String,
	pub net_loc: String,
	pub path: String,
	pub params: String,
	pub query: String,
	pub frag: String,
}