use enum_iterator::all;

enum DummyEnum {
    One,
    Two,
    Three,
    Four,
}

fn main() {
    let dummy_variable = all::<DummyEnum>().collect();    
}
