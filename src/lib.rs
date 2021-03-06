mod utils;

extern crate rand;
use crate::rand::Rng;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;



// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(a: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}


#[wasm_bindgen]
extern {
    fn alert(s: &str);
}


#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    
    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");
    // let canvas=documen
    // Manufacture the element we're gonna append
    let val = document.create_element("canvas")?;
    let canvas = document.get_element_by_id("canvas").unwrap();
    // val.set_inner_html("Hello from Rust!");

    // body.append_child(&val)?;
    let canvas: web_sys::HtmlCanvasElement = canvas
    .dyn_into::<web_sys::HtmlCanvasElement>()
    .map_err(|_| ())
    .unwrap();

    let windows: web_sys::Window =web_sys::window().unwrap();
    let width=windows.inner_width()?;
    let height=windows.inner_height()?;
    let uwidth=width.as_f64().unwrap();
    let uheight=height.as_f64().unwrap();
    console_log!("createdddd module has {}  {} pages of memory",uwidth,uheight );
    let context = canvas
    .get_context("2d")
    .unwrap()
    .unwrap()
    .dyn_into::<web_sys::CanvasRenderingContext2d>()
    .unwrap();
    // context.width=windows.innerWidth;
    canvas.set_width(uwidth as u32);
    canvas.set_height(uheight as u32);
//context.set_fill_style(&"#0000FF".into());     
    context.set_fill_style(&"#000000".into());        

    context.fill_rect(0.0, 0.0, uwidth, uheight);

    context.set_fill_style(&"#076ab0".into());
     let mut x1:f64=100.0;
     let mut y1:f64=100.0;
    context.fill_rect(x1, y1 , 100.0,100.0);
   
    
            x1+=20.0;
            y1+=20.0;
            context.set_fill_style(&"#076ab0".into());
            context.fill_rect(y1, x1 , 100f64,100f64);
            console_log!("xxxxx yyyyy xxxxxx");
    
    // for i in 1..5{
    //     let temp=(i*100)as f64;
    //     context.fill_rect(temp, temp , 100.00, 100.00);
    // }
  
    // web_sys::console::log_2(&"Color : %s ".into(),&context.fill_style().into());




    Ok(())
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, movesquare!");
}

enum minionbehaviour{
    circular,
    airbubble,
}

#[wasm_bindgen]
struct minions{
    x:f64,
    y:f64,
    mx:f64,
    my:f64,
    speed:f64,
    otype:minionbehaviour,
    segement:u32
}

#[wasm_bindgen]
impl minions{
    pub fn new()->Self{
        let mut rng = rand::thread_rng();
        Self{
            x:rng.gen_range(0, 1360)as f64,
            y:rng.gen_range(0, 720)as f64,
            mx:0f64,
            my:0f64,
            speed:rng.gen_range(0, 100)as f64,
            otype:if rng.gen_range(1, 4)%2==0{minionbehaviour::airbubble}else{minionbehaviour::circular},
            segement:20u32
        }
    }

    pub fn moveblock(&mut self){  
        match (self.otype){
     minionbehaviour::circular=>{
        let theta = 2.0f32 * 3.1415926f32 *self.segement as f32 / 20f32;//get the current angle 
        self.segement+=1;
        if  self.segement>=20{
            self.segement=0;
        }
        self.mx = (self.speed as f32 * theta.cos()).into();//calculate the x component 
        self.mx+=self.x;
        self.my = (self.speed as f32 * theta.sin()).into();//calculate the y component
        self.my+=self.y; 
        },
        minionbehaviour::airbubble=>{
            self.my+=self.speed/10f64;
             self.mx=self.x;
            if(self.my>=720f64){
               
                self.my=0f64;
            }
        }
    }
       
    }

}

#[wasm_bindgen]
struct block{
    arr:Vec<minions>,
    window:web_sys::Window,
    canvas:web_sys::HtmlCanvasElement
}

#[wasm_bindgen]
impl block{
    pub fn new()->block{
        utils::set_panic_hook();
        let windows: web_sys::Window =web_sys::window().unwrap();
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let canvas = document.get_element_by_id("canvas").unwrap();
        let canvass: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();
        let mut arw=Vec::<minions>::new();
        for i in 0..30{
            arw.push(minions::new())
        }
       
        Self{
            arr:arw,
            window:windows,
            canvas:canvass
        }
    }

   pub fn moveblock(&mut self){
        // self.x+=10.0;
        // self.y+=10.0;
      for k in self.arr.iter_mut(){
        k.moveblock();
      }
       
    }

    pub fn draw_block(&self){
       
    
        
        let width=self.window.inner_width().unwrap();
        let height=self.window.inner_height().unwrap();
        let uwidth=width.as_f64().unwrap();
        let uheight=height.as_f64().unwrap();
        console_log!("createdddd module has {}  {} pages of memory",uwidth,uheight );
        let context = self.canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();
        
        for k in self.arr.iter(){
            if  let minionbehaviour::airbubble=k.otype{
            context.set_fill_style(&"#076ab0".into());
            }
            else{
                context.set_fill_style(&"#32a852".into());
            }
        // context.fill_rect(k.mx, k.my , 100f64,100f64);
        
        context.begin_path();
        // Draw the outer circle.
        context
        .arc(k.mx, k.my, 50.0, 0.0, std::f64::consts::PI * 2.0)
        .unwrap();

    
        context.fill();
    context.stroke();
    }

        
        
    }

    pub fn clear_background(&self){
   

    
        let width=self.window.inner_width().unwrap();
        let height=self.window.inner_height().unwrap();
        let uwidth=width.as_f64().unwrap();
        let uheight=height.as_f64().unwrap();
        console_log!("createdddd module has {}  {} pages of memory",uwidth,uheight );
        let context = self.canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();
        // context.width=windows.innerWidth;
    //context.set_fill_style(&"#0000FF".into());     
        context.set_fill_style(&"#000000".into());        
    
        context.fill_rect(0.0, 0.0, uwidth, uheight);
    }
}




