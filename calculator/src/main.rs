
use std::io;


fn main() {

    println!("=====================================================");
    println!("                      WELCOME!                       ");
    println!("=====================================================");

    loop{
        // take in the two inputs and the operator
        let mut num1 = String::new();
        let mut num2 = String::new();
        let mut operator = String::new();

        // Input the first number and the preparation of it
        println!("Enter your first number: ");
        io::stdin().read_line(&mut num1);
        let num1: u32 = match num1.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // the operation operator
        println!("Enter the operation you want to perform:  (+-*/)");
        io::stdin().read_line(&mut operator);
        //let operator = operator.to_string();


        // Input the second number and preprocess it
        println!("Enter your second number: ");
        io::stdin().read_line(&mut num2);
        let num2: u32 = match num2.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        let operator: char = operator.trim().chars().next().unwrap();

        if operator == '+'{
            println!("The result of {} + {} = {}",num1, num2, num1 + num2)
        }
        else if operator == '-'{
            println!("The result of {} - {} = {}", num1, num2, num1 - num2)
        }
        else if operator == '*'{
            println!("The result of {} * {} = {}", num1, num2, num1 * num2)
        }
        else if operator == '/'{
            println!("The result of {} / {} = {}", num1, num2, num1 / num2)
        }
        else{
            println!("The operation is impossible!");
        }
        

        // ending the program
        println!("Do you want to quit?  (y, n)");
        let mut comm = String::new();
        io::stdin().read_line(&mut comm);
        let comm: char = comm.trim().chars().next().unwrap();
        if comm == 'y'{
            println!("=====================================================");
            println!("                   THANK YOU, BYE!                   ");
            println!("=====================================================");
            break;
        }
        
    }

    //println!("Hello, world!");
}
