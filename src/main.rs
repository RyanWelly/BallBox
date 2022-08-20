use macroquad::{prelude::*, color};
use macroquad::rand;

const BOX_LEVEL: f32 = 450f32; //maybe define in terms of screen dimensions at runtime
const BOX_HEIGHT: f32 = 5f32;
const BOX_WIDTH: f32 = 40f32;
const PLAYER_SPEED: f32 = 10f32;
const PLAYER_SIZE: Vec2 = const_vec2!([30f32, 50f32]);
const GROUND_LEVEL: f32 = 400f32;
const BALL_SIZE: f32 = 30f32; //Play around with different sizes
const BALL_START_HEIGHT: f32 = 100f32; //play around with different sizes

struct Player {
    rect: Rect,
    orient: Orientation,
    which_box: u8,
}

impl Player {

    pub fn new(location: u8) -> Self {
        Self { 
            rect: Rect::new(
                screen_width() * 0.5f32,
                GROUND_LEVEL,
                PLAYER_SIZE.x,
                PLAYER_SIZE.y
            ),
            orient: Orientation::Right,
            which_box: location, 
        } 
    }
    pub fn draw(&self) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, BLUE);
    }

    pub fn update(&mut self, dt: f32, boxes: &Vec<GameBox>) {
        let mut x_move = 0f32;

        if is_key_down(KeyCode::Left) {
            x_move -= 30f32;
        } 
        if is_key_down(KeyCode::Right) {
            x_move += 30f32;
        }

        self.rect.x += x_move * dt * PLAYER_SPEED; //change this so that it doesn't move smoothly, it just jumps to being above each box.
        if self.rect.x < 0f32 {
            self.rect.x = 0f32;
        }

        

        if self.rect.x > screen_width() {
            self.rect.x = screen_width() - self.rect.w;
        }
    }
}

struct Ball {
    circle: Circle,
    color: BoxColor,
}

impl Ball {
    pub fn new(x: f32, color: BoxColor) -> Self {
        //TODO: change this to a global rng generator
        
        let random_colour = if rand::gen_range(0, 1) == 1 {BoxColor::Green} else {BoxColor::Purple};
        Self { circle: Circle { x: x, y: BALL_START_HEIGHT, r: BALL_SIZE }, color: random_colour }
    }

    pub fn update(&mut self, dt: f32) {
        //maybe take into account angle shooting

        
    }
    
}

enum BoxColor {
    Purple, 
    Green,
}

enum Orientation {
    Left, 
    Right,
}

fn get_color(boxcolor : &BoxColor) -> Color {
    match boxcolor {
        BoxColor::Purple => PURPLE,
        BoxColor::Green => GREEN,
    }
}
struct GameBox {
    x: f32,
    rect: Rect,
    box_type: BoxColor,
}

impl GameBox {

    pub fn new(x: f32, boxcolor : BoxColor) -> Self {
        Self { x: x, rect: Rect::new(
            x,
            BOX_LEVEL,
            BOX_WIDTH, 
            BOX_HEIGHT,
        ), box_type: boxcolor }
    }

    pub fn switch(&mut self) {
        match  &self.box_type{
            BoxColor::Purple => self.box_type = BoxColor::Green,
            BoxColor::Green => self.box_type = BoxColor::Purple,
        }
    }

    pub fn draw(&self) {
        let color = get_color(&self.box_type);
        draw_rectangle(self.x, BOX_LEVEL + 100f32, BOX_WIDTH, BOX_HEIGHT, color) //replace with Box rect parameters
    }
}


#[macroquad::main("BallBox")]
async fn main() {
    //intialise
    let mut player = Player::new(7);
    let mut Boxes: Vec<GameBox> = Vec::new();
    for i in 0..=8 {
        let boxcolor;
        if i % 2 == 0 {boxcolor = BoxColor::Green} else {boxcolor = BoxColor::Purple}
        let new_box: GameBox = GameBox::new({i+ 1} as f32 * 2f32 * BOX_WIDTH, boxcolor);
        Boxes.push(new_box);
    }

    //game Loop

    loop{
        //Updating world
        player.update(get_frame_time(), &Boxes);
        for b in &mut Boxes {
            if let Some(_) = player.rect.intersect(b.rect) {
                if is_key_pressed(KeyCode::Space) {
                    b.switch();
                    break;
                }
            }
                
        }

        //drawing
        clear_background(BLACK);
        for b in &Boxes {
            b.draw();
        }
        player.draw();
        next_frame().await;
    }
}
