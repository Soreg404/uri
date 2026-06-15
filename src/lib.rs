
// https://site.pl:80/path/to/res?query=string#fragment
// scheme  host    port  path     query-string fragment

struct UrlParser {
   exp_scheme: Option<bool>,
   exp_host: Option<bool>,
   exp_frag: Option<bool>,
}

impl UrlParser {
   pub fn new_auto() -> Self {
       Self {
           exp_scheme: None,
           exp_host: None,
           exp_frag: None,
       }
   }
   pub fn new_expect_scheme_host() -> Self {
       Self {
           exp_scheme: Some(true),
           exp_host: Some(true),
           exp_frag: None,
       }
   }
   pub fn new_expect_host_only() -> Self {
       Self {
           exp_scheme: Some(false),
           exp_host: Some(true),
           exp_frag: None,
       }
   }
   pub fn new_expect_path_only() -> Self {
       Self {
           exp_scheme: Some(false),
           exp_host: Some(false),
           exp_frag: None,
       }
   }

   pub fn expect_no_fragment(&mut self) -> &mut Self { self.exp_frag = Some(false); }
   pub fn maybe_relative_path(&mut self) -> &mut Self {
       // assert_neq!(self.exp_host, Some(true), 
       //    "can't have relative path when expecting host");
       todo!()
   }
   
   pub fn parse_index_only(&self, bytes: &[u8]) -> UrlIndexed {
       let mut idx = 0;
       enum Part: usize {
           Scheme,
           Host,
           Path,
           Rest
       }
       let mut part = Part::Scheme;
       while idx < bytes.len() {
           let c = bytes[idx];
          // check invalid char
          
           if part < Path {
               if c == b'.' {
                   part = Part::Host;
                   continue;
               }
               if c == b':' {

               }
           }
       }
   }
   pub fn parse(&self, bytes: &[u8]) -> Url {
       // self.parse_index_only(bytes).get_as_ref()
       todo!()
   }
}

struct Rdx {
    from: usize,
    to: usize
}
impl Rdx {
    fn new(from: usize, to: usize) -> Self {
        assert!(from <= to);
        Self {
            from,
            to
        }
    }
    fn len(&self) -> usize {
        self.to - self.from
    }
    fn as_slice_of(&self, bytes: &[u8]) -> &[u8] {
        &bytes[self.from..self.to]
    }
    fn translate(&mut self, offset: usize) -> &mut Self {
        self.from += offset;
        self.to += offset;
        self
    }
}

pub struct UrlIndexed {
    scheme: Option<Scheme>,
    host: Rdx,
    port: Option<u16>,
    path: Rdx,
    query: Option<Rdx>,
    fragment: Option<Rdx>
}

struct Url<'a> {

}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Scheme {
    HTTP,
    HTTPS
}
