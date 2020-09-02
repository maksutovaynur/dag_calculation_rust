type ArgType = i32;

#[derive(Debug)]
pub enum CellKind {
    FORMULA,
    VALUE(ArgType),
}

#[derive(Debug)]
pub struct ParsedInput {
    pub name: String,
    pub kind: CellKind,
    pub refs: Vec<String>,
}


pub fn parse(input: &str) -> Option<ParsedInput> {
    let lst1: Vec<&str> = input.split(" = ").collect();
    if lst1.len() < 2 { return None; }
    let mut refs: Vec<String> = lst1[1].split("+").map(|s| s.to_string()).collect();
    let kind: CellKind = if refs.len() == 1 {
        match refs[0].parse() {
            Result::Err(_) => CellKind::FORMULA,
            Result::Ok(v) => {
                refs.clear();
                CellKind::VALUE(v)
            }
        }
    } else { CellKind::FORMULA };
    Some(ParsedInput {
        name: lst1[0].to_string(),
        kind,
        refs,
    })
}
