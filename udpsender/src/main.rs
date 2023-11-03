use std::net::UdpSocket;
use std::io;
use std::process::exit;
fn main() -> Result<(), std::io::Error> {
    // Create a UDP socket bound to a local address
    let socket = UdpSocket::bind("127.0.0.1:1001")?;
    let flag2=0;
    // Destination address (IP and port) of the receiver
    let dest_addr = "127.0.0.1:12345";
    let mut flag3 =0;
    // Message to send
   // let message = "Ahmed";
   while flag3 == 0
   {
        println!("Enter Your  Name ");
        let mut name = String::new();
        io::stdin().read_line(&mut name).unwrap();
        socket.send_to(name.trim().as_bytes(), dest_addr)?;
        let mut buffer = [0; 1024];
        let (num_bytes, src_addr) = socket.recv_from(&mut buffer)?;
        let received_message: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&buffer[0..num_bytes]);
        println!("You recieved a message'{}'",  received_message);

       if received_message.trim().to_string()== "Name Registered".to_string()
       {
      
        flag3=1; 
      
       }
}
    loop{
    println!("Choose to 1. Send \n 2. Recieve  \n 3.Exit");
    let mut choice: String = String::new();
    io::stdin().read_line(&mut choice).unwrap();
    let c: i32 = choice.trim().parse().unwrap();
    if(c==1)
    {     
        let mut flag:&str = "1";
        socket.send_to(flag.as_bytes(), dest_addr)?;
       
        // Send the message to the destination address
        let mut flag = 0;
        while flag==0{
        println!("Enter Receiver's Name ");
        let mut message = String::new();
        io::stdin().read_line(&mut message).unwrap();
        // Send the message to the destination address
        socket.send_to(message.trim().as_bytes(), dest_addr)?;
        println!("Message sent to {}: {}", dest_addr, message);
     
        
        
            let mut buffer = [0; 1024];
            let (num_bytes, src_addr) = socket.recv_from(&mut buffer)?;
            let received_message: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&buffer[0..num_bytes]);
            println!("You recieved a message'{}'",  received_message);
            if (received_message.to_string() == "Name not found".to_string())
            {
                println!("Name not found, Try another name");
                
                
            }
            else
             {
                let mut add = received_message.to_string();
                let  dest_addr =add.as_str(); 
                println!("Name found at address '{}' ", dest_addr);
                println!("Enter Message ");
                let mut m = String::new();
                io::stdin().read_line(&mut m).unwrap();
                // Send the message to the destination address
                socket.send_to(m.as_bytes(), dest_addr)?;
                flag =1;
            }

        }
      
    }
    if(c==2)
    {

        
        let mut flag:&str = "2";
        socket.send_to(flag.as_bytes(), dest_addr)?;
     
        
       
        loop{
       
        let mut buffer = [0; 1024];
        let (num_bytes, src_addr) = socket.recv_from(&mut buffer)?;
        let received_message: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&buffer[0..num_bytes]);
        println!("You recieved a message'{}' from '{}'",  received_message.trim(),src_addr);
        println!("Do you want to exit reciving state?");
        println!("Press 2 to exit recieving halt, press any button to remain");
        let mut x = String::new();
        io::stdin().read_line(&mut x).unwrap();
        let x2:i8 =x.trim().parse().unwrap();
        if x2 ==2
        {
            break;
        }
        }
    }
    if(c==3)
    {
        exit(1);
    }
}
    Ok(())
}