use std::path::PathBuf;

use crate::types::location::Location;

pub enum Documentation {
    ModuleDocumentation {
        location: Location,
        doc_text: String,
        module_doc: Vec<Documentation>,
    },
    FunctionDocument {
        location: Location,
        doc_text: String,
        function: String,
    },
    MethodDocumentation {
        location: Location,
        doc_text: String,
        method: String,
    },
    AttributeDocumentation {
        atributte: String,
        doc_text: String,
    },
    ClassDocumentation {
        location: Location,
        class_methods: Box<Documentation>,
        class_atrs: Box<Documentation>,
        doc_text: String,
        class: String,
    },
    TraitDocumentation {
        location: Location,
        trait_n: String,
        doc_text: String,
        class_methods: Box<Documentation>,
    },
    StructureDocumentation {
        location: Location,
        struct_name: String,
        doc_text: String,
        struct_fields: Vec<(String, String)>,
    },
}

pub struct Document {
    path: PathBuf,
    documentation: Vec<Documentation>,
}

impl Document {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            documentation: Vec::new(),
        }
    }
}
