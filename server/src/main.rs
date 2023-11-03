use std::net::{UdpSocket, SocketAddr, ToSocketAddrs};
use std::vec;
use std::collections::HashMap;
use std::process::exit;


fn main() -> Result<(), std::io::Error> {
    let mut size =0;
   
    let mut addresses: HashMap<String, SocketAddr> = HashMap::new();

    
    // Create a UDP socket bound to the receiving address
    let socket = UdpSocket::bind("0.0.0.0:12345")?;
    println!("UDP receiver listening on 0.0.0.0:12345");
    let mut buffer = [0; 1024];
    let mut Namedirectory: Vec<String> = vec![];
    let mut Add_directory: Vec<SocketAddr> = vec![];
    let mut flag:i8 = 0;
    let mut result:usize=0;
    let mut choice: String = "0".to_string();
    loop{
  
    let (num_bytes, src_addr) = socket.recv_from(&mut buffer)?;
    let received_message: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&buffer[0..num_bytes]);
    if (size ==0)
    {
        addresses.insert(received_message.trim().to_string(),src_addr);
      
        println!("Registered {}",received_message.to_string());
        socket.send_to("Name Registered".to_string().as_bytes(), src_addr)?;
       
        let (num_bytes, src_addr) = socket.recv_from(&mut buffer)?;
        let received_message: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&buffer[0..num_bytes]);
     
        choice = received_message.trim().to_string();
        size+=1;
    }
    else{
    println!("connected address {}",src_addr);
    if let Some(result) = Add_Search(&src_addr, &addresses) {
        println!("Name already registered as: {}", result);
        
        let (num_bytes, src_addr) = socket.recv_from(&mut buffer)?;
        let received_message: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&buffer[0..num_bytes]);
                 
        choice = received_message.trim().to_string();
    
        
    } else {
        println!("Address not registered.");
        let mut flag3 = 0;
        if let Some(result) = Name_Search(&received_message.trim().to_string(), &addresses)
        {
            socket.send_to("Name is taken".to_string().as_bytes(), src_addr)?;
            println!("Name is taken");
            while flag3==0 {
                let (num_bytes, src_addr) = socket.recv_from(&mut buffer)?;
                let received_message: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&buffer[0..num_bytes]);
                if let Some(result) = Name_Search(&received_message.trim().to_string(), &addresses)
                {
                    socket.send_to("Name is taken".to_string().as_bytes(), src_addr)?;
                    println!("Name is taken");
                }
                else{
                    size+=1;
                    addresses.insert(received_message.trim().to_string(),src_addr);
                    println!("Registered {}",received_message.to_string());
                    flag3=1;
                    socket.send_to("Name Registered".to_string().as_bytes(), src_addr)?;
                
                    let (num_bytes, src_addr) = socket.recv_from(&mut buffer)?;
                    let received_message: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&buffer[0..num_bytes]);
                 
                    choice = received_message.trim().to_string();


                }

            }

        }
        
        else{
            size+=1;
            addresses.insert(received_message.trim().to_string(),src_addr);
            println!("Registered {}",received_message.to_string());
            flag3=1;
            socket.send_to("Name Registered".to_string().as_bytes(), src_addr)?;
            let (num_bytes, src_addr) = socket.recv_from(&mut buffer)?;
            let received_message: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&buffer[0..num_bytes]);
         
            choice = received_message.trim().to_string();



        }

    }
}
        

   while(choice == "1".to_string())
   {
  
    let (num_bytes, src_addr) = socket.recv_from(&mut buffer)?;
    let received_message: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&buffer[0..num_bytes]);

   // let add =Name_Search(&received_message.to_string(), &addresses);
    if let Some(add) = Name_Search(&received_message.to_string(), &addresses)
    {
        println!("Address = {:?}", add);
        socket.send_to(add.to_string().as_bytes(), src_addr)?;
        choice= "0".to_string();
    }
    else{
        let mut notf = "Name not found";
        socket.send_to(notf.to_string().as_bytes(), src_addr)?;

    }

  
   }
  

}
}

pub fn iterative_Add(a: &[SocketAddr], len: usize, target_value: SocketAddr, ite: usize) -> Option<usize> {
    let mut low: i8 = 0;
    let mut high: i8 = len as i8 - 1;

    while low <= high {
        let mid = ((high - low) / 2) + low;
        let mid_index = mid as usize;
        let val: SocketAddr = a[mid_index];

        if val == target_value {
            return Some(mid_index);
        }

        // Search values that are greater than val - to right of current mid_index
        if val < target_value {
            low = mid + 1;
        }

        // Search values that are less than val - to the left of current mid_index
        if val > target_value {
            high = mid - 1;
        }
    }

    None
}
pub fn get_name_index(name: &String, array: &mut Vec<String>) -> usize {
    let mut v: &mut Vec<String> =array;
    println!("{:?}", &v);

    v.sort_unstable();
    println!("{:?}", &v);
    println!("{}",name);
    //name.replace("\r\n", "");
    println!("{}",name);
    match array.binary_search(name) {
        Ok(index) => index,
        
        Err(_) => {
            println!("Error : variable {} not found in name array", name);
            std::process::exit(1)
        }
    }
}

pub fn Name_Search(name: &String, addresses: &HashMap<String, SocketAddr>) -> Option<SocketAddr> {


    for (key,val) in addresses.into_iter() {
        if (key == name){
                return Some(*val);
        };
        
     }
None
}

pub fn Add_Search(add: &SocketAddr, addresses: &HashMap<String, SocketAddr>) -> Option<String> {


    for (key,val) in addresses.into_iter() {
        if (val == add){
                return Some(key.to_string());
        };
        
     }
None
}
