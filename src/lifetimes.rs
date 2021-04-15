use std::ops::Deref;

// Ownership -> References -> Lifetimes

// Types of lifetimes to cover:
//
// Functions
// Methods
// Structs
// Traits
// static

fn func() {
    let var = String::from("hello world");
    takes_string(&var, "hello");
    println!("{}", var);

    let data = SomeData {
        value: &var,
        label: "test"
    };

    takes_data(&data);
    println!("{:?}", data);
}

fn takes_string<'a>(v: &'a str, x: &'a str) -> &'a str {
    println!("{} {}", v, x);
    v
}

fn takes_data(data: &SomeData) {
    println!("{:?}", data);
}

#[derive(Debug)]
struct SomeData<'a, 'b> {
    value: &'a str,
    label: &'b str
}

#[derive(Debug)]
pub struct DataStore {
    // HashMap
}

#[derive(Debug)]
pub struct Renderer {
    // Shader pipelines
}

#[derive(Debug)]
pub struct Context<'a> {
    state: &'a str,
    data_store: &'a DataStore,
    renderer: &'a Renderer, 
}

pub fn tick(context: &Context) {
    println!("state = {}", context.state);
}

pub fn build_context<'a>(renderer :&'a Renderer, data_store: &'a DataStore) -> Context<'a> {
    // if created here we cannot pass them out due to ownership being within
    // scope of this function. 
    // let data_store = DataStore { };
    // let renderer = Renderer { };

    println!("Lifetimes are fun!");
    func();

    let context = Context {
        state: "loading",
        data_store: data_store,
        renderer: renderer
    };

    context
}

pub trait StateMachine {
    fn get_current_state(&self) -> &str;

    // just function
    fn push_state<'b>(&self, new_state: Box<&'b String>) -> &'b str; // <- Missed this last one
}

impl<'a> StateMachine for Context<'a> {
    fn get_current_state(&self) -> &str {
        self.state
    }

    fn push_state<'b>(&self, new_state: Box<&'b String>) -> &'b str {
        new_state.deref()
    }
}