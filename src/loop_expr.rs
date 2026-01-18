
pub fn parse_loop_expr(text: &str) -> (String, bool) {
    let mut answer = String::new();
    let mut retur = false;
    if let Some((loop_type, expr)) = text.split_once(' '){
        if loop_type.starts_with("пока") {
            answer.push_str("while ");
            if answer.contains("Правда") {
                answer.push_str("true");
            } else {
                if let Some(expr) = expr.strip_suffix("выполнить") {
                    retur = true;
                    answer.push_str(expr);
                    answer.push('{');
                }
                else {
                    answer.push_str(expr);
                }
            }
        } else if loop_type.starts_with("для") {
            answer.push_str("for ");
            if let Some((name, diap)) = expr.split_once(" в ") {
                if let Some(name) = name.trim().strip_suffix(')').and_then(|x| x.strip_prefix("всех(")) {
                    answer.push_str(name);
                }
                answer.push_str(" in ");
                if let Some(diap) = diap.strip_suffix(") выполнить") {
                    retur = true;
                    if let Some((_, diap)) = diap.split_once('(') {
                        answer.push_str(diap);
                    }
                    answer.push('{')
                } 
                else if let Some((_, diap)) = diap.trim().split_once('(') 
                    && let Some(diap) = diap.strip_suffix(')') {
                        answer.push_str(diap);
                }
            }
                 
        }
    }
    (answer, retur) 
}
