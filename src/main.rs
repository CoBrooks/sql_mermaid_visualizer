use std::fs::File;
use std::io::{ Read, Write };

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    file: String,

    #[clap(short, long)]
    output_file: Option<String>
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let mut file = File::open(args.file).expect("Error opening file.");
    let mut contents: String = String::new();
    file.read_to_string(&mut contents)?;

    let l = lex(&contents);
    let e = parse_tokens(&l);
    let mermaid = exprs_to_mermaid(e);

    if let Some(output_path) = args.output_file {
        let mut output_file = File::create(output_path).expect("Error creating or opening file.");
        output_file.write_all(&mermaid.as_bytes()).expect("Error writing to file.");
    } else {
        println!("{}", mermaid);
    }

    Ok(())
}

#[allow(non_camel_case_types, dead_code)]
#[derive(PartialEq, Debug)]
enum Token {
    TableCreate(String),
    Field { name: String, ftype: String },
    TableAlter { table_name: String, foreign_key_name: String },
    ForeignKeyDetails { field_name: String, referenced_table: String },
    ExprEnd,
}

fn lex(s: &str) -> Vec<Token> {
    let lines = s.lines();
    let mut tokens: Vec<Token> = Vec::new();

    let mut commented: bool = false;

    for line in lines {
        if line.starts_with("/*") {
            commented = true;
        }
        if line.ends_with("*/") {
            commented = false;
        }

        if commented {
            continue;
        }

        let orig_words: Vec<&str> = line.split_whitespace().collect();
        
        let line = line.to_lowercase();

        let words = line
            .split_whitespace()
            .collect::<Vec<&str>>();
        let words = words.as_slice();

        if let ["create", "table", _, ..] = words {
            let table_name = orig_words[2].replace('"', "").to_string();
            
            tokens.push(Token::TableCreate(table_name));
        } else if let ["alter", "table", _, "add", "constraint", _] = words {
            let table_name = orig_words[2].replace('"', "").to_string();
            let foreign_key_name = orig_words[5].replace('"', "").to_string();

            tokens.push(Token::TableAlter {
                table_name,
                foreign_key_name
            });
        } else if let ["foreign", "key", _, "references", _, ..] = words {
            let field_name = orig_words[2].strip_prefix("(\"").unwrap().strip_suffix("\")").unwrap().to_string();
            let referenced_table = orig_words[4].replace('"', "").to_string();

            tokens.push(Token::ForeignKeyDetails { 
                field_name,
                referenced_table
            });
        } else if let ["constraint", ..] = words {

        } else if let ["create", "index", ..] = words {

        } else if let [_, _, ..] = words {
            let name = orig_words[0].replace('"', "").to_string();
            let ftype = orig_words[1].replace(|c| ['(', ')', ','].contains(&c), "").to_string();

            tokens.push(Token::Field { 
                name,
                ftype
            });
        } 

        if line.ends_with(';') {
            tokens.push(Token::ExprEnd);
        }
    }

    tokens
}

#[derive(Debug)]
enum Expression {
    CreateTable {
        table_name: String,
        fields: Vec<(String, String)>
    },
    ForeignKey {
        table_name: String,
        foreign_key_name: String,
        field_name: String,
        referenced_table: String
    }
}

fn parse_tokens(v: &Vec<Token>) -> Vec<Expression> {
    let mut exprs: Vec<Expression> = Vec::new();

    let expr_tokens: Vec<&[Token]> = v.split(|t| t == &Token::ExprEnd)
        .filter(|e| !e.is_empty())
        .collect();

    for e in expr_tokens {
        if let [Token::TableCreate(table_name), ..] = e {
            let mut fields: Vec<(String, String)> = Vec::new();

            for field in e[1..].iter() {
                if let Token::Field { name, ftype } = field {
                    fields.push((name.to_string(), ftype.to_string()));
                } else {
                    panic!("Invalid field {:?}", field)
                }
            }

            exprs.push(Expression::CreateTable {
                table_name: table_name.to_string(),
                fields
            });
        } else if let [Token::TableAlter { table_name, foreign_key_name }, Token::ForeignKeyDetails { field_name, referenced_table }] = e {
            exprs.push(Expression::ForeignKey {
                table_name: table_name.to_string(),
                foreign_key_name: foreign_key_name.to_string(),
                field_name: field_name.to_string(),
                referenced_table: referenced_table.to_string()
            })  
        } else {
            panic!("Invalid expression")
        }
    }

    exprs
}

fn exprs_to_mermaid(e: Vec<Expression>) -> String {
    let mut contents: String = String::new();
    contents.push_str("erDiagram\n");

    for expr in e {
        match expr {
            Expression::CreateTable { table_name, fields } => {
                contents.push_str(&format!("\t{} {{\n", table_name));

                for field in fields {
                    let (field_name, field_type) = field;
                    contents.push_str(&format!("\t\t{} {}\n", field_type, field_name));
                }

                contents.push_str("\t}\n\n");
            },
            Expression::ForeignKey { table_name, foreign_key_name, referenced_table, .. } => {
                contents.push_str(&format!("\t{} ||--|{{ {} : \"{}\"\n", table_name, referenced_table, foreign_key_name))
            }
        }
    }

    contents
}

