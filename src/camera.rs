#![allow(warnings)]
use std::cmp::{max,min};
use image::{ImageBuffer, Luma};
use imageproc::window::display_image;
use opencv::{
    prelude::*,
    plot,
    core::{VecN,ToInputArray,Mat,BORDER_DEFAULT,Point_,_InputArray,CV_32F,Vector,add_mat_mat,and_mat_mat,KeyPoint, 
        BORDER_CONSTANT,Rect},
    videoio::{VideoCapture,CAP_ANY},
    imgproc::{filter_2d,cvt_color,COLOR_BGR2GRAY,median_blur,erode,dilate,rectangle, LINE_8},
    features2d::{SimpleBlobDetector,SimpleBlobDetector_Params},
    imgcodecs::{imread,IMREAD_COLOR,IMREAD_GRAYSCALE},
    highgui, types::VectorOfi32, sys::cv_morphologyDefaultBorderValue,
};

pub fn main(){
    // Process images in Vec<Vec<VecN<u8,3>>>
    // 3 - no of channels
    //Video capture shit...
    // let mut cam = VideoCapture::new(0,CAP_ANY).unwrap();
    // let mut frame = Mat::default(); 
    // let mut frame_q = ImgQueue::new(3);
    // loop{
    //     cam.read(&mut frame).unwrap();
    //     let kernel = Mat::from_exact_iter(vec![0,1,0,1,-4,1,0,1,0].into_iter()).unwrap();
    //     let mut filtered_frame = Mat::default();
    //     let mut gray_frame = Mat::default();
    //     let mut median_frame = Mat::default();
    //     median_blur(&frame,&mut median_frame,7);
    //     cvt_color(&median_frame, &mut gray_frame, COLOR_BGR2GRAY, 0);
    //     filter_2d(&gray_frame, &mut filtered_frame, -1 ,&kernel,Point_::new(-1,-1),0 as f64,BORDER_DEFAULT);
    //     let mut sum_frame = filtered_frame.clone();
    //     frame_q.push(filtered_frame.clone());
    //     let _len = frame_q.data.len();
    //     for i in 0..frame_q.data.len(){
    //         sum_frame = add_mat_mat(&sum_frame,&frame_q.data[i]).unwrap().to_mat().unwrap();
    //     }
    //     for i in 0..sum_frame.rows(){
    //         for j in 0..sum_frame.cols(){
    //             let pix:&mut VecN<u8,1> = sum_frame.at_2d_mut(i,j).unwrap();
    //             pix.0[0] = (pix.0[0] as i32+100) as u8;
    //         }
    //     }

        // let mut blobparms = SimpleBlobDetector_Params::default().unwrap();
        // blobparms.filter_by_color = false;
        // blobparms.filter_by_circularity = true;
        // blobparms.filter_by_convexity = true;
        // println!("{:?}",blobparms);
        // let mut detector = SimpleBlobDetector::create(blobparms).unwrap();
        // let mut key_points = Vector::new();
        // let mask = Mat::default();
        // let mut descriptor = Mat::default();
        // detector.detect(&median_frame,&mut key_points,&mask);
        // detector.detect_and_compute(&mut median_frame, &mask, &mut key_points,&mut descriptor, false);
        // println!("{:?}",key_points);
        // highgui::imshow("testing", &sum_frame);
        // highgui::wait_key(1);
    // }
    // reading file from filename
     let src_img = imread("testImages/testGreenCircle.jpg",IMREAD_COLOR).unwrap();
     let mut filtered_img = Mat::default();
     let kernel = Mat::from_exact_iter(vec![0,1,0,1,-4,1,0,1,0].into_iter()).unwrap();
     let mut gray_img = Mat::default();
     cvt_color(&src_img, &mut gray_img, COLOR_BGR2GRAY, 0).unwrap();
     filter_2d(&gray_img, &mut filtered_img, -1 ,&kernel,Point_::new(-1,-1),0 as f64,BORDER_DEFAULT);
     let mut eroded_img = Mat::default();
     let kernel = Mat::default();
     dilate(&filtered_img, &mut eroded_img, &kernel, Point_::new(-1,-1) ,1, BORDER_CONSTANT, VecN::from([0.,0.,0.,0.]));
     let r_img = frame_gen_crop(&eroded_img,250,10);
     img_to_signal(&r_img);
     loop{
         highgui::imshow("src img",&src_img);
         highgui::wait_key(0);
     }
}
fn img_to_signal(img:&Mat){
    let threshold = 250;
    let rows = img.rows();
    let cols = img.cols();
    let mut pixels = Vec::new();
    for i in 0..rows{
        for j in 0..cols{
            let pix:&u8 = img.at_2d(i,j).unwrap();
            if *pix > 250 as u8{
                pixels.push([i,j]);
            }
        }
    }
    println!("pixels: {:?}",pixels);
}

