use crate::parser::ASTNode;
use std::collections::HashMap;
use std::process::exit;

// Semantic Analyzer trait
pub trait SemanticAnalyzer {
    fn analyze(&mut self);
    fn check_variables(&mut self);
}

// scope level with its own symbol table
#[derive(Debug, Clone)]
struct Scope {
    variables: HashMap<String, Option<String>>, // variable -> value 
}

impl Scope {
    fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }
}

// Concrete semantic analyzer implementation with scope support
pub struct LolcodeSemanticAnalyzer {
    // Stack of scopes:local scopes at top, then global
    scope_stack: Vec<Scope>,
    // see if currently inside of variable assignment
    current_assignment: Option<String>,
    // tracks errors on vector
    errors: Vec<String>,
}

impl LolcodeSemanticAnalyzer {
    pub fn new() -> Self {
        Self {
            scope_stack: vec![Scope::new()], // Start with global scope
            current_assignment: None,
            errors: Vec::new(),
        }
    }

    // handles semantic error reporting
    fn semantic_error(&mut self, msg: String) {
        self.errors.push(msg);
    }

    // new scope (push onto stack)
    fn enter_scope(&mut self) {
        self.scope_stack.push(Scope::new());
    }

    // exit current scope (pop)
    fn exit_scope(&mut self) {
        if self.scope_stack.len() > 1 {
            self.scope_stack.pop();
        }
    }

    // get current scope (top of stack)
    fn current_scope(&mut self) -> &mut Scope {
        self.scope_stack.last_mut().unwrap()
    }

    // look for variable in current scope
    fn lookup_variable(&self, name: &str) -> Option<Option<String>> {
        // Search closest to furthest
        for scope in self.scope_stack.iter().rev() {
            if let Some(value) = scope.variables.get(name) {
                return Some(value.clone());
            }
        }
        None
    }

    // Declare a variable in current scope
    fn declare_variable(&mut self, name: String) {
        let scope = self.current_scope();
        
        // Check if variable already exists in current scope 
        if scope.variables.contains_key(&name) {
            self.semantic_error(format!(
                "Variable '{}' is already declared in this scope",
                name
            ));
        } else {
            scope.variables.insert(name, None); // None = declared but not assigned
        }
    }

    // Declare a variable in current scope 
    fn declare_variable_codegen(&mut self, name: String) {
        let scope = self.current_scope();
        scope.variables.insert(name, None);
    }

    // Assign value to a variable
    fn assign_variable(&mut self, name: &str, value: String) {
        // Find the variable in current or parent scopes and assign the value
        for scope in self.scope_stack.iter_mut().rev() {
            if scope.variables.contains_key(name) {
                scope.variables.insert(name.to_string(), Some(value));
                return;
            }
        }
        //Error if not found
        self.semantic_error(format!("Cannot assign to undeclared variable '{}'", name));
    }

    // Traverse the parse tree and check for semantic errors
    fn traverse(&mut self, node: &ASTNode) {
        match node {
            ASTNode::Program { children } => {
                // top level first
                for child in children {
                    self.traverse(child);
                }
            }

            ASTNode::HeadSection { children } => {
                // Head sections don't create new scope
                for child in children {
                    self.traverse(child);
                }
            }

            ASTNode::ParagrafSection { children } => {
                // Enter new scope for paragraf section
                self.enter_scope();
                for child in children {
                    self.traverse(child);
                }
                self.exit_scope();
            }

            ASTNode::ListSection { children } => {
                // Enter new scope for list section
                self.enter_scope();
                for child in children {
                    self.traverse(child);
                }
                self.exit_scope();
            }

            // Variable declaration: #I HAZ varname
            ASTNode::VariableDeclaration { name } => {
                self.declare_variable(name.clone());
                self.current_assignment = Some(name.clone());
            }

            // Variable assignment: #IT IZ value #MKAY
            ASTNode::VariableAssignment { name: _, value } => {
                // Mark the most recently declared variable as assigned with its value
                if let Some(var_name) = self.current_assignment.clone() {
                    self.assign_variable(&var_name, value.clone());
                    self.current_assignment = None;
                }
            }

            // Variable reference: #LEMME SEE varname #MKAY
            ASTNode::VariableReference { name } => {
                match self.lookup_variable(name) {
                    None => {
                        self.semantic_error(format!(
                            "Variable '{}' is used but never declared",
                            name
                        ));
                    }
                    Some(None) => {
                        self.semantic_error(format!(
                            "Variable '{}' is used but never assigned a value",
                            name
                        ));
                    }
                    Some(Some(_)) => {
                        // Variable is declared and assigned
                    }
                }
            }

            // content in bold/italic
            ASTNode::Bold { content } => {
                for child in content {
                    self.traverse(child);
                }
            }

            ASTNode::Italics { content } => {
                for child in content {
                    self.traverse(child);
                }
            }

            // nothing in leaf nodes
            ASTNode::Title { .. } => {}
            ASTNode::Text { .. } => {}
            ASTNode::Item { .. } => {}
            ASTNode::Newline => {}
            ASTNode::Sound { .. } => {}
            ASTNode::Video { .. } => {}
        }
    }

    /// Print all semantic errors and exit if any found
    fn report_errors(&self) {
        if !self.errors.is_empty() {
            eprintln!("=== Semantic Analysis Errors ===");
            for error in &self.errors {
                eprintln!("Semantic error: {}", error);
            }
            eprintln!("================================");
            exit(1);
        }
    }
}

impl SemanticAnalyzer for LolcodeSemanticAnalyzer {
    fn analyze(&mut self) {
        println!("Starting semantic analysis...");
    }

