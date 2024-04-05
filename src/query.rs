use crate::error::{Error, parse_error};
use crate::toml::{decode_string, decode_pathbuf, decode_vecstr};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use toml::Value;

#[allow(unused)]
#[derive(Debug)]
struct Query {
    id: String,
    template: PathBuf,
    conds: Vec<String>,
    output: Option<PathBuf>,
}

impl Query {

    fn decode<P: AsRef<Path>>(
        templates_base_dir: P,
        output_base_dir: P,
        value: &Value
    ) -> Result<Self, Error> {
        match value.as_table() {
            Some(t) => {
                let id = t.get("id")
                    .ok_or(parse_error!("Missing 'id' in 'query' entry"))
                    .map(decode_string)??;
                let template = t.get("template")
                    .ok_or(parse_error!("Missing 'template' in 'query' entry"))
                    .map(|v| decode_pathbuf(v, Some(templates_base_dir.as_ref())))??;
                let conds = t.get("conds")
                    .ok_or(parse_error!("Missing 'conds' in 'query' entry"))
                    .map(decode_vecstr)??;
                let output = match t.get("option") {
                    Some(v) => Some(decode_pathbuf(v, Some(output_base_dir.as_ref()))?),
                    None => None
                };
                Ok(Self { id, template, conds, output })
            },
            None => Err(parse_error!("Invalid 'query' entry"))
        }
    }
}

#[allow(unused)]
#[derive(Debug)]
pub struct Queries {
    inner: Vec<Rc<Query>>,
    cache: HashMap<String, Rc<Query>>,
}

impl Queries {

    pub fn new() -> Self {
        let inner: Vec<Rc<Query>> = vec![];
        let cache: HashMap<String, Rc<Query>> = HashMap::new();
        Self { inner, cache }
    }

    pub fn decode<P: AsRef<Path>>(
        templates_base_dir: P,
        output_base_dir: P,
        value: &Value
    ) -> Result<Self, Error> {
        let items = match value.as_array() {
            Some(xs) => {
                let mut res = Vec::with_capacity(xs.len());
                for x in xs {
                    let q = Query::decode(&templates_base_dir, &output_base_dir, x)?;
                    res.push(Rc::new(q));
                }
                res
            }
            None => return Err(parse_error!("Invalid queries"))
        };
        let cache: HashMap<String, Rc<Query>> = HashMap::new();
        Ok(Self { inner: items, cache })
    }
}
