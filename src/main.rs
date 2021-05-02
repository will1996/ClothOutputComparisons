use std::fs;
fn main() {
   let mut args = std::env::args().skip(1) ;
   let verts1 = read_obj_verts(args.next().unwrap());
   let verts2 = read_obj_verts(args.next().unwrap());
   if verts1.len() == verts2.len(){
    println!("{}",compare_obj_verts(&verts1, &verts2));
   }
   else {
    println!("The two files contain differing numbers of vertices, the error is unacceptably high");
   }
}


fn read_obj_verts(file_path : String)->Vec<Vertex>{
           //let msg = format!("File not found at specified address {}",file_path);
           let f  = fs::read_to_string(file_path).unwrap();
           let mut verts = Vec::new();
           for line in f.lines(){

               println!("line contents: {}",line);
               let mut coords = line.splitn(4,' ');
               let t = coords.next().expect("No type specifier");
               if t=="vt" {
                   //get each value as a string, if It's missing, then just set it to zero
                    let x = match coords.next(){None => "0.0", Some(s)=>s} ;
                    let y = match coords.next(){None => "0.0", Some(s)=>s} ;
                    let z = match coords.next(){None => "0.0", Some(s)=>s} ;

                    println!("t (element type vt, e, f): {}",t);
                    println!("x: {}",x);
                    println!("y: {}",y);
                    println!("z: {}",z);

                verts.push(Vertex{
                    x: match x.parse(){Ok(num)=> num, Err(_) => panic!("Non Numeric Input")},
                    y: match y.parse(){Ok(num)=> num, Err(_) => panic!("Non Numeric Input")},
                    z: match z.parse(){Ok(num)=> num, Err(_) => panic!("Non Numeric Input")}
                })
            }

           };
           return verts
}

fn compare_obj_verts(obj1 : &Vec<Vertex> , obj2 : &Vec<Vertex> )->f64{
        let mut sum = 0.0;
        for i in 0..obj1.len() {
            sum += Vertex::squared_error(&obj1[i],&obj2[i]);
        }
        return sum;
}


struct Vertex{
    x : f64,
    y : f64,
    z : f64
}


impl Vertex{
    fn squared_error(this: &Vertex, other : &Vertex)-> f64{
            (this.x-other.x).powf(2.0)+
            (this.y-other.y).powf(2.0)+
            (this.z-other.z).powf(2.0)
    }



}