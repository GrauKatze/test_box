mod test_box;

fn main() {
    test_box::init(std::env::args().skip(1).collect());
}
