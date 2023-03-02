// fn main() {
//     let mut input = String::new();
//     stdin().read_line(&mut input).unwrap();

//     // read_line leaves a trailing newline, which trim removes
//     let command = input.trim();

//     Command::new(command)
//         .spawn()
//         .unwrap();
// }
//
// fn main() {
//     loop {
//         // use the '>' character as the prompt
//         // need to explicitly flush this to ensure it prints before read_line
//         print!("> ");
//         stdout().flush().expect("Some error message");

//         let mut input = String::new();
//         stdin().read_line(&mut input).unwrap();

//         let command = input.trim();
//         let mut child = Command::new(command).spawn().unwrap();
//         // don't accept another commnad until this one completes
//         child.wait().expect("Some error message");
//     }
// }

// fn main() {
//     loop {
//         print!("[Shell]:==>");
//         stdout().flush().expect("Error");

//         let mut input = String::new();
//         stdin().read_line(&mut input).unwrap();

//         // everyhing after the first whitespace character
//         // is interpreted as args to the command
//         let mut parts = input.trim().split_whitespace();
//         let command = parts.next().unwrap();
//         let args = parts;

//         let mut child = Command::new(command)
//             .args(args)
//             .spawn()
//             .unwrap();

//         child.wait().expect("Error");
//     }
// }
// fn main() {
//     loop {
//         print!("> ");
//         stdout().flush().expect("This is a msg");

//         let mut input = String::new();
//         stdin().read_line(&mut input).unwrap();

//         let mut parts = input.trim().split_whitespace();
//         let command = parts.next().unwrap();
//         let args = parts;

//         match command {
//             "cd" => {
//                 let new_dir = args.peekable().peek().map_or("/", |x| *x);
//                 let root = Path::new(new_dir);
//                 if let Err(e) = env::set_current_dir(&root) {
//                     eprintln!("{}", e);
//                 }
//             }
//             "exit" => return,
//             command => {
//                 let child = Command::new(command).args(args).spawn();
//                 // gracefully handle malformed user input
//                 match child {
//                     Ok(mut child) => {
//                         child.wait().expect("This is a msg");
//                     }
//                     Err(e) => eprintln!("{}", e),
//                 }
//             }
//         }
//     }
// }
