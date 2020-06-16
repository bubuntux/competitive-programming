struct BrowserHistory {
    history: Vec<String>,
    index: usize,
}

impl BrowserHistory {
    fn new(homepage: String) -> Self {
        BrowserHistory { history: vec![homepage], index: 0 }
    }

    fn visit(&mut self, url: String) {
        self.index += 1;
        self.history.truncate(self.index);
        self.history.push(url);
    }

    fn back(&mut self, steps: i32) -> String {
        let aux = self.index as i32 - steps;
        if aux <= 0 {
            self.index = 0;
        } else {
            self.index = aux as usize;
        }
        self.history[self.index].clone()
    }

    fn forward(&mut self, steps: i32) -> String {
        let aux = self.index as i32 + steps;
        if aux >= self.history.len() as i32 {
            self.index = self.history.len() - 1;
        } else {
            self.index = aux as usize;
        }
        self.history[self.index].clone()
    }
}

// https://leetcode.com/problems/design-browser-history/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let mut browser = BrowserHistory::new(String::from("leetcode.com"));
        browser.visit(String::from("google.com"));
        browser.visit(String::from("facebook.com"));
        browser.visit(String::from("youtube.com"));

        assert_eq!(String::from("facebook.com"), browser.back(1));
        assert_eq!(String::from("google.com"), browser.back(1));
        assert_eq!(String::from("facebook.com"), browser.forward(1));

        browser.visit(String::from("linkedin.com"));

        assert_eq!(String::from("linkedin.com"), browser.forward(2));
        assert_eq!(String::from("google.com"), browser.back(2));
        assert_eq!(String::from("leetcode.com"), browser.back(7));
    }
}
