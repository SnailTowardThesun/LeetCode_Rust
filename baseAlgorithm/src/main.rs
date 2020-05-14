
/**
 * bubble sort
 */
fn bubbleSort(array: Vec<i32>) -> Vec<i32> {
    let mut array = array;
    for i in 0..(array.len() - 1) {
        for j in 0..(array.len() - 1 - i) {
            if (array[j+1] < array[j]) {
                let tmp = array[j];
                array[j] = array[j+1];
                array[j+1] = tmp;
            }
        }
    }

    return array;
}

#[test]
fn test_bubble_sort () {
    let demo = vec![3, 5, 6, 1, 0, 2, 9];
    let array = bubbleSort(demo);

    for number in &array {
        println!("{}", number);
    }
}

/**
 * select sort
 */
fn selectSort(array: Vec<i32>) -> Vec<i32> {
    let mut array = array;

    for i in 0..array.len() {
        for j in (i+1)..array.len() {
            if array[j] < array[i] {
                let tmp = array[j];
                array[j] = array[i];
                array[i] = tmp;
            }
        }
    }

    return array;
}

#[test]
fn test_select_sort() {
    let demo = vec![3, 5, 6, 1, 0, 2, 10, 9];
    let array = selectSort(demo);

    for number in &array {
        println!("{}", number);
    }
}

/**
 * inseration sort
 */
fn inserationSort(array: Vec<i32>) -> Vec<i32> {
}

#[test]
fn test_inseration_sort() {
    let demo = vec![3, 5, 6, 1, 0, 2, 10, 9];
    let array = inserationSort(demo);

    for number in &array {
        println!("{}", number);
    }
}

fn main() {
    println!("Hello, world!");
}
