use term;
use uil_parsers;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct ColoredErrorReporter {
    t: Rc<RefCell<Box<term::StdoutTerminal>>>,
}

impl ColoredErrorReporter {
    pub fn new() -> ColoredErrorReporter {
        ColoredErrorReporter {
            t: Rc::new(RefCell::new(term::stdout().unwrap())),
        }
    }
}

impl uil_parsers::ErrorReporter for ColoredErrorReporter {

    fn log(&self, msg: String) {

        let mut test = match msg.find("Error") {
            Some(i) => {
                self.t.borrow_mut().fg(term::color::RED).unwrap();
                (write!(self.t.borrow_mut(), "Error")).unwrap();
                self.t.borrow_mut().reset().unwrap();
                self.t.borrow_mut().attr(term::Attr::Bold).unwrap();
                (writeln!(self.t.borrow_mut(), "{}", &msg[(i+"Error".len())..])).unwrap();
                true
            }
            None => false,
        };

        if !test {
            test = match msg.find("Warning") {
                Some(i) => {
                    self.t.borrow_mut().fg(term::color::YELLOW).unwrap();
                    (write!(self.t.borrow_mut(), "Warning")).unwrap();
                    self.t.borrow_mut().reset().unwrap();
                    self.t.borrow_mut().attr(term::Attr::Bold).unwrap();
                    (writeln!(self.t.borrow_mut(), "{}", &msg[(i+"Warning".len())..])).unwrap();
                    true
                }
                None => false,
            }
        }

        if !test {
            self.t.borrow_mut().fg(term::color::BLUE).unwrap();
            (write!(self.t.borrow_mut(), "Info: ")).unwrap();
            self.t.borrow_mut().reset().unwrap();
            self.t.borrow_mut().attr(term::Attr::Bold).unwrap();
            (writeln!(self.t.borrow_mut(), "{}", msg)).unwrap();
        }

        self.t.borrow_mut().reset().unwrap();
    }
}
