use std::collections::HashMap;
use std::thread;
use std::time;
use std::collections::HashSet;
use std::hash::Hash;

fn vectors() {
    let mut a = Vec::new();
    a.push(1);
    a.push(2);
    a.push(3);

    println!("a = {:?}", a);

    // usize isize
    let idx: usize = 0;  // index must be usize -> think about it; it makes sense
    a[idx] = 123;
    println!("a[{}] = {}", idx, a[idx]);

    match a.get(6) {  // does not crash -> return option type
        Some(x) => println!("a[6] = {}", x),
        None => println!("no element on pos 6")
    }

    for x in &a {
        println!("{}", x)
    }

    loop {
        let last_elem = a.pop();
        match last_elem {
            Some(x) => println!("last element was {}", x),
            None => break
        }
    }

    // code below is similar to the loop above
    // while let Some(x) = a.pop() {
    //     println!("last element was {}", x)
    // }
}


fn hash_maps() {
    let mut shapes = HashMap::new();
    shapes.insert(String::from("triangle"), 3);
    shapes.insert(String::from("square"), 4);
    println!("a {} has {} sides", "square", shapes["square"]);

    for (key, value) in &shapes {
        println!("{} : {}", key, value);
    }

    shapes.insert("square".into(), 5);

    println!("{:?}", shapes);

    shapes.entry("circle".into()).or_insert(1);  // check if circle is in hashmap; if not insert 1
    println!("{:?}", shapes);

    {
        let actual= shapes.entry("circle".into()).or_default();
        *actual = 0;
    }
    println!("{:?}", shapes);

}


fn hash_sets() {
    let mut greeks = HashSet::new();
    greeks.insert("gamma");
    greeks.insert("delta");
    println!("{:?}", greeks);

    greeks.insert("gamma");  // returns boolean
    println!("{:?}", greeks);

    if ! greeks.contains("kappa") {
        println!("kappa not in greeks");
    }

    greeks.remove("delta");  // returns boolean

    let _1_5: HashSet<_> = (1..=5).collect();
    let _6_10: HashSet<_> = (6..=10).collect();
    let _1_10: HashSet<_> = (1..=10).collect();
    let _2_8: HashSet<_> = (2..=8).collect();

    println!("is {:?} a subset of {:?} -> {}", _2_8, _1_10, _2_8.is_subset(&_1_10));
    println!("is {:?} a disjoint of {:?} -> {}", _1_5, _6_10, _1_5.is_disjoint(&_6_10));

    // can also do union, intersection
    // difference
    // symmetric_difference = union - intersection
}


fn iterators() {
    let mut vec = vec![3, 2, 1];  // macro to create the vector

    for x in &vec {
        println!("{}", *x);
    }

    for x in vec.iter() {
        println!("{}", x);  // rust figures out if we are accessing a reference and follows it if needed (kinda like python)
        // x +=1 doesn't compile because x is immutable
    }

    for x in vec.iter_mut() {
        *x += 2;
    }

    for x in &vec {
        println!("{}", *x);
    }
}


fn main() {
    vectors();  // similar to python list, but typed
    hash_maps();  // similar to python dict, but typed
    hash_sets();
    iterators();
}