    fn check_variables(&mut self) {
        // Variable checking during traversal
    }
}

impl LolcodeSemanticAnalyzer {
    // analyze parse tree
    pub fn analyze_tree(&mut self, tree: &ASTNode, input_filename: &str) {
        println!("Starting semantic analysis...");
        
        //Traverse tree and check semantics
        self.traverse(tree);
        
        // Report any errors found
        self.report_errors();
        
        println!("Semantic analysis completed successfully!");
        
        // Task 4: Generate HTML code
        println!("Generating HTML output...");
        
        // Reset scopes for HTML generation traversal
        self.scope_stack = vec![Scope::new()];
        self.current_assignment = None;
        
        // Re-traverse to generate HTML (this time populating scopes with values)
        let html = self.generate_html_with_traversal(tree);
        
        // Write to output file
        let output_filename = self.write_html_file(&html, input_filename);
        
        println!("HTML generated successfully: {}", output_filename);
        
        // Open in browser
        self.open_in_browser(&output_filename);
    }

    // Generate HTML by re-traversing the tree and maintaining scope
    fn generate_html_with_traversal(&mut self, node: &ASTNode) -> String {
        match node {
            ASTNode::Program { children } => {
                let mut body_content = String::new();
                for child in children {
                    body_content.push_str(&self.generate_html_with_traversal(child));
                }
                
                format!(
                    "<!DOCTYPE html>\n<html>\n<head>\n<meta charset=\"UTF-8\">\n<title>LOLCODE Markdown</title>\n</head>\n<body>\n{}</body>\n</html>",
                    body_content
                )
            }

            ASTNode::HeadSection { children } => {
                let mut content = String::new();
                for child in children {
                    content.push_str(&self.generate_html_with_traversal(child));
                }
                content
            }

            ASTNode::ParagrafSection { children } => {
                self.enter_scope();
                
                let mut content = String::new();
                for child in children {
                    content.push_str(&self.generate_html_with_traversal(child));
                }
                
                self.exit_scope();
                
                format!("<p>\n{}</p>\n", content)
            }

            ASTNode::ListSection { children } => {
                self.enter_scope();
                
                let mut items = String::new();
                for child in children {
                    items.push_str(&self.generate_html_with_traversal(child));
                }
                
                self.exit_scope();
                
                format!("<ul>\n{}</ul>\n", items)
            }

            ASTNode::Title { content } => {
                format!("<h1>{}</h1>\n", content)
            }

            ASTNode::Text { content } => {
                format!("{} ", content)
            }

            ASTNode::Bold { content } => {
                let mut inner = String::new();
                for child in content {
                    inner.push_str(&self.generate_html_with_traversal(child));
                }
                format!("<b>{}</b>", inner)
            }

            ASTNode::Italics { content } => {
                let mut inner = String::new();
                for child in content {
                    inner.push_str(&self.generate_html_with_traversal(child));
                }
                format!("<i>{}</i>", inner)
            }

            ASTNode::Item { content } => {
                format!("<li>{}</li>\n", content)
            }

            ASTNode::Newline => {
                "<br>\n".to_string()
            }

            ASTNode::Sound { url } => {
                format!("<audio controls src=\"{}\"></audio>\n", url)
            }

            ASTNode::Video { url } => {
                format!("<video controls src=\"{}\"></video>\n", url)
            }

            ASTNode::VariableDeclaration { name } => {
                self.current_assignment = Some(name.clone());
                self.declare_variable_codegen(name.clone());
                String::new()
            }
            
            ASTNode::VariableAssignment { value, .. } => {
                if let Some(var_name) = self.current_assignment.clone() {
                    self.assign_variable(&var_name, value.clone());
                    self.current_assignment = None;
                }
                String::new()
            }
            
            ASTNode::VariableReference { name } => {
                match self.lookup_variable(name) {
                    Some(Some(value)) => value,
                    _ => format!("[undefined: {}]", name)
                }
            }
        }
    }

    // Write HTML to output file
    fn write_html_file(&self, html: &str, input_filename: &str) -> String {
        use std::fs;
        use std::path::Path;
        
        // Create output filename by replacing .lol with .html
        let path = Path::new(input_filename);
        let output_filename = path.with_extension("html");
        
        // Write HTML to file
        fs::write(&output_filename, html).unwrap_or_else(|e| {
            eprintln!("Failed to write HTML file: {}", e);
            exit(1);
        });
        
        output_filename.to_string_lossy().to_string()
    }

    // Open HTML file in browser
    fn open_in_browser(&self, filename: &str) {
        use std::process::Command;
        use std::path::Path;
        use std::env;
        
        // Get absolute path
        let path = Path::new(filename);
        let absolute_path = if path.is_absolute() {
            path.to_path_buf()
        } else {
            env::current_dir()
                .unwrap_or_else(|_| Path::new(".").to_path_buf())
                .join(path)
        };
        
        let path_str = absolute_path.to_string_lossy().to_string();
        
        // using windows OS and chrome to open
        #[cfg(target_os = "windows")]
{
    let windows_path = path_str.replace("/", "\\");
    
    // Try Chrome first
    let chrome_result = Command::new("chrome")
        .arg(&windows_path)
        .spawn();
    
    if chrome_result.is_err() {
        // Fallback to default browser
        let _ = Command::new("cmd")
            .args(&["/C", "start", "", &windows_path])
            .spawn();
    }
}
    
        
    }

    /// Get the current scope's symbol table (useful for debugging)
    pub fn get_current_scope(&self) -> &HashMap<String, Option<String>> {
        &self.scope_stack.last().unwrap().variables
    }
}