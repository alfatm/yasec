pub struct Context {
    prefix: Vec<String>,
    var_name: String,
    default_var_value: Option<String>,
}

impl Context {
    pub fn new(prefix: impl AsRef<str>) -> Self {
        let prefix = prefix.as_ref().to_owned();
        let mut ret = Self {
            prefix: vec![],
            var_name: String::default(),
            default_var_value: None,
        };
        if !prefix.is_empty() {
            ret.prefix.push(prefix);
        }
        ret
    }

    pub fn prefix(&self) -> String {
        self.prefix.join("_")
    }

    pub fn with_var_name(&self, var_name: &str) -> Self {
        Self {
            prefix: self.prefix.clone(),
            var_name: var_name.to_owned(),
            default_var_value: None,
        }
    }

    pub fn infer_var_name(&self) -> String {
        match self.var_name.len() {
            0 => self.prefix(),
            _ => self.var_name.to_owned(),
        }
    }

    pub fn push_prefix(&self, val: String) -> Self {
        let mut ret = Self {
            prefix: self.prefix.clone(),
            var_name: self.var_name.clone(),
            default_var_value: None,
        };
        ret.prefix.push(val);
        ret
    }

    pub fn with_default_value(&self, val: &str) -> Self {
        Self {
            prefix: self.prefix.clone(),
            var_name: self.var_name.clone(),
            default_var_value: Some(String::from(val)),
        }
    }

    pub fn get_default_value(&self) -> Option<String> {
        self.default_var_value.as_ref().cloned()
    }
}
