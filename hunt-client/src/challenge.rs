/// e.g.
/// {
///   "numbers": "string-of-characters",
///   "sorted_indices": [4, 2, 1, 0, 3],
/// }
#[derive(Debug, serde::Deserialize)]
pub struct Challenge {
    /* FILL ME */
}

impl Challenge {
    pub fn sorted(self) -> Sorted {
        /* 1. make collection of bytes from String */
        /* 2. sort collection */
        /* 3. result in Sorted struct */

        /* STRETCH GOAL: speed up sorting! */
        Sorted {
            numbers: FILL_ME,
            sorted_indices: self.sorted_indices,
        }
    }
}

#[derive(Debug)]
pub struct Sorted {
    /* FILL THIS: what type will sorted produce for numbers? */
    pub numbers: FILL_ME,
    pub sorted_indices: Vec<usize>,
}

impl Sorted {
    pub fn answer(&self) -> Result<String, anyhow::Error> {
        let mut res = String::new();
        /* 1. get number at each provided index */
        /* 2. convert number to char */
        /* 3. add char to res */
        Ok(res)
    }
}

