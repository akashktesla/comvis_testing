#![allow(warnings)]
use image::ImageBuffer;
use opencv::{
    prelude::*,
    core::VecN,
    videoio::{VideoCapture,CAP_ANY},
    imgcodecs::{imread,IMREAD_COLOR},
    highgui,
};
use rgb::RGB;
pub fn main(){
    // Process images in Vec<Vec<VecN<u8,3>>>
    // 3 - no of channels
    //
    //Video capture shit...
    // let mut cam = VideoCapture::new(0,CAP_ANY).unwrap();
    // let mut frame = Mat::default(); 
    // loop{
    //     cam.read(&mut frame).unwrap();
    //     let frame_vec:Vec<Vec<VecN<u8,3>>> = frame.to_vec_2d().unwrap(); //let's use this shit for image processing
    //     highgui::imshow("testing", &frame);
    //     highgui::wait_key(1);
    // }

    // reading file from filename
     let a = imread("testImages/A120roombu.jpg",IMREAD_COLOR).unwrap();
     let a_vec:Vec<Vec<VecN<u8,3>>> = a.to_vec_2d().unwrap(); 
     let kernel = [0,1,0,1,-4,1,0,1,0];
     filter3x3(a_vec, kernel);
     loop{
         highgui::imshow("imread",&a);
         highgui::wait_key(0);
     }
}

fn correlation(a:[i32;9],b:[i32;9])-> [i32;9]{
    let mut c:[i32;9] = [0,0,0,0,0,0,0,0,0];
    for i in 0..9{
        c[i] = a[i]* b[i]; 
    }
    return c
}

fn filter3x3(img:Vec<Vec<VecN<u8,3>>>,kernel:[i32;9]){


}




