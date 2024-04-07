use askama::Template;
use std::fmt::Display;

#[derive(Template)]
#[template(path = "request.template.php", escape = "none")]
pub struct RequestTemplate<'a> {
    pub class_name: &'a str,
    pub fields: Vec<Field<'a>>,
}

pub struct Field<'a> {
    pub name: &'a str,
    pub rules: Vec<&'a str>,
}

// TODO: I'm not sure this is necessary
impl<'a> Display for Field<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = f.fill();
        write!(f, "{c}")
    }
}
