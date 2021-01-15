#![allow(dead_code)]

#[derive(Debug)]
enum Node {
    Object(Object),
    Array(Array),
    Literal(Literal),
}

#[derive(Debug)]
struct Object {
    children: Vec<Property>,
    span: Span,
}

#[derive(Debug)]
struct Property {
    key: Literal,
    value: PropertyValue,
    span: Span,
}

#[derive(Debug)]
enum PropertyValue {
    Object(Object),
    Array(Array),
    Literal(Literal),
}

#[derive(Debug)]
struct Array {
    children: Vec<PropertyValue>,
    span: Span,
}

#[derive(Debug)]
struct Literal {
    value: LiteralValue,
    span: Span,
}

#[derive(Debug)]
enum LiteralValue {
    Str(String),
    Num(i64),
    Bool(bool),
    Null,
}

#[derive(Debug)]
struct Span {
    start: Point,
    end: Point,
}

#[derive(Debug)]
struct Point {
    line: u64,
    column: u64,
}

pub struct AST {
    line: u64,
    column: u64,
    pointer: usize,
    size: usize,
    nodes: Vec<Node>,
}

impl AST {
    fn new(file: &str) -> Self {
        Self {
            line: 1,
            column: 1,
            pointer: 0,
            size: file.len(),
            nodes: vec![],
        }
    }
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
    fn parse_tree(state: &mut Self, file: &str) -> Vec<Node> {
        let mut result: Vec<Node> = vec![];
        while state.pointer < state.size {
            let curr = Self::get_chr(state.pointer, file);
            match curr {
                '{' => {
                    result.push(Node::Object(Self::object(file, state)));
                }
                '[' => {
                    result.push(Node::Array(Self::array(file, state)));
                }
                '"' => {
                    result.push(Node::Literal(Self::string(file, state)));
                }
                ' ' => {
                    Self::consume_many(' ', state, file);
                }
                _ => {
                    if Self::check_next(curr, "number") {
                        result.push(Node::Literal(Self::number(file, state)));
                    } else if Self::check_next(curr, "new_line") {
                        state.pointer += 1;
                        state.line += 1;
                        state.column = 1;
                    }
                }
            }
            state.pointer += 1;
            state.column += 1;
        }
        result
    }

    fn array(file: &str, state: &mut AST) -> Array {
        let start = Point {
            line: state.line,
            column: state.column,
        };
        Self::consume('[', state, file);
        let mut arr: Vec<PropertyValue> = vec![];
        while Self::get_chr(state.pointer, file) != ']' {
            match Self::get_chr(state.pointer, file) {
                '{' => {
                    arr.push(PropertyValue::Object(Self::object(file, state)));
                }
                '[' => {
                    arr.push(PropertyValue::Array(Self::array(file, state)));
                }
                '"' => {
                    arr.push(PropertyValue::Literal(Self::string(file, state)));
                }
                ' ' => {
                    Self::consume_many(' ', state, file);
                }
                _ => {
                    arr.push(PropertyValue::Literal(Self::abstract_literal(file, state)));
                }
            }
            Self::consume_many(' ', state, file);
            Self::consume_or(',', state, file);
            Self::consume_many(' ', state, file);
        }
        let end = Point {
            line: state.line,
            column: state.column,
        };
        Self::consume(']', state, file);
        Array {
            children: arr,
            span: Span { start, end },
        }
    }

    fn object(file: &str, state: &mut AST) -> Object {
        Self::consume('{', state, file);
        let mut children: Vec<Property> = vec![];
        let start = Point {
            line: state.line,
            column: state.column,
        };
        while Self::get_chr(state.pointer, file) != '}' {
            let start = Point {
                line: state.line,
                column: state.column,
            };
            let key: Literal;
            let value: PropertyValue;
            Self::consume_many(' ', state, file);
            match Self::get_chr(state.pointer, file) {
                '"' => {
                    key = Self::string(file, state);
                    println!("{:?}", key);
                }
                _ => {
                    panic!();
                }
            };
            Self::consume_many(' ', state, file);
            Self::consume(':', state, file);
            Self::consume_many(' ', state, file);
            match Self::get_chr(state.pointer, file) {
                '{' => {
                    value = PropertyValue::Object(Self::object(file, state));
                }
                '[' => {
                    value = PropertyValue::Array(Self::array(file, state));
                }
                '"' => {
                    value = PropertyValue::Literal(Self::string(file, state));
                }
                _ => {
                    value = PropertyValue::Literal(Self::abstract_literal(file, state));
                }
            };
            let end = Point {
                line: state.line,
                column: state.column,
            };
            children.push(Property {
                key,
                value,
                span: Span { start, end },
            });
            Self::consume_many(' ', state, file);
            Self::consume_or(',', state, file);
            Self::consume_many(' ', state, file);
        }
        let end = Point {
            line: state.line,
            column: state.column,
        };
        Self::consume('}', state, file);
        Object {
            children,
            span: Span { start, end },
        }
    }

