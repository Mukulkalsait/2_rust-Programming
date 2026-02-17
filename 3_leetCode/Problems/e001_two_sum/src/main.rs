use std::collections::HashMap;


fn main() {
    let data: Vec<u32> = vec![2,3,7,12,24,33];
    let target: u32 = 19;
    

    let res:Vec<usize> = two_sum(data, target);
    println!("{:?}",res);

    
}

fn two_sum(data:Vec<u32>,target:u32)-> Vec<usize>{
    let res: Vec<usize> = vec![];
    let mut map:HashMap<u32,usize> = HashMap::new();

    for (i, &val) in data.iter().enumerate(){ 

        let compliment = target - val;

        if let Some(&index) = map.get(&compliment) { 
            return  vec![index,i];
        }
        map.insert(val, i);
    }
    res
}


