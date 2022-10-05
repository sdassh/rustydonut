// Famous 3d ASCII donut, rewritten in rust. 
// Original article: https://www.a1k0n.net/2011/07/20/donut-math.html
fn main(){
    let (mut a, mut b):(f32,f32) = (0.,0.);
    let s:[char; 12] = ['.',',','-','~',':',';','=','!','*','#','$', '@' ];
    print!("\x1B[2J");
    loop {
        let (mut b2, mut z) = ([' '; 1760],[0. ;1760],);
        for j in (0..628).step_by(7){
            for i in (0..628).step_by(2){
                let sini = ((i as f32)/100.).sin();
                let cosi = ((i as f32)/100.0).cos();
                let sinj = ((j as f32)/100.0).sin();
                let cosj = ((j as f32)/100.0).cos();
                let cosj2 = cosj+2.;
                let sina = a.sin();
                let cosa =a.cos();
                let sinb = b.sin();
                let cosb = b.cos();
                let m =1./(sini*cosj2*sina+sinj*cosa+5.);
                let t =sini*cosj2*cosa-sinj*sina;
                let x = 40.+30.*m*(cosi*cosj2*cosb-t*sinb);
                let y=12.+15.*m*(cosi*cosj2*sinb+t*cosb);
                let o=x as usize+80*y as usize;
                let n=8.*((sinj*sina-sini*cosj*cosa)*cosb-sini*cosj*sina-sinj*cosa-cosi*cosj*sinb);
                if 22.>y&&y>0.&&x>0.&&80.>x&&m>z[o]{z[o]=m;b2[o]= s[n as usize];
                    }}}print!("\x1b[H");
                    for k in 0..1760 {
                        if k%80!=0{print!("{}",b2[k]);}
                        else{println!("");}}a+=0.04;b+=0.02;}}
            
    
