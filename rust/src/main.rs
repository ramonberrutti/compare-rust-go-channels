use std::sync::mpsc;
use std::thread;

fn sum(s: &[i32], c: mpsc::Sender<i32>) {
    let sum = s.into_iter().sum();
    c.send(sum).unwrap();
}

fn main() {
    let s = &[7, 2, 8, -9, 4, 0];

    let (sender, receiver) = mpsc::channel();

    let (left, right) = s.split_at(s.len() / 2);

    let tx1 = mpsc::Sender::clone(&sender);
    thread::spawn(move || {
        sum(left, tx1);
    });

    thread::spawn(move || {
        sum(right, sender);
    });

    let x = receiver.recv().unwrap();
    let y = receiver.recv().unwrap();

    println!("{} + {} = {}", x, y, x + y);
}
