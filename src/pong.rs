use raylib::prelude::*;

const PLAYER_WIDTH: f32 = 40.0;
const PLAYER_HEIGHT: f32 = 300.0;
const BALL_RADIUS: f32 = 20.0;
fn movement(rl: &mut RaylibHandle,
            dt: f32,
            player1: &mut Rectangle, 
            player2: &mut Rectangle) {
    
    if rl.is_key_down(KeyboardKey::KEY_W) {
        player1.y -= 300.0*dt
    }

    if rl.is_key_down(KeyboardKey::KEY_S) {
        player1.y += 300.0*dt
    }

    if rl.is_key_down(KeyboardKey::KEY_UP) {
        player2.y -= 300.0*dt

    }
    if rl.is_key_down(KeyboardKey::KEY_DOWN) {
        player2.y += 300.0*dt
    }
}

fn ball_movement(rl: &mut RaylibHandle,
                 dt: f32,
                 ball_position: &mut Vector2,
                 ball_speed: &mut Vector2,
                 player1: &Rectangle,
                 player2: &Rectangle) {
    ball_position.x += ball_speed.x * dt;
    ball_position.y += ball_speed.y * dt;

    if (ball_position.x >= rl.get_screen_width() as f32 - BALL_RADIUS) 
    || (ball_position.x <= BALL_RADIUS) {
          ball_speed.x *= -1.0
    }
    
    if player1.check_collision_circle_rec(*ball_position,BALL_RADIUS)
    ||  player2.check_collision_circle_rec(*ball_position, BALL_RADIUS){
        ball_speed.x *= -1.0
    } 
    // if ( ball_position.x - player2.x + BALL_RADIUS <= 0.01)
    //     &(ball_position.y >= player2.y)
    //     & (ball_position.y <= player2.y + PLAYER_HEIGHT) 
    //       {ball_speed.x *= -1.0}
    
    println!("{:#?} , {:#?}", player1.y.round(), ball_position.x.round());
    if (ball_position.y >= rl.get_screen_height() as f32 - BALL_RADIUS) 
    || (ball_position.y <= BALL_RADIUS) 
          {ball_speed.y *= -1.0}
}

pub fn pong() {
    let (screen_width, screen_height) = (1300.0,900.0);
    let (mut rl, thread) = raylib::init()
        .size(screen_width as i32, screen_height as i32)
        .title("hello raylib")
        .build();
    
    let mut player1: Rectangle = Rectangle {
        height: PLAYER_HEIGHT,
        width: PLAYER_WIDTH,
        x: 200.0,
        y: (screen_height)/2.0 - PLAYER_HEIGHT/2.0 
    };
    let mut ball_position = Vector2::new(
                            screen_height/2.0,
                            screen_width/2.0);
    
    let mut ball_speed = Vector2::new(400.0,400.0);

    let mut player2: Rectangle = Rectangle {
        height: PLAYER_HEIGHT,
        width: PLAYER_WIDTH,
        x: (screen_width) - 200.0,
        y: (screen_height)/2.0 - PLAYER_HEIGHT/2.0
    };
    // let mut camera = Camera2D {
    //     offset: Vector2 {x: 200.0, y: 200.0},
    //     target: Vector2 {x: player.x,y: player.y},
    //     rotation: 0.0,
    //     zoom: 1.0
    // };
    rl.set_target_fps(60);
    // Main loop
    loop {
        let dt = rl.get_frame_time();
        let frame = rl.get_fps();
        
        // Movement
        movement(&mut rl, dt, &mut player1, &mut player2);

        ball_movement(&mut rl, dt, &mut ball_position,
                      &mut ball_speed, &player1, &player2);

        let mut mode2d = rl.begin_drawing(&thread);

        mode2d.clear_background(Color::BEIGE);
        mode2d.draw_circle_v(&ball_position, BALL_RADIUS, Color::ORANGE);
        mode2d.draw_rectangle_rec(&player1,Color::RED);
        mode2d.draw_rectangle_rec(&player2,Color::WHITE);
        
        let pos = vec![player1.x, player1.y, player2.x, player2.y];
        let postxt = format!("pos {:#?}", pos);
        let fps = format!("FPS {frame}");
        mode2d.draw_text(&postxt,200,400,20,Color::BLACK);
        mode2d.draw_text(&fps,150,300,30,Color::BLUE);
        mode2d.draw_text("hello",100,200,20,Color::BLACK)
    }

}
