use std::collections::HashMap;



fn main() {
    let data:Vec<u32>= vec![2,7,11,15];
    let target: u32 = 9;

    let res:Vec<usize> = two_sum(data,target);
    println!("resualt = {:?}",res);
}


fn two_sum(data:Vec<u32>, target:u32)-> Vec<usize>{
    let res: Vec<usize> = vec![];
    let mut map:HashMap<u32,usize> = HashMap::new();

    for (i, &value) in data.iter().enumerate() { 
        // println!("i = {}, and val = {}",i ,  value);
        let compliment = target - value;

        if let Some(&index) = map.get(&compliment){
             return vec![index,i];
        }
        map.insert(value, i);
    }
    res
}
