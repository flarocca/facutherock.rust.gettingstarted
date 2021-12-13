pub trait Renderer {
    fn render(&self, pattern: &str);
}

pub struct PrintLnRenderer;

impl PrintLnRenderer {
    pub fn new () -> Self {
        PrintLnRenderer{}
    }
}

impl Renderer for PrintLnRenderer {

    fn render(&self, pattern: &str) {
        println!("{}", pattern);
    }
}