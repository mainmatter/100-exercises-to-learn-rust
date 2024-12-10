// Scoped threads

/*
    All the lifetime issues we discussed so far have a common source: the spawned thread can outlive
    its parent. We can sidestep this issue by using scoped threads.

    let v = vec![1, 2, 3];
    let midpoint = v.len() / 2;

    std::thread::scope(|scope| {
        scope.spawn(|| {
            let first = &v[..midpoint];
            println!("Here's the first half of v: {first:?}");
        });
        scope.spawn(|| {
            let second = &v[midpoint..];
            println!("Here's the second half of v: {second:?}");
        });
    });

    println!("Here's v: {v:?}");

    scope
    The std::thread::scope function creates a new scope. std::thread::scope takes a closure as
    input, with a single argument: a Scope instance. 

    Scoped spawns
    Scope exposes a spawn method.
    Unlike std::thread::spawn, all threads spawned using Scope will be automatically joined when
    the scope ends.


*/

// TODO: Given a vector of integers, split it in two halves
//  and compute the sum of each half in a separate thread.
//  Don't perform any heap allocation. Don't leak any memory.

pub fn sum(v: Vec<i32>) -> i32 {
    let mid = v.len()/2;
    let (first, second) = v.split_at(mid);

    std::thread::scope(|scope| {
        let first = scope.spawn(|| first.iter().sum::<i32>());
        let second = scope.spawn(|| second.iter().sum::<i32>());
        first.join().unwrap() + second.join().unwrap()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(sum(vec![]), 0);
    }

    #[test]
    fn one() {
        assert_eq!(sum(vec![1]), 1);
    }

    #[test]
    fn five() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn nine() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]), 45);
    }

    #[test]
    fn ten() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 55);
    }
}
