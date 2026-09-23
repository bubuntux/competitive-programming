// https://rustcurious.com/4

// Add fields to track the number of likes and dislikes
// on the video, and add methods so the tests pass.
//
// The tests expect the following methods:
//
// - add_like() -- increment the number of likes
// - add_dislike() -- increment the number of dislikes
// - likes_dislikes() -- return a tuple (likes, dislikes)
// - like_ratio() -- return likes / (likes + dislikes)

pub struct Video {
    title: String,
    likes: usize,
    dislikes: usize,
}

impl Video {
    pub fn new(title: String) -> Video {
        Video {
            title,
            likes: 0,
            dislikes: 0,
        }
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn add_like(&mut self) {
        self.likes += 1;
    }

    pub fn add_dislike(&mut self) {
        self.dislikes += 1;
    }

    pub fn likes_dislikes(&self) -> (usize, usize) {
        (self.likes, self.dislikes)
    }

    pub fn like_ratio(&self) -> f64 {
        self.likes as f64 / (self.likes + self.dislikes) as f64
    }
}

// -------------------------------------------------------
// No need to change anything below this line.

#[test]
fn test() {
    let mut video = Video::new(String::from("Structs"));
    assert_eq!(video.title(), "Structs");

    video.add_like();
    video.add_like();
    video.add_like();
    video.add_dislike();
    assert_eq!(video.likes_dislikes(), (3, 1));
    assert_eq!(video.like_ratio(), 0.75);
}

// This test checks that likes_dislikes and like_ratio
// can be called on an immutable video object. This will
// fail if those method declarations take `&mut self`.
#[test]
fn test_immutable() {
    let video = Video::new(String::from("Structs"));
    assert_eq!(video.likes_dislikes(), (0, 0));
    assert!(video.like_ratio().is_nan());
}
