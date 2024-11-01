use std::io;

const BRACKETS: [(char, char); 3] = [('(', ')'), ('{', '}'), ('[', ']')];

fn main() {
    first()
}

fn first() {
    let mut input = String::new();
    let mut stack: Vec<char> = Vec::new();

    println!("Введите строку");
    io::stdin().read_line(&mut input).unwrap();

    for s in input.chars() {
        for (opening, closing) in BRACKETS {
            if s == opening {
                stack.push(s);
            } else if s == closing {
                match stack
                    .pop()
                    .expect("Плохая строка: Не обнаружена открывающая скобка")
                {
                    bracket if bracket == opening => bracket,
                    _ => panic!(
                        "Плохая строка: Найдено несоответствие открывающей и закрывающей скобок"
                    ),
                };
            }
        }
    }

    if stack.len() != 0 {
        panic!("Плохая строка: обнаружена незакрытая скобка")
    } else {
        println!("Прекрасная строка!")
    }
}
