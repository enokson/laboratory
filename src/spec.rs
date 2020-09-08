pub struct Spec {
    pub name: String,
    pub test: Box<dyn Fn() -> Result<(), String>>,
    pub pass: Option<bool>,
    pub error_msg: Option<String>,
    pub ignore: bool
}
impl Spec {
    pub fn new <T>(name: String, handle: T) -> Spec
        where
            T: Fn() -> Result<(), String> + 'static
    {
        Spec {
            name,
            test: Box::new(handle),
            pass: None,
            error_msg: None,
            ignore: false
        }
    }
    pub fn skip (mut self) -> Self {
        self.ignore = true;
        self
    }
    pub fn run(&mut self) {
        let test = self.test.as_ref();
        if self.ignore == false {
            match (test)() {
                Ok(_) => {
                    self.pass = Some(true);
                }
                Err(message) => {
                    self.pass = Some(false);
                    self.error_msg = Some(message);
                },
                _ => {
                    self.pass = Some(false);
                    self.error_msg = Some("something happened".to_string());
                }
            }
        }
    }
}