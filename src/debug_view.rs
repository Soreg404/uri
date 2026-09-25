use std::fmt::{ Debug, Formatter, Error };

impl Debug for super::Uri<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>{
        /*
         * todo: later: decoding path parts
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
         *
         */

        f.debug_struct("Uri")
            .field("scheme", &self.scheme.map(String::from_utf8_lossy))
            .field("authority", &self.authority)
            .field("path", &String::from_utf8_lossy(self.path))
            // todo:
            //.field("[[path decoded]]", &path_parts)
            .field("query", &self.query.map(String::from_utf8_lossy))
            // todo:
            //.field("[[query decoded]]", &"todo")
            .field("fragment", &self.fragment.map(String::from_utf8_lossy))
            .finish()
    }
}

impl Debug for super::UriAuthority<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>{
        f.debug_struct("Authority")
            .field("host", &String::from_utf8_lossy(&self.host))
            .field("port", &self.port)
            .finish()
    }
}

impl Debug for super::UriOpaque<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>{
        f.debug_struct("UriOpaque")
            .field("scheme", &String::from_utf8_lossy(&self.scheme))
            .field("path", &String::from_utf8_lossy(self.path))
            // todo:
            //.field("[[path decoded]]", &path_parts)
            // todo:
            //.field("query", &self.query.map(String::from_utf8_lossy))
            // todo:
            //.field("[[query decoded]]", &"todo")
            .field("fragment", &self.fragment.map(String::from_utf8_lossy))
            .finish()
    }
}
