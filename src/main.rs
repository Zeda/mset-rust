extern crate image;

use image::{ImageBuffer, Rgb};
fn atou32(s : &str) -> u32{
    let mut x = 0;
    for c in s.chars(){
        x = x*10+(c as u32-'0' as u32);
    }
    return x;
}
const CUTOFF : f64 = 256.0;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let width;
    let height;
    let max_iter;
    let cm0;
    let cm1;
    let cm2;
    if args.len()>1{
        width = atou32(&args[1]);
    } else {
        width = 512;
    }

    if args.len()>2{
        height = atou32(&args[2]);
    } else {
        height = 512;
    }

    if args.len()>3{
        max_iter = atou32(&args[3]);
    } else {
        max_iter = 128;
    }

    if args.len()>4{
        cm0 = atou32(&args[4]);
    } else {
        cm0 = 19;
    }

    if args.len()>5{
        cm1 = atou32(&args[5]);
    } else {
        cm1 = 19;
    }

    if args.len()>6{
        cm2 = atou32(&args[6]);
    } else {
        cm2 = 19;
    }

    // a default (black) image containing Rgb values
    let mut image = ImageBuffer::<Rgb<u8>, Vec<u8>>::new(width, height);
    let mut x = 0;
    let mut cx = -2.0;
    let delta = 2.5/(width as f64);
    while x<width{
        let mut y = 0;
        let mut cy = -1.25;
        while y<height{
            let mut zx = cx;
            let mut zy = cy;
            let mut i = max_iter;
            let mut zx2 = zx*zx;
            let mut zy2 = zy*zy;
            while (i>0)&(zx2+zy2<CUTOFF){
                zy = 2.0*zx*zy+cy;
                zx = zx2-zy2+cx;
                zx2 = zx*zx;
                zy2 = zy*zy;
                i-=1;
            }
            let c0l=(i*cm0&255) as f64;
            let c0;
            if i==0{
                c0 = c0l as u8;
            } else {
                let c0u=((i+1)*cm0&255) as f64;
                c0 = (((256.0+c0u-(c0u-c0l)*CUTOFF/(zx2+zy2)) as u32)&255) as u8;
            }

            let c1l=(i*cm1&255) as f64;
            let c1;
            if i==0{
                c1 = c1l as u8;
            } else {
                let c1u=((i+1)*cm1&255) as f64;
                c1 = (((256.0+c1u-(c1u-c1l)*CUTOFF/(zx2+zy2)) as u32)&255) as u8;
            }

            let c2l=(i*cm2&255) as f64;
            let c2;
            if i==0{
                c2 = c2l as u8;
            } else {
                let c2u=((i+1)*cm2&255) as f64;
                c2 = (((256.0+c2u-(c2u-c2l)*CUTOFF/(zx2+zy2)) as u32)&255) as u8;
            }

            image.put_pixel(x, y, image::Rgb([c0,c1,c2]));
            y+=1;
            cy += delta;
        }
        cx += delta;
        x+=1;
    }
    // set a central pixel to white

    // write it out to a file
    image.save("output.png").unwrap();
}
