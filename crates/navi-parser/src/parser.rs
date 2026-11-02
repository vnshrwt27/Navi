
use std::{
    error::Error,
    fs,
    path::Path,
};
use streaming_iterator::StreamingIterator;
use tree_sitter::{
    Language as TsLanguage,
    Parser,
    Query,
    QueryCursor,
};

use navi_core::models::{
    file::Language,
    symbol::SymbolKind,
};



#[derive(Debug)]
pub struct ParsedSymbol {
    pub name: String,
    pub kind: SymbolKind,
    pub start: usize,
    pub end: usize,
}

#[derive(Debug)]
pub struct ParsedFile {
    pub symbols: Vec<ParsedSymbol>,
    pub calls: Vec<String>,
    pub imports: Vec<String>,
}

const RUST_SRC: &str = r#"
(function_item name: (identifier) @def.function)
(struct_item name: (type_identifier) @def.struct)
(enum_item name: (type_identifier) @def.enum)
(trait_item name: (type_identifier) @def.trait)
(call_expression function: (identifier) @ref.call)
(use_declaration argument: (_) @imp)
"#;

const PYTHON_SRC: &str = r#"
(function_definition name: (identifier) @def.function)
(class_definition name: (identifier) @def.class)
(call function: (identifier) @ref.call)
(import_statement) @imp
(import_from_statement) @imp
"#;

const JAVA_SRC: &str = r#"
(method_declaration name: (identifier) @def.method)
(class_declaration name: (identifier) @def.class)
(method_invocation name: (identifier) @ref.call)
(import_declaration) @imp
"#;

fn get_grammar(language: &Language) -> Result<TsLanguage, Box<dyn Error>> {
    let grammar = match language {
        Language::Rust => tree_sitter_rust::LANGUAGE.into(),
        Language::Python => tree_sitter_python::LANGUAGE.into(),
        Language::Java => tree_sitter_java::LANGUAGE.into(),
        _ => return Err("Unsupported language".into()),
    };

    Ok(grammar)
}

fn get_query(language: &Language) -> Result<&'static str, Box<dyn Error>> {
    let query = match language {
        Language::Rust => RUST_SRC,
        Language::Python => PYTHON_SRC,
        Language::Java => JAVA_SRC,
        _ => return Err("Unsupported language".into()),
    };

    Ok(query)
}

pub fn extract_entities(
    source_file: &Path,
    language: &Language,
) -> Result<ParsedFile, Box<dyn Error>> {

    // 1. Select Tree-sitter grammar
    let grammar = get_grammar(language)?;

    // 2. Select language-specific query
    let query_source = get_query(language)?;

    // 3. Read source file
    let source = fs::read_to_string(source_file)?;

    // 4. Initialize parser
    let mut parser = Parser::new();

    parser.set_language(&grammar)?;

    // 5. Parse source into AST
    let tree = parser
        .parse(&source, None)
        .ok_or("Failed to parse source")?;

    let root_node = tree.root_node();

    // 6. Compile query
    let query = Query::new(&grammar, query_source)?;

    // 7. Execute query against AST
    let mut cursor = QueryCursor::new();

    let mut parsed_file = ParsedFile {
        symbols: Vec::new(),
        calls: Vec::new(),
        imports: Vec::new(),
    };

    let mut matches = cursor.matches(
    &query,
    root_node,
    source.as_bytes(),
);

while let Some(query_match) = matches.next(){
        for capture in query_match.captures {
            let node = capture.node;

            let capture_name =
                query.capture_names()[capture.index as usize];

            let text = node
                .utf8_text(source.as_bytes())?
                .to_string();

            // 8. Convert Tree-sitter captures into Navi IR
            match capture_name {

                "def.function" | "def.method" => {
                    parsed_file.symbols.push(ParsedSymbol {
                        name: text,
                        kind: SymbolKind::Function,
                        start: node.start_byte(),
                        end: node.end_byte(),
                    });
                }

                "def.struct" => {
                    parsed_file.symbols.push(ParsedSymbol {
                        name: text,
                        kind: SymbolKind::Struct,
                        start: node.start_byte(),
                        end: node.end_byte(),
                    });
                }

                "def.enum" => {
                    parsed_file.symbols.push(ParsedSymbol {
                        name: text,
                        kind: SymbolKind::Enum,
                        start: node.start_byte(),
                        end: node.end_byte(),
                    });
                }

                "def.trait" => {
                    parsed_file.symbols.push(ParsedSymbol {
                        name: text,
                        kind: SymbolKind::Trait,
                        start: node.start_byte(),
                        end: node.end_byte(),
                    });
                }

                "def.class" => {
                    parsed_file.symbols.push(ParsedSymbol {
                        name: text,
                        kind: SymbolKind::Class,
                        start: node.start_byte(),
                        end: node.end_byte(),
                    });
                }

                "ref.call" => {
                    parsed_file.calls.push(text);
                }

                "imp" => {
                    parsed_file.imports.push(text);
                }

                _ => {}
            }
        }
    }

    Ok(parsed_file)
}
