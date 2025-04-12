use std::rc::Rc;
use std::cell::RefCell;

pub struct UpdateHostsContentSubject {
    observers: Vec<Rc<RefCell<dyn Observer>>>,
}
 
impl UpdateHostsContentSubject {
    pub fn new() -> Self {
        Self { observers: Vec::new() }
    }
 
    pub fn register(&mut self, observer: Rc<RefCell<dyn Observer>>) {
        self.observers.push(observer); 
    }
 
    pub fn notify(&self, id: &str) {
        for observer in &self.observers  {
            observer.borrow_mut().update(id); 
        }
    }
}

pub trait Observer {
    fn update(&mut self, id: &str);
}

#[cfg(test)]
mod tests {
    use crate::editor::Editor;

    use super::*;

    struct Dummy;

    #[test]
    pub fn test_pub_sub() {
        let mut subject = UpdateHostsContentSubject::new();
        let a = Rc::new(RefCell::new(Editor::new()));
        subject.register(a);
        subject.notify("Button  clicked");
    }
}