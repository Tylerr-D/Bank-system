use std::io;

fn main(){

 let mut balance = 1000;

 let mut history: Vec<String> = Vec::new();



//the fun begins
loop {
        println!("\n===== BANK =====");
        println!("Current Balance: ${}", balance);
        println!("1. Deposit");
        println!("2. Withdraw");
        println!("3. View Transaction History");
        println!("4. Exit");
        println!("Choose an option:");


    let mut choice = String::new();

    io::stdin()
     .read_line(&mut choice)
    .expect("Failed to read line");

        let choice: u32 = choice
        .trim()
        .parse()
        .expect("Enter a valid number!");

        //bunch of big blocks of if-else from here

        if choice == 1 {
            println!("Enter amount to deposit:");

            let  mut amount = String::new();

            io::stdin()
            .read_line(&mut amount)
            .expect("Failed to read the line");

            let amount: i32 = amount
            .trim()
            .parse()
            .expect("Enter a valid number bruhh");


            balance +=amount;
            history.push(format!("+${}", amount));

            println!("Deposited ${}", amount)
        }

        else if choice == 2 {
            println!("Enter amount to withdraw:");

            let  mut amount = String::new();

            io::stdin()
            .read_line(&mut amount)
            .expect("Failed to read the line");

            let amount: i32 = amount
            .trim()
            .parse()
            .expect("Enter a valid number bruhh");

            if amount<= balance{
            balance -=amount;
            history.push(format!("-${}", amount));

            println!("Withdrew ${}", amount)
            }

            else{
                println!("Insufficient funds!");
                println!("Earn some money gng");
            }
        }

            else  if choice == 3 {
            println!("\n===== TRANSACTION HISTORY =====");
            if history.is_empty(){
                println!("no transactions yet")
            }

            else {
                for transaction in &history{
println!("{}", transaction);
                }
            }

        }

                    else if choice == 4 {
            println!("Thanks for using the bank!");
            break;
                    }
else {
            println!("Invalid choice!");
}
             }
            
}     