fn frame_gen(img:&Mat,threshold:u8,padding:i32)->((i32,i32),(i32,i32)){
     let mut minr = 0;
     let mut maxr = 0;
     let mut minc = 0;
     let mut maxc = 0;
    //top to down
    let rows = img.rows();
    let cols = img.cols();
    let mut flag = true;
    let mut i = 0;
    while(flag && i<rows){
        let mut j = 0;
        while(j< cols){
            let pix:&VecN<u8,1> = img.at_2d(i,j).unwrap();
            if pix[0]>threshold{
                minr = i;
                flag = false;
                break;
            }
            j+=1;
        }
        i+=1;
    }
    //down to top
    flag = true;
    let mut i = rows-1;
    while(flag && i>0){
        let mut j = 0;
        while(j< cols){
            let pix:&VecN<u8,1> = img.at_2d(i,j).unwrap();
            if pix[0]>threshold{
                maxr = i;
                flag = false;
                break;
            }
            j+=1;
        }
        i-=1;
    }

    //left to right
    flag = true;
    let mut i = 0;
    while(flag && i<cols){
        let mut j = 0;
        while(j<rows){
            let pix:&VecN<u8,1> = img.at_2d(j,i).unwrap();
            if pix[0]>threshold{
                minc = i;
                flag = false;
                break;
            }
            j+=1;
        }
        i+=1;
    }
    //right to left
    flag = true;
    let mut i = cols-1;
    while(flag && i>0){
        let mut j = 0;
        while(j<rows){
            let pix:&VecN<u8,1> = img.at_2d(j,i).unwrap();
            if pix[0]>threshold{
                maxc = i;
                flag = false;
                break;
            }
            j+=1;
        }
        i-=1;
    }


    minr = max(minr-padding,0);
    minc = max(minc-padding,0);
    maxr = min(maxr+padding,rows);
    maxc = min(maxc+padding,cols);
    ((minr,minc),(maxr,maxc))
}

fn frame_gen_rect(img:&Mat,threshold:u8,padding:i32)->Rect{
    let minr;
    let maxr;
    let minc;
    let maxc;
    ((minr,minc),(maxr,maxc)) = frame_gen(img,threshold,padding);
    let r = relu(minc - padding);
    let c = relu(minr - padding);
    let width = maxc-minc + 2*padding;
    let height = maxr-minr + 2*padding;
    Rect::new(r,c,width,height)
}

fn frame_gen_crop(img:&Mat,threshold:u8,padding:i32)->Mat{
    let minr;
    let maxr;
    let minc;
    let maxc;
    ((minr,minc),(maxr,maxc)) = frame_gen(img,threshold,padding);
    let img = img.row_bounds(minr,maxr).unwrap(); 
    let img = img.col_bounds(minc,maxc).unwrap();
    img
}


fn relu(x:i32)->i32{
    if x < 0{
        return 0
    }
    return x
}


struct ImgQueue{
    data:Vec<Mat>,
    size:usize

}
impl ImgQueue{
    fn new(size:usize)->ImgQueue{
        ImgQueue{
            data:Vec::new(),
            size

        }
    }
    fn push(&mut self,data:Mat){
        let _len = self.data.len();
        if _len >= self.size{
            self.data.remove(0);
            self.data.push(data);
        }
        else{
            self.data.push(data);
        }
    }
}

fn correlation(a:[u8;9],b:[i32;9])-> u8{
    let mut c:i32 = 0;
    for i in 0..9{
        c += a[i] as i32 * b[i]; 
    }
    if c>0{
        return c as u8
    }
        return 0 as u8
}

fn filter_test(img:&mut Mat){
    let rows = img.rows();
    let cols = img.cols();
    for i in 0..rows{
        for j in 0..cols{
            // let a = img.
            let mut a:&mut VecN<u8,3> = img.at_2d_mut(i,j).unwrap();
            a[0] = 150;
        }
    }
}

//provide grayscale image pleamz
fn filter3x3(mut img:Vec<Vec<VecN<u8,3>>>,kernel:[i32;9])->Vec<Vec<u8>>{
    let height = img.len();
    let width = img[0].len();
    let mut r_img:Vec<Vec<u8>> = Vec::new();
    for i in 1..height-1{
        let mut temp = Vec::new();
        for j in 1..width-1{
            //initialize image matrix
            let matrix_3x3 =  [
            img[i-1][j-1][0],
            img[i-1][j][0],
            img[i-1][j+1][0],
            img[i][j-1][0],
            img[i][j][0],
            img[i][j+1][0],
            img[i][j-1][0],
            img[i][j][0],
            img[i][j+1][0]
            ];
            let mut c = correlation(matrix_3x3, kernel);
            // println!("{}",c);
            temp.push(c);
        }
        r_img.push(temp);
    }
    return r_img
}


