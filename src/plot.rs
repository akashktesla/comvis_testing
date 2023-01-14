#![allow(warnings)]
use opencv::{
    prelude::*,
    core::{VecN,Mat,Point_, CV_8U,Rect,CV_8UC3, Size_, Scalar},
    imgcodecs::{imread,IMREAD_COLOR,IMREAD_GRAYSCALE}, highgui, 
    imgproc::{arrowed_line,rectangle,line, LINE_4,LINE_8,put_text},
};

pub fn main(){
    // let x = vec![1.,2.,3.,4.,5.,6.,7.,6.,7.,8.];
    // let y = vec![5.,2.,6.,1.,2.,3.,5.,9.,2.,1.];
    let xy:Vec<Point_<f32>> = vec![Point_::new(1.,2.),Point_::new(2.,2.),Point_::new(3.,6.),Point_::new(4.,1.),Point_::new(5.,2.)];
    let plot_img = plot(xy);
    loop{
        highgui::named_window("plot", 2);
        highgui::imshow("plot", &plot_img);
        highgui::wait_key(0);
    }

}

fn plot(data:Vec<Point_<f32>>)->Mat{
    let size = Size_::new(1366,768);
    let padding = 40;
    let sc = Scalar::new(255., 255., 255., 255.);
    let mut _plot = (Mat::ones_size(size, CV_8UC3).unwrap()+sc).into_result().unwrap().to_mat().unwrap();;
    let color = VecN::from([0.,0.,0.,1.],);
    arrowed_line(&mut _plot, Point_::new(padding,size.height-padding),Point_::new(padding,padding),color,2,1,0,0.02);
    arrowed_line(&mut _plot, Point_::new(padding,size.height-padding),Point_::new(size.width-padding,size.height-padding),color,2,1,0,0.01);

    let pp = plot_points(&data,size,padding);
    let scalar = 4.;
    println!("{:?}",pp);
    let _len = pp.len();
    let color = VecN::from([0.,0.,0.,1.]);
    for i in 1.._len{
        line(&mut _plot, pp[i-1],pp[i],color, 3, LINE_8, 0);
        let text = format!("{}",data[i].x);
        let origin = Point_::new(pp[i].x,size.height-padding/2);
        let color = VecN::from([0.,0.,0.,1.],);
        put_text(&mut _plot, &text, origin, 1, 1.5, color, 1, LINE_4, false);
    }
    _plot

}

fn plot_points(data:&Vec<Point_<f32>>,size:Size_<i32>,padding:i32)->Vec<Point_<i32>>{
    let minx = data.into_iter().min_by(|a, b| a.x.partial_cmp(&b.x).unwrap()).unwrap().x;
    let maxx = data.into_iter().max_by(|a, b| a.x.partial_cmp(&b.x).unwrap()).unwrap().x;
    let miny = data.into_iter().min_by(|a, b| a.y.partial_cmp(&b.y).unwrap()).unwrap().y;
    let maxy = data.into_iter().max_by(|a, b| a.y.partial_cmp(&b.y).unwrap()).unwrap().y;

    let mut returns = Vec::new();
    for i in data{
        let x = ((((i.x-minx)/(maxx-minx))*(size.width-2*padding) as f32)as i32)+padding;
        let y = -((((i.y-miny)/(maxy-miny))*(size.height-2*padding) as f32)as i32)+size.height-padding;
        returns.push(Point_::new(x,y));
        // returns.push((((i-min)/(max-min)) *scalar as f64)as i32)
    }
    returns
}
