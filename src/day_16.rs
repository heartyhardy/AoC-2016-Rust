
struct Disk{
    size:usize,
    initial:Vec<u32>,
    fit:Vec<u32>,
    checksum:Vec<u32>
}

impl Disk{
    //New Disk with default size
    pub fn new()->Disk{
        let size = 35651584;
        let sinit = "10010000000110000";
        let mut initial:Vec<u32> = sinit.chars().map(|c|c.to_digit(10).unwrap()).collect();
        let fit:Vec<u32>=Vec::new();
        let checksum:Vec<u32> = Vec::new();

        Disk{
            size,
            initial,
            fit,
            checksum
        }
    }

    //fill the disk
    pub fn fill(&mut self){
        if self.initial.len() >= self.size{
            self.fit = self.initial[0..self.size].to_vec();
            return
        }

        let mut dup:Vec<u32> = self.initial.clone();
        dup.reverse();

        for d in 0..dup.len(){
            match dup[d]{
                0 => dup[d]=1,
                1 => dup[d] =0,
                _ => ()
            }
        }

        self.initial.push(0);
        self.initial.append(&mut dup);

        self.fill();
    }

    pub fn checksum(&mut self){

        if self.checksum.len() & 1 ==  1 {
            return
        }

        let mut i = 1;
        if self.checksum.len()==0{
            self.checksum = self.fit.clone();
        }
        
        let mut csum:Vec<u32> = Vec::new();

        while i < self.checksum.len(){
            if self.checksum[i] == self.checksum[i-1]{
                csum.push(1);
            }else{
                csum.push(0);
            }
            i+=2;
        }

        self.checksum.clear();
        self.checksum.append(&mut csum);

        self.checksum();
    }
}


pub fn run(){
    let mut disk = Disk::new();
    disk.fill();
    println!("{:?}", disk.initial );
    disk.checksum();
    println!("{:?}", disk.checksum);
}
