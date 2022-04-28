use std::collections::HashMap;

use crate::types::*; 
use crate::camera;
use crate::assets;


#[derive(Clone)]
pub struct SceneData{
    pub text_sz: Vec2,
    pub plate_num: usize,
    pub plate_size: Vec2,
    pub timing: Vec<usize>,
    pub cycle: bool
}
impl SceneData{
    pub fn new(text_sz: Vec2, plate_num: usize, plate_size: Vec2,
         timing: Vec<usize>, cycle: bool) -> SceneData{
            SceneData{
                text_sz,
                plate_num,
                plate_size,
                timing,
                cycle
            }
        }

}

#[derive(Clone)]
pub struct GraphicSet{
    texture: assets::TextureRef,
    plates: Vec<Rect>,
}
impl GraphicSet{
    pub fn new(texture: assets::TextureRef, text_sz: Vec2, plate_num: usize, 
        plate_size: Vec2) -> GraphicSet{
            GraphicSet{
                texture,
                plates: Cutscene::load_plates(text_sz, plate_num, plate_size)
            }
        }

    pub fn get_texture(&self) -> assets::TextureRef{
        self.texture
    }
    pub fn get_plate(&self, plate_index: usize) -> Rect{
        assert!(self.plates.len()>plate_index);
        assert_ne!(plate_index, 0);
        
        self.plates[plate_index]
    }
}

#[derive(Clone)]
pub struct Cutscene{
    texture: assets::TextureRef,
    plates: Vec<Rect>,
    is_active: bool,
    frame_triggered: usize,
    cur_frame: usize,
    cur_plate: usize,
    timing: Vec<usize>,
    cycle: bool,
}

impl Cutscene{
    pub fn new(texture: assets::TextureRef, text_sz: Vec2, plate_num: usize, 
        plate_size: Vec2, timing: Vec<usize>, cycle: bool) -> Cutscene{
            Cutscene{
                texture,
                plates: Cutscene::load_plates(text_sz, plate_num, plate_size), 
                is_active: false,
                frame_triggered: 0,
                cur_frame: 0,
                cur_plate: 0,
                timing,
                cycle
            }
        }
    pub fn new_data(texture: assets::TextureRef, data: SceneData) -> Cutscene{    
        Cutscene{
                texture,
                plates: Cutscene::load_plates(data.text_sz, data.plate_num, data.plate_size), 
                is_active: false,
                frame_triggered: 0,
                cur_frame: 0,
                cur_plate: 0,
                timing: data.timing,
                cycle: data.cycle,
            }
        }

    fn load_plates(text_sz: Vec2, plate_num: usize, plate_size: Vec2) -> Vec<Rect>{
        let mut temp = Vec::new();
        let plates_across = text_sz.x / plate_size.x; 
        let plates_high = text_sz.y / plate_size.y; 

        let mut pos = Vec2::new(0.0,0.0);

        //checks that the image can actually hold that number
        assert!((plates_across*plates_high) as usize >= plate_num);

        for i in 0..plates_high as usize{
            for j in 0..plates_across as usize{
                pos.x = j as f32 * plate_size.x;
                pos.y = i as f32 * plate_size.y;

                if plate_num > temp.len() {
                    temp.push(Rect{pos, sz: plate_size});
                }
                else{
                    break;
                }
            }
        }
        temp
    }

    pub fn trigger(&mut self) -> ()
    {
        if !self.is_active
        {
            self.is_active = true;  
            self.frame_triggered = self.cur_frame;
        }
        
    }

    fn tick(&mut self) -> (){

        if  self.cur_frame - self.frame_triggered == self.timing[self.cur_plate]{
            self.cur_plate += 1; 
            self.frame_triggered = self.cur_frame;

            if self.cur_plate >= self.plates.len()
            {
                self.is_active = self.cycle;

                if self.is_active(){
                    self.cur_plate = 0;
                }
                else{
                    self.cur_plate = self.plates.len() -1;
                }
            }
        }
    }

    pub fn is_active(&self) -> bool{
        self.is_active
    }

    pub fn set_plate(&mut self, plate_index: usize) -> (){
        assert!(self.plates.len()>plate_index);

        self.cur_plate = plate_index;
    }

    pub fn incr_frame(&mut self) -> (){
       // dbg!(self.cur_frame, self.is_active, self.frame_triggered, self.timing[self.cur_plate]);
        self.cur_frame += 1;
        if self.is_active{
            self.tick();
        }
    }

    pub fn last_plate(&self) -> usize{
        self.plates.len()-1
    }

    pub fn get_texture_ref(&self) -> assets::TextureRef{
        self.texture
    }

    pub fn get_current_plate(&self) -> Rect{
        self.plates[self.cur_plate]
    }

    /*pub fn load_buffer(&mut self, fb2d:  &mut Image) -> ()
    {
        self.incr_frame();
        let rect = Rect{pos:Vec2::new(0,0), sz: self.plates[self.cur_plate].sz};
        fb2d.bitblt(&self.plates[self.cur_plate], rect, Vec2::new(0,0));
    }*/

}