    fn string(file: &str, state: &mut AST) -> Literal {
        let start = Point {
            line: state.line,
            column: state.column,
        };
        Self::consume('"', state, file);
        let curr_pointer = state.pointer;
        Self::match_until('"', state, file);
        let lit = LiteralValue::Str(file[curr_pointer..state.pointer].to_string());
        let end = Point {
            line: state.line,
            column: state.column,
        };
        return Literal {
            value: lit,
            span: Span { start, end },
        };
    }

    fn abstract_literal(file: &str, state: &mut AST) -> Literal {
        let curr = Self::get_chr(state.pointer + 1, file);
        if Self::check_next(curr, "number") {
            return Self::number(file, state);
        } else {
            return Self::boolean_null(file, state);
        }
    }

    fn number(file: &str, state: &mut AST) -> Literal {
        let curr_pointer = state.pointer;
        let start = Point {
            line: state.line,
            column: state.column,
        };
        while true {
            let res = Self::get_chr(state.pointer, file)
                .to_string()
                .parse::<i64>();
            match res {
                Ok(_) => {
                    state.pointer += 1;
                    state.column += 1;
                }
                Err(_) => {
                    let end = Point {
                        line: state.line,
                        column: state.column,
                    };
                    return Literal {
                        value: LiteralValue::Num(
                            file[curr_pointer..state.pointer]
                                .to_string()
                                .parse::<i64>()
                                .unwrap(),
                        ),
                        span: Span { start, end },
                    };
                }
            }
        }
        todo!()
    }

    fn boolean_null(file: &str, state: &mut AST) -> Literal {
        let start = Point {
            line: state.line,
            column: state.column,
        };
        if &file[state.pointer..state.pointer + 4] == "true" {
            state.pointer += 4;
            state.column += 4;
            let end = Point {
                line: state.line,
                column: state.column,
            };
            return Literal {
                value: LiteralValue::Bool(true),
                span: Span { start, end },
            };
        } else if &file[state.pointer..state.pointer + 5] == "false" {
            state.pointer += 5;
            state.column += 5;
            let end = Point {
                line: state.line,
                column: state.column,
            };
            return Literal {
                value: LiteralValue::Bool(false),
                span: Span { start, end },
            };
        } else {
            println!("in");
            state.pointer += 4;
            state.column += 4;
            let end = Point {
                line: state.line,
                column: state.column,
            };
            return Literal {
                value: LiteralValue::Null,
                span: Span { start, end },
            };
        }
    }

    fn get_chr(pos: usize, file: &str) -> char {
        return file.chars().nth(pos).expect("failed here");
    }

    fn consume(chr: char, state: &mut AST, file: &str) {
        if Self::get_chr(state.pointer, file) != chr {
            panic!("Error");
        }
        state.pointer += 1;
        state.column += 1;
    }

    fn consume_or(chr: char, state: &mut AST, file: &str) {
        if Self::get_chr(state.pointer, file) != chr {
            return;
        }
        state.pointer += 1;
        state.column += 1;
    }

    fn consume_many(chr: char, state: &mut AST, file: &str) {
        while Self::get_chr(state.pointer, file) == chr {
            state.pointer += 1;
            state.column += 1;
        }
    }

    fn check_next(chr: char, instance: &str) -> bool {
        match instance {
            "number" => match chr.to_string().parse::<i64>() {
                Ok(_) => {
                    return true;
                }
                Err(_) => {
                    return false;
                }
            },
            "new_line" => {
                return chr == '\n';
            }
            _ => {
                return true;
            }
        }
    }

    fn match_until(chr: char, state: &mut AST, file: &str) {
        while Self::get_chr(state.pointer, file) != chr {
            state.pointer += 1;
            state.column += 1;
        }
        state.pointer += 1;
        state.column += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_chr() {
        assert_eq!(AST::get_chr(1, "433"), '3');
    }
    #[test]
    fn basic_str() {
        let mut ast = AST::new("fefwewe");
        let temp = r#""adasnf""#;
        println!("{}", temp);
        println!("{:?}", AST::string(temp, &mut ast));
        assert!(true);
    }
    #[test]
    fn basic_array() {
        let mut ast = AST::new(r#"{"a":[55,6,7,null]}"#);
        let temp = r#"{"a":[55,6,7,null]}"#;
        println!("{:?}", AST::parse_tree(&mut ast, temp));
        assert!(true);
    }
    #[test]
    fn basic_object() {
        let mut ast = AST::new(r#"{"a":5,"b":[4,5, "gf"]}"#);
        let temp = r#"{"a":5,"b":[4,5 , "gf"]}"#;
        println!("{:?}", AST::object(temp, &mut ast));
        assert!(true);
    }
}
