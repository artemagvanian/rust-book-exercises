//! P2: Terminal user interface
//! 
//! This problem explores the differences between designing systems with
//! classes and traits. The adjacent file `tui.cpp` provides a C++ implementation
//! of a terminal user inteface (TUI), i.e. a simple set of graphical elements that
//! can be drawn into the terminal. The C++ code uses classes, inheritance, and
//! virtual methods.
//! 
//! To see the C++ code in action, you can build and execute the code by running:
//! 
//! ```bash
//! ./run.cpp-sh
//! ```
//! 
//! Your task is to redesign the C++ TUI API into Rust. Your API should similarly
//! contain data structures that represent Text, Heading, and Container. You should
//! replicate the behavior of `main` in `tui.cpp` into the `container_test` function.

use std::cmp::max;

struct Dimensions {
    width: usize,
    height: usize
}

struct Text {
    text: String,
}

trait Element {
    fn dimensions(&self) -> Dimensions;
    fn render(&self);
}

impl Element for Text {
    fn dimensions(&self) -> Dimensions {
        Dimensions { width: self.text.len(), height: 1 }
    }

    fn render(&self) {
        print!("{}", self.text);
    }
}

struct Heading {
    inner_text: Text
}

impl Element for Heading {
    fn dimensions(&self) -> Dimensions {
        self.inner_text.dimensions()
    }

    fn render(&self) {
        print!("{}", self.inner_text.text);
    }
}

struct Container {
    children: Vec<Box<dyn Element>>,
}

impl Container {
    fn dimensions(&self) -> Dimensions {
        let mut max_width: usize = 0;
        let mut sum_height: usize = 0;
        for child in &self.children {
            let dims: Dimensions = child.dimensions();
            max_width = max(max_width, dims.width);
            sum_height += dims.height;
        }
        return Dimensions{ width: max_width + 2, height: sum_height};
    }

    fn render(&self) {
        let dims = self.dimensions();
        let render_line = || {
            print!("+");
            for _ in 0..dims.width - 2 {
                print!("-");
            }
            println!("+");
        };
        render_line();

        for child in &self.children {
            let child_dims = child.dimensions();
            print!("|");
            child.render();
            for _ in 0..dims.width - 2 - child_dims.width {
                print!(" ");
            }
            println!("|");
        }
        render_line();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn container_test() {
        let text = Heading { inner_text: Text { text: String::from("Hello world") }};
        let text2 = Text { text: String::from("This is a long string of text") };
        let mut children: Vec<Box<dyn Element>> = vec![];

        children.push(Box::new(text));
        children.push(Box::new(text2));
        let container = Container { children };

        container.render();
    }
}
