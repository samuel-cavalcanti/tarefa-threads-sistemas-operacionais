use clap::clap_app;
use tarefa_threads::fibonacci;

use std::thread;
use std::{
    collections::LinkedList,
    sync::{Arc, Mutex},
    usize,
};

fn main() {
    let matches = clap_app!(tarefa_threads =>
        (author: "Samuel Cavalcanti <scavalcanti111@gmail.com>")
        (about: "Tarefa Threads, o usuario digitara na linha de comandos a quantidade de numeros de Fibonacci que o programa deve gerar ")
        (@arg Number: +required "Digite a quantidade de numeros de fibonacci")
        (version: "1.0")
    ).get_matches();

    let input = matches
        .value_of("Number")
        .unwrap()
        .parse::<usize>()
        .expect("\nError nao foi possivel fazer o parse do input. Um numero positivo era o esperado.\n");

    println!("Por favor aguarde ...");

    let mutex = Arc::new(Mutex::new(LinkedList::new()));

    let mutex_thread_child = Arc::clone(&mutex);
    let handle = thread::spawn(move || {
        let mut mutex_fibonacci_numbers = mutex_thread_child.lock().unwrap();
        *mutex_fibonacci_numbers = fibonacci(input);
    });
    handle.join().unwrap();

    let mutex_fibonacci_numbers = mutex.lock().unwrap();

    println!("Sequencia: ");

    for (index, number) in mutex_fibonacci_numbers.iter().enumerate() {
        println!("F({}) - {}", index, number);
    }
}
