use std::collections::LinkedList;

pub fn fibonacci(number: usize) -> LinkedList<i32> {
    let mut fibonacci_numbers: LinkedList<i32> = LinkedList::new();
    fibonacci_numbers.push_back(0);
    if number == 1 {
        return fibonacci_numbers;
    }

    fibonacci_numbers.push_back(1);
    let mut size = fibonacci_numbers.len();
    if number == size {
        return fibonacci_numbers;
    }

    let mut n_element: i32 = 0;

    let mut n_1_element: i32 = 1;

    while size < number {
        size = fibonacci_numbers.len();

        let option = n_1_element.checked_add(n_element);

        match option {
            Some(next_fibonacci_number) => {
                fibonacci_numbers.push_back(next_fibonacci_number);
                n_element = n_1_element;
                n_1_element = next_fibonacci_number;
            }
            None => {
                eprintln!(
                    "Nao foi possivel adicionar, variavel inteira de 32 bits atingiu seu limite!!"
                );
                return fibonacci_numbers;
            }
        }
    }

    return fibonacci_numbers;
}

mod test_fibonacci {
    use std::collections::LinkedList;

    use super::fibonacci;

    #[test]
    fn test_fibonacci() {
        let mut expected = LinkedList::new();

        expected.push_back(0);
        assert_eq!(expected, fibonacci(1));
        expected.push_back(1);
        assert_eq!(expected, fibonacci(2));

        for element in vec![
            1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597, 2584, 4181, 6765,
        ] {
            expected.push_back(element);
        }

        assert_eq!(expected, fibonacci(20))
    }

    #[test]
    fn test_overflow_fibonacci() {
        let fibonacci_numbers = fibonacci(100);

        assert_eq!(fibonacci_numbers.len(), 47);
    }
}
