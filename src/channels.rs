use std::sync::mpsc::{channel, Sender, Receiver};

#[derive(Debug)]
struct FooObj {
    a: u32, b: u32
}

pub fn channels_main() {
    let (tx, rx): (Sender<FooObj>, Receiver<FooObj>) = channel();

    let foo = FooObj {a: 1, b: 2};

    tx.send(foo).unwrap();
    let bar = rx.recv().unwrap();

    // println!("{:?}", foo); <- this fails, because borrow checker :)

    println!("{:?}", bar);

    // let baz = rx.try_recv().unwrap(); // <- this fails (empty queue)
}
