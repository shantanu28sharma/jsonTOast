#![allow(dead_code)]

enum Node {
    Object(Object),
    Array(Array),
    LiteralValue(LiteralValue),
}

#[derive(Default)]
struct Object {
    children: Vec<Property>,
}

// #[derive(Default)]
struct Property {
    key: LiteralValue,
    value: PropertyValue,
}

// #[derive(Default)]
enum PropertyValue {
    Object(Object),
    Array(Array),
    LiteralValue(LiteralValue),
}

#[derive(Default)]
struct Identifier {
    value: Literal,
    raw: String,
}

#[derive(Default)]
struct Array {
    children: Vec<Option<PropertyValue>>,
}

#[derive(Default)]
struct Literal {
    value: Option<LiteralValue>,
    raw: String,
}

// #[derive(Default)]
enum LiteralValue {
    Str(String),
    Num(i64),
    Bool(bool),
}

struct Span {
    start: Point,
    end: Point,
}

struct Point {
    line: u64,
    column: u64,
}

struct AST {
    line: u64,
    column: u64,
    pointer: usize,
    size: usize,
    nodes: Vec<Node>,
}

enum CurrentElem {
    Array,
}

impl AST {
    fn build_ast(file: &str) -> Vec<Node> {
        let mut state = Self {
            line: 1,
            column: 1,
            pointer: 0,
            size: file.len(),
            nodes: vec![],
        };
        Self::parse_tree(&mut state, file);
        vec![]
    }
    fn parse_tree(state: &mut Self, file: &str) -> Vec<Option<Node>> {
        let mut result: Vec<Option<Node>> = vec![];
        while state.pointer != state.size {
            let curr = Self::get_chr(state.pointer, file);
            match curr {
                '{' => {
                    result.push(Some(Node::Object(Self::object(file, state))));
                }
                '[' => {
                    result.push(Some(Node::Array(Self::array(file, state))));
                }
                '"' => {
                    result.push(Some(Node::LiteralValue(Self::string(file, state))));
                }
                _ => {
                    result.push(Some(Node::LiteralValue(Self::number(file, state))));
                }
            }
            state.pointer += 1;
        }
        result
    }

    fn array(file: &str, state: &mut AST) -> Array {
        Self::consume('[', state, file);
        let mut arr: Vec<Option<PropertyValue>> = vec![];
        while Self::get_chr(state.pointer, file) != ']' {
            match Self::get_chr(state.pointer, file) {
                '{' => {
                    arr.push(Some(PropertyValue::Object(Self::object(file, state))));
                }
                '[' => {
                    arr.push(Some(PropertyValue::Array(Self::array(file, state))));
                }
                '"' => {
                    arr.push(Some(PropertyValue::LiteralValue(Self::string(file, state))));
                }
                _ => {
                    arr.push(Some(PropertyValue::LiteralValue(Self::number(file, state))));
                }
            }
            Self::consume(',', state, file);
        }
        Array { children: arr }
    }

    fn object(file: &str, state: &mut AST) -> Object {
        Self::consume('{', state, file);
        let mut children : Vec<Property> = vec![];
        while Self::get_chr(state.pointer, file) != '}' {
            let key: LiteralValue;
            let value: PropertyValue;
            match Self::get_chr(state.pointer, file) {
                '"' => {
                    key = Self::string(file, state);
                }
                _ => {
                    key = Self::number(file, state);
                }
            };
            Self::consume(':', state, file);
            match Self::get_chr(state.pointer, file) {
                '{' => {
                    value = PropertyValue::Object(Self::object(file, state));
                }
                '[' => {
                    value = PropertyValue::Array(Self::array(file, state));
                }
                '"' => {
                    value = PropertyValue::LiteralValue(Self::string(file, state));
                }
                _ => {
                    value = PropertyValue::LiteralValue(Self::number(file, state));
                }
            };
            children.push(Property{
                key,
                value
            });
            Self::consume(',', state, file);
        }

        Object {
            children,
        }
    }

    fn number(file: &str, state: &mut AST) -> LiteralValue {
        let curr_pointer = state.pointer;
        while true {
            let res = Self::get_chr(state.pointer, file)
                .to_string()
                .parse::<i64>();
            match res {
                Ok(_) => {
                    state.pointer += 1;
                }
                Err(_) => {
                    return LiteralValue::Num(
                        file[curr_pointer..state.pointer + 1]
                            .to_string()
                            .parse::<i64>()
                            .unwrap(),
                    );
                }
            }
        }
        todo!()
    }

    fn string(file: &str, state: &mut AST) -> LiteralValue {
        Self::consume('"', state, file);
        let curr_pointer = state.pointer;
        Self::match_until('"', state, file);
        let lit = LiteralValue::Str(file[curr_pointer..state.pointer].to_string());
        return lit;
    }

    fn get_chr(pos: usize, file: &str) -> char {
        return file.chars().nth(pos).unwrap();
    }

    fn consume(chr: char, state: &mut AST, file: &str) {
        if file.chars().nth(state.pointer).unwrap() != chr {
            panic!("Error");
        }
        state.pointer += 1;
    }

    fn match_until(chr: char, state: &mut AST, file: &str) {
        while Self::get_chr(state.pointer, file) != chr {
            state.pointer += 1;
        }
        state.pointer += 1;
    }
}

#[cfg(test)]
mod tests{
    
}