use minreq;

pub fn get(url: String) -> String {
    let resp = minreq::get(url).send().unwrap();
    return resp.as_str().unwrap().to_string();
}
