use std::{cell::RefCell, collections::HashMap, rc::Rc};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Trace {
    pub id: Option<String>,
    pub app_key: String,
    pub span_name: String,
    pub service: Option<String>,
    pub attributes: HashMap<String, String>,
}

#[derive(Debug)]
pub struct List {
    pub pathtraces: Vec<Pathtrace>,
}

#[derive(Debug)]
pub struct Pathtrace {
    head: Option<Rc<RefCell<Node>>>,
}

#[derive(Debug)]
struct Node {
    value: Trace,
    next: Option<Rc<RefCell<Node>>>,
}

impl Pathtrace {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn push(&mut self, value: Trace) {
        let node = Rc::new(RefCell::new(Node::new(value)));

        match &self.head {
            Some(head) => head.borrow_mut().append(node),
            None => self.head = Some(node),
        }
    }
}

impl Node {
    pub fn new(value: Trace) -> Self {
        Node { value, next: None }
    }

    pub fn append(&mut self, value: Rc<RefCell<Node>>) {
        self.next = Some(value);
    }
}

#[cfg(test)]

mod tests {
    use std::collections::HashMap;

    use super::{Pathtrace, Trace};

    #[test]
    fn should_create_pathtraces() {
        let trace = Trace {
            id: None,
            app_key: "testing".to_string(),
            span_name: "test-span".to_string(),
            service: Some("delta".to_string()),
            attributes: HashMap::new(),
        };
        let trace_delta = Trace {
            id: None,
            app_key: "testing".to_string(),
            span_name: "test-span-delta".to_string(),
            service: None,
            attributes: HashMap::new(),
        };
        let mut pathtrace = Pathtrace::new();
        pathtrace.push(trace);
        pathtrace.push(trace_delta);

        assert!(pathtrace.head.is_some())
    }
}
