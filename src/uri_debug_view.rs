use std::fmt::{ Debug, Formatter, Error };

impl Debug for super::Uri<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>{
        fn helper(s: &[u8]) -> &str {
            str::from_utf8(s)
                .expect("URI Parser is supposed to allow only ASCII characters")
        }

        let mut b1 = Vec::new();
        b1.resize(1000, 0u8);
        let mut b2 = Vec::new();
        b2.resize(10000, 0u8);
        let mut b3 = Vec::new();
        b3.resize(100, 0usize);
        let path_parts = {
            match self.get_decoded_path(&mut b1, &mut b2, &mut b3) {
                Err(()) => Err(()),
                Ok(v) => {
                    let v = v.path_parts;
                    let mut ret = Vec::new();
                    for p in v.iter() {
                        ret.push(String::from_utf8_lossy(p).to_string());
                    }
                    Ok(ret)
                }
            }
        };

        f.debug_struct("Uri")
            .field("scheme", &self.scheme.map(helper))
            .field("host", &self.host_raw.map(helper))
            .field("port", &self.port)
            .field("path_raw", &helper(self.path_raw))
            .field("[[path decoded]]", &path_parts)
            .field("query_raw", &self.query_raw.map(helper))
            .field("[[query decoded]]", &"todo")
            .field("fragment", &self.fragment_raw.map(helper))
            .finish()
    }
}

