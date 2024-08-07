use std::collections::VecDeque;

fn main(){
    let mut deque: VecDeque<i32> = VecDeque::new();
    //add elements to the back
    deque.push_back(1);
    deque.push_back(2);
    deque.push_back(3);

    // Add elements to the front

    deque.push_front(0);
    deque.push_front(-1);

    let back = deque.pop_back();
    println!("Popped from back: {:?}", back);

    let front = deque.pop_front();
    println!("Popped from front: {:?}", front);

    println!("Deque: {:?}", deque);
}