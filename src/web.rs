pub fn add_heading_block(nonce: String, hash: String) {
    let window = web_sys::window().expect("no window found");
    let document = window.document().expect("no window found");
    let body = document.body().expect("no window found");
    let heading = document.create_element("h1").expect("no window found");
    heading.set_inner_html(&format!("Nonce: {}, Hash: {}", nonce, hash));
    body.append_child(&heading).unwrap();
}
