use wasip1_virtual_layer::*;

wit_bindgen::generate!({
    // the name of the world in the `*.wit` input file
    world: "hello",
});

struct Hello;

impl Guest for Hello {
    fn world() {
        println!("Hello, world!");
    }
}

export!(Hello);
