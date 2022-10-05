use std::{path::PathBuf, fs::{read_to_string, self}};
use serde::Serialize;
use tinytemplate::{TinyTemplate, format_unescaped};

pub fn build_site_at_path(path: PathBuf) -> String {
    let contents = read_to_string(path.clone()).expect("Given file could not be read!");

    let title = path.to_str().expect("The filename is non-UTF-8!");

    build_site(contents.as_str(), title)
}

pub fn build_site(md: &str, title: &str) -> String {
    let ast = build_ast(md, title);

    build_site_from_ast(ast)
}

pub fn build_ast(md: &str, title: &str) -> AST {
    let mut result = AST::new(title.to_string());

    for line in md.lines() {
        let line = line.trim();

        if line.len() >= 1 {
            result.push_element(match &line[..1] {
                "*" => {
                    ASTElement::Header(line[2..].to_string())
                },
                "+" => {
                    ASTElement::ListElement(line[2..].to_string())
                },
                _ => {
                    ASTElement::Paragraph(line.to_string())
                },
            });
        } else {
            if !line.is_empty() {
                result.push_element(ASTElement::Paragraph(line.to_string()));
            }
        }
    }

    result
}

#[derive(Serialize)]
struct SiteTemplateContext {
    title: String,
    content: String,
}

pub fn build_site_from_ast(ast: AST) -> String {
    let mut tt = TinyTemplate::new();

    tt.set_default_formatter(&format_unescaped);

    let template = fs::read_to_string("template.html").expect("Couldn't find the 'template.html' file!");

    tt.add_template(
        "site",
        template.as_str(),
    ).expect("The 'template.html' file is ill-formed!");

    let title = ast.title.clone();

    let content_tree = build_content_tree_from_ast(ast);
    let content = build_site_content_from_content_tree(content_tree);

    let context = SiteTemplateContext {
        title,
        content,
    };

    tt.render("site", &context).expect("Couldn't resolve the template!")
}

pub fn build_content_tree_from_ast(ast: AST) -> ContentTree {
    let mut result = ContentTree::new();

    let mut current_list = None;

    for element in ast.elements {
        if let ASTElement::ListElement(_) = element {} else if let Some(list) = current_list.take() {
            result.push_element(ContentTreeElement::List(list));
        }

        match element {
            ASTElement::Header(content) => {
                result.push_element(ContentTreeElement::Header(content));
            },
            ASTElement::Paragraph(content) => {
                result.push_element(ContentTreeElement::Paragraph(content));
            },
            ASTElement::ListElement(content) => {
                if let Some(mut list) = current_list.take() {
                    list.push(content);
                    current_list = Some(list);
                } else {
                    current_list = Some(vec![content]);
                }
            },
        }
    }

    result
}

pub fn build_site_content_from_content_tree(content: ContentTree) -> String {
    let mut result = String::new();

    for element in content.elements {
        let element_html = match element {
            ContentTreeElement::Header(content) => {
                format!("<p>{}</p>\n", content)
            },
            ContentTreeElement::Paragraph(content) => {
                format!("{}\n", content)
            },
            ContentTreeElement::List(list_elements) => {
                let mut list = String::new();

                for list_element in list_elements {
                    list += format!("\t<li>{}</li>\n", list_element).as_str();
                }

                format!("<ul>\n{}</ul>\n", list)
            },
        };

        result += element_html.as_str();
    }

    result
}

pub struct AST {
    pub title: String,
    pub elements: Vec<ASTElement>,
}

impl AST {
    pub fn new(title: String) -> Self {
        Self {
            title,
            elements: Vec::new(),
        }
    }

    pub fn push_element(&mut self, element: ASTElement) {
        self.elements.push(element);
    }

    pub fn pop_element(&mut self) -> Option<ASTElement> {
        self.elements.pop()
    }
}

pub enum ASTElement {
    Header(String),
    Paragraph(String),
    ListElement(String),
}

pub struct ContentTree {
    elements: Vec<ContentTreeElement>,
}

impl ContentTree {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
        }
    }

    pub fn push_element(&mut self, element: ContentTreeElement) {
        self.elements.push(element);
    }

    pub fn pop_element(&mut self) -> Option<ContentTreeElement> {
        self.elements.pop()
    }
}

pub enum ContentTreeElement {
    Header(String),
    Paragraph(String),
    List(Vec<String>),
}