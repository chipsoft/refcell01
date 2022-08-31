use std::cell::RefCell;

struct Uart {
    counter: u32
}

impl Uart {
    pub fn new() -> Self {
        Uart {counter: 0}
    }

    pub fn print_counter(&self) {
        println!("{}", self.counter)
    }

    pub fn inc_counter(&mut self) {
        self.counter += 1;
    }
}

struct Tx<'a> {
    hw: &'a RefCell<Uart>,
}

impl<'a> Tx<'a> {
    pub fn new(hw: &'a RefCell<Uart>) -> Self {
        Tx {hw}
    }

    pub fn print_counter(&self) {
        self.hw.borrow().print_counter();
    }

    pub fn inc_counter(&self) {
        self.hw.borrow_mut().inc_counter();
    }

}

struct FastInit<'a> {
    hw: &'a RefCell<Uart>,
}

impl<'a> FastInit<'a> {
    pub fn new(hw: &'a RefCell<Uart>) -> Self {
        FastInit {hw}
    }

    pub fn print_counter(&self) {
        self.hw.borrow().print_counter();
    }

    pub fn inc_counter(&self) {
        self.hw.borrow_mut().inc_counter();
    }

}

fn main() {
    let u = RefCell::new(Uart::new());
    let fast_init0 = FastInit::new(&u);
    let tx0 = Tx::new(&u);
    fast_init0.print_counter();
    tx0.print_counter();
    fast_init0.inc_counter();
    tx0.inc_counter();
    println!("After increment");
    fast_init0.print_counter();
    tx0.print_counter();
}
