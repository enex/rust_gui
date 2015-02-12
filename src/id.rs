/// The id-type used internaly to identify each widget.
/// it is an array consistiong of 12 ids of the elements an their children
/// if the id == 0, the element has no id assigned. Thats why ids should start
/// from 1
pub type ID = [u16;12];

///The null id
pub const NULL_ID: ID = [0,0,0,0,0,0,0,0,0,0,0,0];
