static mut Move_speed : f32 = 1000.0;
static mut Profile : u16 = 0;

redhook::hook! {
    unsafe fn _ZN6Player12GetJumpSpeedEv(_x: *const ()) -> f32 => jump_high {        
        1000.0
    }
}

redhook::hook! {
    unsafe fn _ZN6Player15GetWalkingSpeedEv(_x: *const ()) -> f32 => walk_fast {
        Move_speed
    }
}
redhook::hook! {
    unsafe fn _ZN6Player15GetJumpHoldTimeEv(_x: *const ()) -> f32 => jump_hold {
        1000.0
    }
}

redhook::hook! {
    unsafe fn _ZN6Player7CanJumpEv(_x: *const ()) -> bool => can_jump {
        true
    }
}

//Notice: GAME LOOP
redhook::hook! {
    unsafe fn _ZN6Player4TickEf(this: *const (),delta_time: f32) => game_loop {
        redhook::real!(_ZN6Player4TickEf)(this,delta_time);
    }
}

//_ZN8ZeroCool15SpawnProjectileEP6Player
redhook::hook! {
    unsafe fn _ZN18ZeroCoolProjectileC2EP6IActor(this: *const (), y: *const ()) => change_profile {
        redhook::real!(_ZN18ZeroCoolProjectileC2EP6IActor)(this,y);
        Profile = Profile +1;
        match Profile{
            1 => {
                Move_speed = 500.0;
            }
            2 => {
                Move_speed = 1000.0;
            
            }
            3 => {
                Move_speed = 5000.0;
                Profile = 1;
            }
            
            _ => {}
        }

    }
}