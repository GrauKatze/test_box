#![warn(missing_docs)]
//! Утилита для тестирования работы приложений на предмет соответствия их выходных данных
//! Example
//! ```bash
//!     test_box -p path/to/app
//! ```
mod test_box;

fn main() {
    test_box::TestBox::init(std::env::args().skip(1).collect());
}
