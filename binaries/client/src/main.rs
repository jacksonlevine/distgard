

use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::fmt::Subscriber;
use tracing::{error, info};
use std::env;
use std::fs::File;

use voxelland::windowandkey::{WindowAndKeyContext, AM_I_A_FUCKING_SERVER, UNCAPKB};

use voxelland::game::{Game, DECIDEDSEEDOREXISTS, DECIDEDSPORMP, DECIDEDWORLD, HEADLESS, RECEIVED_WORLD, SHOULDRUN, SINGLEPLAYER};

pub static mut ISSERVER: bool = false;

fn main() {

    
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        if args[1] == "s" {
            println!("I AM SERVER");
            unsafe { ISSERVER = true };
            unsafe { AM_I_A_FUCKING_SERVER = true };
        }
    }


    if unsafe { ISSERVER } {

        
       

        let mut wak = WindowAndKeyContext::new("Server", 100, 100);
        println!("Got past this point");
        let gameh = Game::new(&wak.window, true, true, &wak.addressentered, &wak.serveraddress);
        let mut game = gameh.join().unwrap();
        println!("Got past this point2");
        game.initialize_being_in_world();
        println!("Got past this point3");
        wak.game = Some(game);
        
        while !wak.window.read().should_close() {
            wak.glfw.poll_events();
        }



        






    } else {

    

    // Create a non-blocking, asynchronous file writer
    let file = File::create("app.log").expect("Unable to create log file");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file);

    // Create a tracing subscriber with the non-blocking writer
    let subscriber = Subscriber::builder()
        .with_writer(non_blocking)
        .with_span_events(FmtSpan::CLOSE)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    // Capture the default panic hook
    let default_hook = std::panic::take_hook();

    // Set a custom panic hook
    std::panic::set_hook(Box::new(move |panic_info| {
        // Call the default hook to print the panic message to stderr
        default_hook(panic_info);

        // Log the panic message
        if let Some(location) = panic_info.location() {
            error!(
                "Panic occurred at {}:{}:{} -- {}",
                location.file(),
                location.line(),
                location.column(),
                panic_info
            );
        } else {
            error!("Panic occurred: {}", panic_info);
        }
    }));

    

    let mut wak_context = WindowAndKeyContext::new("Distant Garden", 1280, 720);

    unsafe {
        while !DECIDEDSPORMP {
            if !wak_context.window.read().should_close() {
                wak_context.run();
            } else {
                return ();
            }
        }



        if SINGLEPLAYER {

            while !DECIDEDWORLD {
                if !wak_context.window.read().should_close() {
                    wak_context.run();
                } else {
                    return ();
                }
            }

            while !DECIDEDSEEDOREXISTS {
                if !wak_context.window.read().should_close() {
                    wak_context.run();
                } else {
                    return ();
                }
            }

        } else {

            //MULTIPLAYER

            







        }


        

        
    }
    

    

    let gameh = Game::new(&wak_context.window, true, unsafe { HEADLESS }, &wak_context.addressentered, &wak_context.serveraddress);

    while !gameh.is_finished() {
        if !wak_context.window.read().should_close() {
            wak_context.run();
        } else {
            return ();
        }
    }


    let game: Game;

    match gameh.join() {
        Ok(gamei) => {
            game = gamei;
        }
        Err(_e) => {
            panic!("Failed to create Game.");
        }
    }

    info!("gltf model count: {}", game.gltf_models.len());

    wak_context.game = Some(game);

    let handle = wak_context.game.as_mut().unwrap().initialize_being_in_world();

    while !handle.is_finished() {
        if !wak_context.window.read().should_close() {
            wak_context.run();
        } else {
            return ();
        }
    }

    match handle.join() {
        Ok(_) => {
            wak_context.game.as_mut().unwrap().loadedworld.store(true, std::sync::atomic::Ordering::Relaxed);
        }
        Err(_e) => {

        }
    }
    wak_context.game.as_mut().unwrap().vars.menu_open = false;
    
    wak_context.game.as_mut().unwrap().start_world();
    wak_context.game.as_mut().unwrap().set_mouse_focused(true);
    wak_context.game.as_mut().unwrap().window.write().set_cursor_mode(glfw::CursorMode::Disabled);
    unsafe {
        UNCAPKB.store(true, std::sync::atomic::Ordering::Relaxed);
    }
    
    while !wak_context.window.read().should_close() {
        wak_context.run();
    }

    unsafe { SHOULDRUN = false; }
    }
}
