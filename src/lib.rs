pub struct Route {
    name: String,
    grade: String,
    crag: String,
}

impl Route {
    pub fn new(name: String, grade: String, crag: String) -> Self {
        // TODO: Validate grade
        Self { name, grade, crag }
    }
}

pub struct Ascent {
    route: Route,
    // TODO: Add date
}

impl Ascent {
    pub fn new(route: Route) -> Self {
        // TODO: Once added, validate date
        Self { route }
    }
}

pub fn log_ascent(ascent: Ascent) {
    println!(
        "Logged {} {} at {}",
        ascent.route.name, ascent.route.grade, ascent.route.crag,
    );
}
