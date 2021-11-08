pub struct Scanner<'code> {
    code: &'code String,
}

/**
 * Basic scanner implementation
 **/
impl<'code> Scanner<'code> {
    pub fn new(code: &'code String) -> Self {
        Scanner { code }
    }

    pub fn scan_tokens(self) {
        println!("{}", self.code);
    }
}
