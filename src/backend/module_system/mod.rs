use std::collections::{HashMap};
use std::fs::{File};
use std::io::Read;
use std::path::{Path, PathBuf};
use crate::backend::config::Config;
use crate::backend::error_parser::Error;
use crate::backend::lexer::tokenizer;

use crate::backend::nodes::{Expression, Statement};
use crate::backend::parser::Parser;
use crate::backend::tokens::Primary;

pub struct Node {
    pub path: String,
    pub ast: Vec<Statement>,
    pub deps: Vec<usize>,
}

pub struct ModuleGraph {
    pub nodes: Vec<Node>,
    index: HashMap<PathBuf, usize>,
    pub order: Vec<usize>,
}

impl ModuleGraph {
    pub fn new(config: Config) -> Self {
        let content = config.run();
        let ast = Self::parse_module_code(content);
        let abs = Path::new(&config.file_path).canonicalize().unwrap();

        let first_node = Node {
            path: abs.to_string_lossy().into_owned(),
            ast,
            deps: vec![],
        };

        let mut index = HashMap::new();
        index.insert(abs, 0);

        Self { nodes: vec![first_node], index, order: vec![] }
    }

    pub fn scan_imports(&self, ast: &Vec<Statement>) -> Vec<String> {
        let mut paths: Vec<String> = vec![];
        for statement in ast {
            match statement {
                Statement::Import { path, .. } => {
                    match self.import_path(path) {
                        Ok(v) => {paths.push(v)},
                        Err(_) => {}
                    }

                },
                _ => {}
            }
        }

        paths
    }



    pub fn parse_module_code(content: String) -> Vec<Statement> {
        let Ok(lexer_output) = tokenizer(&content) else {
            eprintln!("Lexer error: Couldn't parse the code");
            std::process::exit(1);
        };


        let mut parser = Parser::new(lexer_output);
        let ast = match parser.parse_code() {
            Ok(ast) => ast,
            Err(e) => {
                eprintln!("{}", e);
                std::process::exit(1);
            }
        };

        ast
    }



    fn import_path(&self, expr: &Expression) -> Result<String, String> {
        match expr {
            Expression::Literal(Primary::Str(s)) => Ok(s.clone()),
            _ => Err("import path must be a string literal".to_string()),
        }
    }



    fn resolve_import(&self, importer: &Path, import: &str) -> std::io::Result<PathBuf> {
        let dir = importer.parent().unwrap_or(Path::new("."));
        dir.join(import).canonicalize()
    }


    pub fn read_file_values(&self, file_path: &Path) -> String {
        let mut file = match File::open(file_path) {
            Ok(file_data)=> file_data,
            Err(error) => panic!("Problem opening the file: {error:?}"),
        };

        let mut contents = String::new();
        match file.read_to_string(&mut contents) {
            Ok(content) => content,
            Err(error) => panic!("Couldn't Read File Content: {error:?}")
        };


        return contents
    }
    pub fn walk_modules(&mut self) -> Result<(), Error> {
        let mut order: Vec<usize> = vec![];
        let mut stack: Vec<usize> = vec![];
        self.dfs(0, &mut order, &mut stack)?;
        self.order = order;
        Ok(())
    }

    pub fn dfs(
        &mut self,
        index: usize,
        order: &mut Vec<usize>,
        stack: &mut Vec<usize>,
    ) -> Result<(), Error> {
        stack.push(index);

        let imports = self.scan_imports(&self.nodes[index].ast);
        let current_path = PathBuf::from(&self.nodes[index].path);

        for import in imports {
            let import_abs = self
                .resolve_import(&current_path, &import)
                .map_err(|e| Error::RuntimeError { message:  format!("Couldn't resolve import \"{}\"  in {}: \n \t \t{}", import, self.nodes[index].path, e)})?;

            let existing = self.index.get(&import_abs).copied();

            let dep_index = match existing {
                Some(i) => {
                    if let Some(pos) = stack.iter().position(|&s| s == i) {
                        let chain: Vec<&str> = stack[pos..]
                            .iter()
                            .chain(std::iter::once(&i))
                            .map(|&n| self.nodes[n].path.as_str())
                            .collect();
                        return Err(Error::RuntimeError { message: format!("Circular import:\n {} \n", chain.join(" -> ")) });
                    }
                    i
                }
                None => {
                    let content = self.read_file_values(&import_abs);
                    let ast = Self::parse_module_code(content);

                    let new_index = self.nodes.len();
                    self.nodes.push(Node {
                        path: import_abs.to_string_lossy().into_owned(),
                        ast,
                        deps: vec![],
                    });
                    self.index.insert(import_abs, new_index);

                    self.dfs(new_index, order, stack)?;
                    new_index
                }
            };

            self.nodes[index].deps.push(dep_index);
        }

        stack.pop();
        order.push(index);
        Ok(())
    }
}
