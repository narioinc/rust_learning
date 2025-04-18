//trait is public so that consuming clients can invoke this against the structs impl this
pub trait Summary {
    fn summarize(&self) -> String;

    /*fn summaryCount(&self) -> i32 {
        let mut count = 0;
        for c in self.summarize().chars() {
            if c == ' ' {
                count += 1;
            }
        }
        count
    }*/
}

pub trait Display {
    fn display(&self) -> String {
        String::from("No summary available")
    }
}

impl Display for dyn Summary {
    fn display(&self) -> String {
        format!("Summary: {}", self.summarize())
    }
}


// a new article struct
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

//impl for new article which implements the Summary trait
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}: {}", self.headline, self.content)
    }

   
}

impl Display for NewsArticle {
    fn display(&self) -> String {
        format!("Summary: {} ", self.summarize())
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

//impl for tweet which implements the Summary trait
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}


//enums can also implement traits
pub enum MessageBoard {
    ANNOUNCEMENTS (String),
    AWARDS (String),
    
}

//nhere we implement the summary trait for the enum Messageboard
impl Summary for MessageBoard {
 fn summarize(&self) -> String {
        match self {
            MessageBoard::ANNOUNCEMENTS(msg) => format!("Announcement: {}", msg),
            MessageBoard::AWARDS(msg) => format!("Award: {}", msg),
        }
    }
}

//we can now let the parameters of fucntions to be objects of structs that "implement" the summary trait 
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
