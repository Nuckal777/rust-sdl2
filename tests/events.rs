extern crate sdl2;
use sdl2::event;

fn main() {
    let sdl = sdl2::init().unwrap();
    let ev = sdl.event().unwrap();
    let mut ep = sdl.event_pump().unwrap();

    test1(&ev);
    test2(&ev, &mut ep);
}

fn test1(ev: &sdl2::EventSubsystem) {
    let user_event1_id = ev.register_event().unwrap();
    let user_event2_id = ev.register_event().unwrap();

    assert!(user_event1_id != user_event2_id);
}

fn test2(ev: &sdl2::EventSubsystem, ep: &mut sdl2::EventPump) {
    let user_event_id = ev.register_event().unwrap();

    let event = event::Event::User {
        timestamp: 0,
        window_id: 0,
        type_: user_event_id,
        code: 456,
        data1: 0x1234 as *mut ::sdl2::libc::c_void,
        data2: 0x5678 as *mut ::sdl2::libc::c_void,
    };

    let (t1, a1, a2) = match event {
        event::Event::User { type_: t1, data1: a1, data2: a2, .. } => { (t1, a1, a2) }
        _ => { panic!("expected user event") }
    };

    ev.push_event(event.clone()).unwrap();

    let received = ep.poll_event().unwrap();

    assert_eq!(&event, &received);
    match &received {
        &event::Event::User { type_: t2, data1: b1, data2: b2, .. }  => {
            assert_eq!(t1, t2);
            assert_eq!(a1, b1);
            assert_eq!(a2, b2);
        }
        other => { panic!("Received non User event: {:?}", other) }
    }
}
