use regex::Regex;

pub struct Route {
    name: String,
    grade: String,
    crag: String,
}

impl Route {
    pub fn new(name: String, grade: String, crag: String) -> Result<Self, &'static str> {
        let valid_yds = Regex::new(r"^5\.([0-9]|1[0-5][a-d])$").expect("regex should compile");

        if !valid_yds.is_match(&grade) {
            return Err("grade must be in YDS with no pluses, minuses, or slashes");
        }

        Ok(Self { name, grade, crag })
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_grade() {
        let invalid_grades = [
            "5.9+".to_string(),
            "5.10".to_string(),
            "5.11a/b".to_string(),
            "5.12-".to_string(),
        ];

        for invalid_grade in invalid_grades {
            let result = Route::new(
                "Some Route".to_string(),
                invalid_grade,
                "Some Crag".to_string(),
            );

            assert!(result.is_err());
        }
    }

    #[test]
    fn valid_grade() {
        let valid_grades = [
            "5.0".to_string(),
            "5.9".to_string(),
            "5.10a".to_string(),
            "5.11d".to_string(),
        ];

        for valid_grade in valid_grades {
            let result = Route::new(
                "Some Route".to_string(),
                valid_grade,
                "Some Crag".to_string(),
            );

            assert!(result.is_ok());
        }
    }
}
