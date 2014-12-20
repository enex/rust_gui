#[deriving(Copy)]
pub struct Tabs{
    pub color: (f64,f64,f64),
}

pub struct Tab{
    pub title: String,
}

pub enum TabsEvent{
    Switch(Tab) //event which is called if the tab is changed
}
