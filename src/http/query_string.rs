use std::collections::HashMap;
use std::convert::From;
pub struct QueryString<'buf> {
  data: HashMap<&'buf str, Value<'buf>>
}

pub enum Value<'buf > {
  Single(&'buf str),
  Multiple(Vec<&'buf str>),
}

impl<'buf > QueryString<'buf > {
  fn get(&self, key: &str) -> Option<&Value> {
    self.data.get(key)
  } 
}

// a=1&b=2&c&d=&e===&d=7&d=abc
impl<'buf> From<&'buf str> for QueryString<'buf> {
  fn from(s: &str) -> Self {
    let mut data = HashMap::new();

    for sub_str in s.split('&') {
      let mut key = sub_str;
      let mut val = "";
      if let Some(i) = sub_str.find('=') {
        key = &sub_str[..i];
        val = &sub_str[i+1..];
      }
      data.entry(key)
      .and_modify(|existing: &mut Value| match existing {
        Value::Single(pre_val) => {
          *existing = Value::Multiple(vec![pre_val, val]);
        },
        Value::Multiple(vec) => vec.push(val)

      })
      .or_insert(Value::Single(val));
    }
    QueryString { data };
    unimplemented!()
  }
}