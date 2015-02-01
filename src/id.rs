pub type ID = [u16;12];

mod experiment{
    use std::iter::Iterator;
    #[derive(Copy, Clone, Show)]
    pub struct ID{
        data: [u8;15],
        end: u8
    }

    impl ID{
        pub fn new() -> ID{
            ID{
                data: [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
                end: 0
            }
        }
        ///push last component id back
        #[inline(always)]
        pub fn push(&mut self, i: u16){
            if self.end > 14{
                panic!("to many ids");
            }
            if i >= 32768{
                panic!("value given is to big to be stored, limit the number of components used");
            }
            if i < 128{
                self.data[self.end as usize] = i as u8;
                self.end += 1;
            }else{
                self.data[self.end as usize] = (i >> 1) as u8;
                self.data[(self.end +1) as usize] = (i >> 6) as u8;
                self.end += 2;
            }
        }

        ///get an iterator over all methods
        #[inline(always)]
        pub fn iter<'a>(&'a self) -> ID_Iterator<'a>{
            ID_Iterator{
                id: self,
                pos: 0
            }
        }
    }

    pub struct ID_Iterator<'a>{
        id: &'a ID,
        pos: u8,
    }
    impl<'a> Iterator for ID_Iterator<'a>{
        type Item = u16;
        fn next(&mut self) -> Option<u16>{
            None
        }
    }

    /*#[test]
    fn test_ID(){
        let mut id = ID::new();
        id.push(127);
        id.push(12);
        id.push(1023);
        assert!(id.end == 4, "end has wrong size");
        assert!(id.data[0] == 0b01111111, "wrong number {}", 0b01111111);
        assert!(id.data[1] == 12);
        assert!(id.data[2] == 0b11111111, "wront it is {}", id.data[2]);
        assert!(id.data[3] == 0b11100000, "wront it is {}", id.data[3]);
    }*/
}
