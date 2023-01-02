#![ allow(warnings) ]
use image::io::Reader as ImageReader;
use image::{Luma,Rgb};
use image::ImageBuffer;
use image::DynamicImage;
use imageproc::filter::filter3x3;
use imageproc::window::display_image;
use std::thread;
fn main() {
    let mut img =  ImageReader::open("testImages/img.png").unwrap().decode().unwrap();
    shape_classification(img);
}

fn shape_classification(img:DynamicImage){
    let mut img_rgb = img.clone().into_rgb8();
    let laplace_kernel = [0.,1.,0.,1.,-4.,1.,0.,1.,0.];
    let mut img2: image::ImageBuffer<image::Luma<u8>, std::vec::Vec<u8>> = filter3x3(&mut img.into_luma8(),&laplace_kernel);
    display_image("image",&img2,100,100);
    let x = img2.width();
    let y = img2.height();

    let mut rc:ImageBuffer<Luma<u8>, Vec<u8>> = ImageBuffer::new(x, y);
    let mut gc:ImageBuffer<Luma<u8>, Vec<u8>> = ImageBuffer::new(x, y);
    let mut bc:ImageBuffer<Luma<u8>, Vec<u8>> = ImageBuffer::new(x, y);
    for i in 0..x{
        for j in 0..y{
            let red_pixel = img_rgb.get_pixel(i, j)[0];
            let pixel = rc.get_pixel_mut(i,j);
            *pixel = Luma([red_pixel]); 

            let green_pixel = img_rgb.get_pixel(i, j)[1];
            let pixel = gc.get_pixel_mut(i,j);
            *pixel = Luma([green_pixel]); 

            let blue_pixel = img_rgb.get_pixel(i, j)[2];
            let pixel = bc.get_pixel_mut(i,j);
            *pixel = Luma([blue_pixel]); 
        }
    }
    let laplace_kernel = [0.,1.,0.,1.,-4.,1.,0.,1.,0.];
    let redge: image::ImageBuffer<image::Luma<u8>, std::vec::Vec<u8>> = filter3x3(&mut rc,&laplace_kernel);
    let gedge: image::ImageBuffer<image::Luma<u8>, std::vec::Vec<u8>> = filter3x3(&mut gc,&laplace_kernel);
    let bedge: image::ImageBuffer<image::Luma<u8>, std::vec::Vec<u8>> = filter3x3(&mut bc,&laplace_kernel);
    // display_image("red", &redge, 100, 100);
    // display_image("green", &gedge, 100, 100);
    // display_image("blue", &bedge, 100, 100);
    
    //combine all the edges to one
    //final image
    let mut fi:ImageBuffer<Luma<u8>, Vec<u8>> = ImageBuffer::new(x, y);
    for i in 0..x{
        for j in 0..y{
            let r = redge.get_pixel(i, j).0[0];
            let g = gedge.get_pixel(i, j).0[0];
            let b = bedge.get_pixel(i, j).0[0];
            let mut pixel = fi.get_pixel_mut(i,j);
            let mut temp = r as i16 + g as i16 + b as i16;
            if temp > 255{
                temp = 255
            }
            *pixel = Luma([temp as u8])
        }
    }

    display_image("final image", &fi, 1000, 1000);

}







