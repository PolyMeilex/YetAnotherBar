use xcb_util::ewmh;

#[derive(Debug)]
pub struct EwmhEvent {
    pub num_desktops: u32,
    pub current_desktop: u32,
    pub desktop_names: Vec<String>,
    pub desktop_monitors: Vec<Option<RandrMonitor>>,
}

#[derive(Debug, Clone)]
pub struct RandrMonitor {
    pub name: String,
    pub x: i16,
    pub y: i16,
    pub w: u16,
    pub h: u16,
}

fn on_change(
    ewmh_conn: &ewmh::Connection,
    screen_idx: i32,
    randr_monitors: &Vec<RandrMonitor>,
) -> EwmhEvent {
    let num_desktops = ewmh::get_number_of_desktops(&ewmh_conn, screen_idx)
        .get_reply()
        .unwrap();

    let current_desktop = ewmh::get_current_desktop(&ewmh_conn, screen_idx)
        .get_reply()
        .unwrap();
    let list = ewmh::get_desktop_names(&ewmh_conn, screen_idx)
        .get_reply()
        .unwrap();
    let desktop_names: Vec<String> = list.strings().iter().map(|s| s.to_string()).collect();

    let viewport = ewmh::get_desktop_viewport(&ewmh_conn, screen_idx)
        .get_reply()
        .unwrap();

    let desktop_monitors: Vec<Option<RandrMonitor>> = viewport
        .desktop_viewports()
        .iter()
        .map(|t| {
            let x = t.x;
            let y = t.y;

            let m = randr_monitors
                .iter()
                .find(|m| m.x as u32 == x && m.y as u32 == y);
            if let Some(m) = m {
                Some(m.to_owned())
            } else {
                None
            }
        })
        .collect();

    return EwmhEvent {
        num_desktops,
        current_desktop,
        desktop_names,
        desktop_monitors,
    };
}

pub fn init(sender: relm::Sender<EwmhEvent>) {
    let (xcb_conn, screen_idx) = xcb::Connection::connect(None).unwrap();

    let root_window = xcb_conn
        .get_setup()
        .roots()
        .nth(screen_idx as usize)
        .unwrap()
        .root();

    let randr_monitors: Vec<RandrMonitor> = {
        let t = xcb::randr::get_screen_resources_current(&xcb_conn, root_window)
            .get_reply()
            .unwrap();
        let ops = t.outputs();

        let mut monitors = Vec::new();
        for op in ops.iter() {
            let a = xcb::randr::get_output_info(&xcb_conn, *op, xcb::CURRENT_TIME)
                .get_reply()
                .unwrap();
            let crtc =
                xcb::randr::get_crtc_info(&xcb_conn, a.crtc(), xcb::CURRENT_TIME).get_reply();

            if let Ok(crtc) = crtc {
                let name = String::from_utf8(a.name().into()).unwrap();
                let x = crtc.x();
                let y = crtc.y();

                let w = crtc.width();
                let h = crtc.height();

                monitors.push(RandrMonitor { name, x, y, w, h })
            }
        }
        monitors
    };

    let ewmh_conn = ewmh::Connection::connect(xcb_conn).ok().unwrap();

    let ewmh_conn = std::rc::Rc::new(ewmh_conn);

    let attributes = [(xcb::CW_EVENT_MASK, xcb::EVENT_MASK_PROPERTY_CHANGE)];
    xcb::change_window_attributes(&ewmh_conn, root_window, &attributes);
    ewmh_conn.flush();

    // Initial Update
    sender
        .send(on_change(&ewmh_conn, screen_idx, &randr_monitors))
        .expect("ewmh send");

    let properties = [
        ewmh_conn.NUMBER_OF_DESKTOPS(),
        ewmh_conn.CURRENT_DESKTOP(),
        ewmh_conn.DESKTOP_NAMES(),
    ];
    loop {
        let event = ewmh_conn.wait_for_event().unwrap();

        use xcb::xproto::{PropertyNotifyEvent, PROPERTY_NOTIFY};
        if event.response_type() == PROPERTY_NOTIFY {
            let event: &PropertyNotifyEvent = unsafe { xcb::cast_event(&event) };
            if properties.iter().any(|p| *p == event.atom()) {
                sender
                    .send(on_change(&ewmh_conn, screen_idx, &randr_monitors))
                    .expect("ewmh send");
            }
        }
    }
}
