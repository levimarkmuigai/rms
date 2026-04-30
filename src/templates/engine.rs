use std::collections::HashMap;

pub fn render(template: &str, ctx: &HashMap<&str, String>) -> String {
    let mut out = template.to_string();
    for (k, v) in ctx {
        out = out.replace(&format!("{{{{{k}}}}}"), v);
        out = out.replace(&format!("{{{{ {k} }}}}"), v);
    }
    out
}
